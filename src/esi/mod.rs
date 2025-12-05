//! # EVE ESI Request Methods
//!
//! Provides utility methods for making requests to EVE Online's ESI. These
//! methods are used internally by the [`crate::endpoints`] module to make requests.
//!
//! Despite the use case intended primarily to be internal, these functions are exported publicly
//! to allow for using the ESI client to make requests to custom ESI routes. This is useful
//! for when this crate hasn't implemented an ESI route yet but you still wish to use the client
//! to make requests to the route.
//!
//! For usage regarding making ESI requests with the eve_esi crate, see the
//! [endpoints module documentation](crate::endpoints)
//!
//! ## Modules
//! - [`public`]: Methods for making public requests to ESI endpoints
//! - [`authenticated`]: Methods for making authenticated requests to ESI endpoints using an access token
//!
//! ## Usage
//!
//! ```no_run
//! use serde::{Serialize, Deserialize};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Setup a basic Client with a user agent to identify requests
//!     let user_agent = "MyApp/1.0 (contact@example.com; +https://github.com/your/repository)";
//!     let esi_client = eve_esi::Client::new(user_agent).expect("Failed to build ESI Client");
//!
//!     // Define the struct to deserialize the ESI response to
//!     #[derive(Serialize, Deserialize)]
//!     pub struct CharacterAffiliations {
//!         pub alliance_id: Option<i64>,
//!         pub character_id: i64,
//!         pub corporation_id: i64,
//!         pub faction_id: Option<i64>,
//!     };
//!
//!     // Define the URL to make the request to
//!     let esi_endpoint_url = "https://esi.evetech.net/characters/affiliation/";
//!
//!     // Make the request with the earlier defined struct
//!     // - The first type, `<Vec<CharacterAffiliations>`, represents the response body to deserialize
//!     // - The second type, `Vec<i64>`, represents the request body to serialize (not applicable to GET requests)
//!     let character_ids = vec![2114794365];
//!
//!     let character_affiliations = esi_client
//!         .esi()
//!         .post_to_public_esi::<Vec<CharacterAffiliations>, Vec<i64>>(&esi_endpoint_url, &character_ids)
//!         .await;
//! }
//! ```

pub mod authenticated;
pub mod public;

mod util;

use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;

use crate::{model::esi::EsiRequest, Client, Error};

/// Response from a cached ESI request.
///
/// Represents the result of a request that may return 304 Not Modified
/// when conditional headers like `If-None-Match` or `If-Modified-Since` are used.
#[derive(Debug, Clone)]
pub enum CachedResponse<T> {
    /// Fresh data was returned (200 OK response)
    ///
    /// Contains the deserialized response body and optional caching headers for future requests
    Fresh {
        /// The deserialized response data
        data: T,
        /// ETag from the response headers, if present
        etag: Option<String>,
        /// Last-Modified timestamp from the response headers, if present
        last_modified: Option<DateTime<Utc>>,
    },

    /// Resource has not been modified (304 Not Modified response)
    ///
    /// The server indicates the cached version is still valid
    NotModified,
}

impl<T> CachedResponse<T> {
    /// Returns `true` if the response is fresh data.
    pub fn is_fresh(&self) -> bool {
        matches!(self, CachedResponse::Fresh { .. })
    }

    /// Returns `true` if the response is not modified.
    pub fn is_not_modified(&self) -> bool {
        matches!(self, CachedResponse::NotModified)
    }

    /// Consumes the response and returns the data if fresh, or None if not modified.
    pub fn into_data(self) -> Option<T> {
        match self {
            CachedResponse::Fresh { data, .. } => Some(data),
            CachedResponse::NotModified => None,
        }
    }

    /// Consumes the response and returns a tuple of (data, etag, last_modified) if fresh.
    ///
    /// This is useful when you want to extract all caching metadata along with the data.
    ///
    /// # Returns
    /// - `Some((T, Option<String>, Option<DateTime<Utc>>))`: The data with caching headers if fresh
    /// - `None`: If the response was not modified
    pub fn into_parts(self) -> Option<(T, Option<String>, Option<DateTime<Utc>>)> {
        match self {
            CachedResponse::Fresh {
                data,
                etag,
                last_modified,
            } => Some((data, etag, last_modified)),
            CachedResponse::NotModified => None,
        }
    }

    /// Returns a reference to the data if fresh, or None if not modified.
    pub fn data(&self) -> Option<&T> {
        match self {
            CachedResponse::Fresh { data, .. } => Some(data),
            CachedResponse::NotModified => None,
        }
    }

    /// Returns the ETag if the response is fresh and contains one.
    pub fn etag(&self) -> Option<&str> {
        match self {
            CachedResponse::Fresh {
                etag: Some(etag), ..
            } => Some(etag.as_str()),
            _ => None,
        }
    }

    /// Returns the Last-Modified timestamp if the response is fresh and contains one.
    pub fn last_modified(&self) -> Option<DateTime<Utc>> {
        match self {
            CachedResponse::Fresh {
                last_modified: Some(timestamp),
                ..
            } => Some(*timestamp),
            _ => None,
        }
    }
}

/// Provides utility methods for making requests EVE Online's ESI endpoints
///
/// See the [module-level documentation](super) for an overview, methods, & usage example.
pub struct EsiApi<'a> {
    pub(crate) client: &'a Client,
}

impl Client {
    /// Access to utility functions to make ESI requests
    ///
    /// See the [module-level documentation](super) for an overview, methods, & usage example.
    pub fn esi(&self) -> self::EsiApi<'_> {
        self::EsiApi::new(self)
    }
}

impl<'a> EsiApi<'a> {
    /// Creates a new instance of [`EsiApi`]
    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Make a request to ESI using the provided [`EsiRequest`] configuration.
    ///
    /// This method consolidates all ESI request logic, handling both authenticated and public requests
    /// based on the configuration in the [`EsiRequest`] struct. It automatically:
    /// - Validates access tokens if present (expiration & scope checks)
    /// - Adds authentication headers for authenticated requests
    /// - Applies all custom headers from the request
    /// - Handles request body for POST, PUT, and PATCH methods
    /// - Returns deserialized response data
    ///
    /// # Arguments
    /// - `request`: The configured [`EsiRequest`] containing endpoint, method, headers, and authentication details
    ///
    /// # Returns
    /// A Result containing the deserialized response data or an error
    pub async fn request<T: DeserializeOwned>(&self, request: EsiRequest<T>) -> Result<T, Error> {
        // Validate token if this is an authenticated request
        if let Some(access_token) = request.access_token() {
            self.validate_token_before_request(access_token, request.required_scopes().clone())
                .await?;
        }

        let reqwest_client = &self.client.inner.reqwest_client;

        // Build the request with the appropriate HTTP method
        let mut req_builder = reqwest_client.request(request.method().clone(), request.endpoint());

        // Add authorization header if access token is present
        if let Some(access_token) = request.access_token() {
            let bearer = format!("Bearer {}", access_token);
            req_builder = req_builder.header("Authorization", bearer);
        }

        // Add all custom headers from the request
        for (key, value) in request.headers() {
            req_builder = req_builder.header(key, value);
        }

        // Add JSON body if present (for POST, PUT, PATCH requests)
        if let Some(body) = request.body_json() {
            req_builder = req_builder.json(body);
        }

        // Send the request
        let response = req_builder.send().await?;
        response.error_for_status_ref()?;

        // Deserialize and return the response
        let result: T = response.json().await?;
        Ok(result)
    }

    /// Make a cached request to ESI using the provided [`EsiRequest`] configuration.
    ///
    /// This method is similar to [`request`](Self::request) but handles 304 Not Modified responses
    /// when conditional headers are present in the request. It returns a [`CachedResponse`] enum
    /// that distinguishes between fresh data and cached data that hasn't changed.
    ///
    /// # Arguments
    /// - `request`: The configured [`EsiRequest`] with conditional cache headers already set
    ///
    /// # Returns
    /// - `Ok(CachedResponse::Fresh)`: New data was received with optional ETag
    /// - `Ok(CachedResponse::NotModified)`: Resource hasn't changed since the conditional header date/ETag
    /// - `Err(Error)`: Request failed
    pub async fn request_cached<T: DeserializeOwned>(
        &self,
        request: EsiRequest<T>,
    ) -> Result<CachedResponse<T>, Error> {
        // Validate token if this is an authenticated request
        if let Some(access_token) = request.access_token() {
            self.validate_token_before_request(access_token, request.required_scopes().clone())
                .await?;
        }

        let reqwest_client = &self.client.inner.reqwest_client;

        // Build the request with the appropriate HTTP method
        let mut req_builder = reqwest_client.request(request.method().clone(), request.endpoint());

        // Add authorization header if access token is present
        if let Some(access_token) = request.access_token() {
            let bearer = format!("Bearer {}", access_token);
            req_builder = req_builder.header("Authorization", bearer);
        }

        // Add all custom headers from the request
        for (key, value) in request.headers() {
            req_builder = req_builder.header(key, value);
        }

        // Add JSON body if present (for POST, PUT, PATCH requests)
        if let Some(body) = request.body_json() {
            req_builder = req_builder.json(body);
        }

        // Send the request
        let response = req_builder.send().await?;

        // Check for 304 Not Modified
        if response.status() == reqwest::StatusCode::NOT_MODIFIED {
            return Ok(CachedResponse::NotModified);
        }

        // Check for other errors
        response.error_for_status_ref()?;

        // Extract ETag header if present
        let etag = response
            .headers()
            .get("etag")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        // Extract Last-Modified header if present and parse it
        let last_modified = response
            .headers()
            .get("last-modified")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| DateTime::parse_from_rfc2822(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        // Deserialize and return the response
        let result: T = response.json().await?;
        Ok(CachedResponse::Fresh {
            data: result,
            etag,
            last_modified,
        })
    }
}
