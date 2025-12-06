//! ESI API client implementation.
//!
//! This module contains the core logic for executing ESI requests,
//! including authentication, header management, and response handling.
//!
//!
//! # Example
//! ```no_run
//! use eve_esi::Client;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct ServerStatus {
//!     players: i32,
//! }
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new("MyApp/1.0")?;
//! let request = client.esi().new_request::<ServerStatus>("https://esi.evetech.net/latest/status/");
//! let status = request.send().await?;
//! # Ok(())
//! # }
//! ```

use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use std::time::Duration;

use crate::{Client, Error};

use super::{CacheHeaders, CachedResponse, EsiRequest, EsiResponse, RateLimitHeaders};

/// Provides utility methods for making requests to EVE Online's ESI endpoints.
///
/// This struct is the core executor for ESI requests. It handles:
/// - Token validation for authenticated requests
/// - Building and sending HTTP requests
/// - Processing responses (including 304 Not Modified for cached requests)
/// - Extracting caching headers (ETag, Last-Modified)
pub struct EsiApi<'a> {
    pub(crate) client: &'a Client,
}

impl Client {
    /// Access to utility functions to make ESI requests.
    ///
    /// Returns an [`EsiApi`] instance that can be used to execute ESI requests.
    pub fn esi(&self) -> EsiApi<'_> {
        EsiApi::new(self)
    }
}

impl<'a> EsiApi<'a> {
    /// Creates a new instance of [`EsiApi`].
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Creates a new [`EsiRequest`] for the given endpoint.
    ///
    /// This is the recommended way to create ESI requests as it automatically
    /// ties the request's lifetime to the client.
    ///
    /// # Arguments
    /// - `endpoint`: The ESI API endpoint URL to request
    ///
    /// # Returns
    /// A new [`EsiRequest`] instance ready to be configured with headers, authentication, etc.
    pub fn new_request<T: DeserializeOwned>(&self, endpoint: impl Into<String>) -> EsiRequest<T> {
        EsiRequest::new(self.client, endpoint)
    }

    /// Extracts headers from reqwest::HeaderMap and populates an EsiResponse with data.
    ///
    /// This helper function extracts caching and rate limiting headers from the HTTP response
    /// and wraps the deserialized data in an EsiResponse struct.
    ///
    /// # Arguments
    /// - `headers`: The HTTP headers from the response
    /// - `data`: The deserialized response data
    ///
    /// # Returns
    /// An EsiResponse containing the data and populated headers
    fn populate_esi_response_from_headers<T>(
        headers: &reqwest::header::HeaderMap,
        data: T,
    ) -> EsiResponse<T> {
        // Extract cache headers
        let cache_control = headers
            .get("cache-control")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        let etag = headers
            .get("etag")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        let last_modified = headers
            .get("last-modified")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| DateTime::parse_from_rfc2822(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        // Extract rate limit headers
        let group = headers
            .get("x-esi-error-limit-group")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        let limit = headers
            .get("x-esi-error-limit-limit")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        let remaining = headers
            .get("x-esi-error-limit-remain")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());

        let used = headers
            .get("x-esi-error-limit-used")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());

        let retry_after = headers
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs);

        EsiResponse {
            data,
            cache: CacheHeaders {
                cache_control,
                etag,
                last_modified,
            },
            rate_limit: RateLimitHeaders {
                group,
                limit,
                remaining,
                used,
                retry_after,
            },
        }
    }

    /// Internal method that executes the request with common logic.
    ///
    /// This consolidates all the shared request execution logic:
    /// - Token validation
    /// - Request building with headers, auth, and body
    /// - Error handling and logging
    ///
    /// # Arguments
    /// - `request`: The configured [`EsiRequest`] to execute
    ///
    /// # Returns
    /// A Result containing the raw [`reqwest::Response`] or an error
    async fn execute_request<T: DeserializeOwned>(
        &self,
        request: &EsiRequest<T>,
    ) -> Result<reqwest::Response, Error> {
        let method = request.method().clone();
        let endpoint = request.endpoint().to_string();

        log::debug!("ESI Request: {} {}", method, endpoint);

        let start_time = std::time::Instant::now();

        // Validate token if this is an authenticated request
        if let Some(access_token) = request.access_token() {
            self.validate_token_before_request(access_token, request.required_scopes().clone())
                .await?;
        }

        let reqwest_client = &self.client.inner.reqwest_client;

        // Build the request with the appropriate HTTP method
        let mut req_builder = reqwest_client.request(method.clone(), &endpoint);

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
        let response = req_builder.send().await;

        let elapsed = start_time.elapsed();

        match response {
            Ok(r) => {
                log::debug!(
                    "ESI Request completed: {} {} ({}ms)",
                    method,
                    endpoint,
                    elapsed.as_millis()
                );
                Ok(r)
            }
            Err(err) => {
                log::debug!(
                    "ESI Request failed: {} {} ({}ms) - {}",
                    method,
                    endpoint,
                    elapsed.as_millis(),
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Make a request to ESI using the provided [`EsiRequest`] configuration.
    ///
    /// This method handles ESI requests for both authenticated and public endpoints.
    /// It automatically:
    /// - Validates access tokens if present (expiration & scope checks)
    /// - Adds authentication headers for authenticated requests
    /// - Applies all custom headers from the request
    /// - Handles request body for POST, PUT, and PATCH methods
    /// - Returns deserialized response data wrapped in EsiResponse with headers
    ///
    /// # Arguments
    /// - `request`: The configured [`EsiRequest`] containing endpoint, method, headers, and authentication details
    ///
    /// # Returns
    /// A Result containing an EsiResponse with the deserialized response data and headers
    pub async fn request<T: DeserializeOwned>(
        &self,
        request: &EsiRequest<T>,
    ) -> Result<EsiResponse<T>, Error> {
        let method = request.method().clone();
        let endpoint = request.endpoint().to_string();

        let response = self.execute_request(&request).await?;

        if let Err(err) = response.error_for_status_ref() {
            log::error!("ESI Request failed: {} {} - {}", method, endpoint, err);
            return Err(err.into());
        }

        // Extract headers before consuming the response
        let headers = response.headers().clone();

        // Deserialize and return the response
        let body = response.text().await?;
        let result: T = serde_json::from_str(&body).map_err(|e| {
            log::error!(
                "Failed to deserialize response for {} {}: {}. Body: {}",
                method,
                endpoint,
                e,
                body
            );
            Error::from(e)
        })?;

        log::info!("ESI Request succeeded: {} {}", method, endpoint);

        // Create a temporary response-like struct for header extraction
        Ok(Self::populate_esi_response_from_headers(&headers, result))
    }

    /// Make a cached request to ESI using the provided [`EsiRequest`] configuration.
    ///
    /// This method is similar to [`request`](Self::request) but handles 304 Not Modified responses
    /// when conditional headers are present in the request. It returns a [`CachedResponse`] enum
    /// that distinguishes between fresh data and cached data that hasn't changed.
    ///
    /// **Note:** This method is typically called internally by [`EsiRequest::send_cached`].
    /// Most users should use that method instead for a more convenient API.
    ///
    /// # Arguments
    /// - `request`: The configured [`EsiRequest`] with conditional cache headers already set
    ///
    /// # Returns
    /// - `Ok(CachedResponse::Fresh)`: New data was received wrapped in EsiResponse with all headers
    /// - `Ok(CachedResponse::NotModified)`: Resource hasn't changed since the conditional header date/ETag
    /// - `Err(Error)`: Request failed
    pub async fn request_cached<T: DeserializeOwned>(
        &self,
        request: &EsiRequest<T>,
    ) -> Result<CachedResponse<EsiResponse<T>>, Error> {
        let method = request.method().clone();
        let endpoint = request.endpoint().to_string();

        let response = self.execute_request(&request).await?;

        // Check for 304 Not Modified
        if response.status() == reqwest::StatusCode::NOT_MODIFIED {
            log::info!(
                "ESI Cached Request succeeded (not modified): {} {}",
                method,
                endpoint
            );
            return Ok(CachedResponse::NotModified);
        }

        // Check for other errors
        if let Err(err) = response.error_for_status_ref() {
            log::error!(
                "ESI Cached Request failed: {} {} - {}",
                method,
                endpoint,
                err
            );
            return Err(err.into());
        }

        // Extract headers before consuming the response
        let headers = response.headers().clone();

        // Deserialize and return the response
        let body = response.text().await?;
        let data: T = serde_json::from_str(&body).map_err(|e| {
            log::error!(
                "Failed to deserialize cached response for {} {}: {}. Body: {}",
                method,
                endpoint,
                e,
                body
            );
            Error::from(e)
        })?;

        log::info!(
            "ESI Cached Request succeeded (fresh): {} {}",
            method,
            endpoint
        );

        Ok(CachedResponse::Fresh(
            Self::populate_esi_response_from_headers(&headers, data),
        ))
    }
}
