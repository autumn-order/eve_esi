//! Request builder types for ESI API calls.
//!
//! This module provides types for building and configuring ESI requests with
//! type-safe headers, authentication, and caching strategies.
//!
//! # Example
//! ```no_run
//! use eve_esi::{Client, CacheStrategy};
//! use chrono::{DateTime, Utc};
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct ServerStatus {
//!     players: i32,
//! }
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new("MyApp/1.0")?;
//!
//! // Simple request - recommended approach
//! let request = client.esi().new_request::<ServerStatus>("https://esi.evetech.net/latest/status/");
//! let status = request.send().await?;
//!
//! // Cached request
//! let last_check: DateTime<Utc> = Utc::now();
//! let request = client.esi().new_request::<ServerStatus>("https://esi.evetech.net/latest/status/");
//! let response = request
//!     .send_cached(CacheStrategy::IfModifiedSince(last_check))
//!     .await?;
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{Client, Error};

use super::CachedResponse;

/// Strategy for conditional caching requests to ESI.
///
/// Used with [`EsiRequest::send_cached`] to specify which HTTP conditional
/// headers to send for cache validation.
///
/// # Date Formatting
///
/// For `IfModifiedSince` and `Both` variants, the `DateTime<Utc>` is automatically
/// converted to HTTP date format (RFC 2822) when sent to the server.
#[derive(Debug, Clone)]
pub enum CacheStrategy {
    /// Use `If-None-Match` header with an ETag value.
    ///
    /// The server returns 304 Not Modified if the ETag matches the current resource.
    IfNoneMatch(String),

    /// Use `If-Modified-Since` header with a timestamp.
    ///
    /// The server returns 304 Not Modified if the resource hasn't been modified since the date.
    /// The datetime is automatically formatted to HTTP date format (RFC 2822).
    IfModifiedSince(DateTime<Utc>),

    /// Use both `If-None-Match` and `If-Modified-Since` headers.
    ///
    /// When both are present, `If-None-Match` takes precedence but the server should respect both.
    /// This provides defensive caching. The datetime is automatically formatted to HTTP date format.
    Both {
        /// ETag value for If-None-Match header
        etag: String,
        /// Timestamp for If-Modified-Since header (automatically formatted to HTTP date)
        modified_since: DateTime<Utc>,
    },
}

/// Builder for ESI API requests with configurable headers and authentication.
///
/// Provides a fluent interface for setting endpoint URLs, authentication tokens,
/// and ESI-specific HTTP headers like compatibility date, language, and caching headers.
#[derive(Clone)]
pub struct EsiRequest<'a, T> {
    /// Reference to the ESI client
    client: &'a Client,
    /// The endpoint to request e.g. "https://esi.evetech.net/latest/status/"
    endpoint: String,
    /// HTTP method for the request (GET, POST, PUT, DELETE, PATCH)
    method: Method,
    /// Access token used to access authenticated endpoints
    access_token: Option<String>,
    /// Required OAuth2 scopes for authenticated requests
    required_scopes: Vec<String>,
    /// Optional JSON body data for POST, PUT, PATCH requests
    body_json: Option<Value>,
    /// Headers to send with ESI request
    headers: HashMap<String, String>,
    /// Phantom data to hold the response type
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: DeserializeOwned> EsiRequest<'a, T> {
    /// Creates a new [`EsiRequest`] with the specified client and endpoint.
    ///
    /// **Note:** It's recommended to use [`crate::esi::EsiApi::new_request`] instead:
    /// ```ignore
    /// let request = client.esi().new_request::<ResponseType>("endpoint_url");
    /// ```
    ///
    /// # Arguments
    /// - `client`: The [`Client`] to use for sending the request
    /// - `endpoint`: The ESI API endpoint URL to request
    ///
    /// # Returns
    /// New instance with the client and endpoint set and all other fields at default values
    pub fn new(client: &'a Client, endpoint: impl Into<String>) -> Self {
        Self {
            client,
            endpoint: endpoint.into(),
            method: Method::GET,
            access_token: None,
            required_scopes: Vec::new(),
            body_json: None,
            headers: HashMap::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Sets the HTTP method for the request.
    ///
    /// # Arguments
    /// - `method`: The HTTP method to use
    ///
    /// # Returns
    /// Updated instance with the HTTP method set
    pub fn with_method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    /// Sets the access token for authenticated ESI requests.
    ///
    /// # Arguments
    /// - `token`: The OAuth2 access token
    ///
    /// # Returns
    /// Updated instance with the access token set
    pub fn with_access_token(mut self, token: impl Into<String>) -> Self {
        self.access_token = Some(token.into());
        self
    }

    /// Sets the `X-Compatibility-Date` header (required by ESI).
    ///
    /// This header ensures API compatibility for breaking changes.
    ///
    /// # Arguments
    /// - `date`: The compatibility date in YYYY-MM-DD format (e.g., "2025-11-06")
    ///
    /// # Returns
    /// Updated instance with the compatibility date header set
    pub fn with_compatibility_date(mut self, date: impl Into<String>) -> Self {
        self.headers
            .insert("X-Compatibility-Date".to_string(), date.into());
        self
    }

    /// Sets the `X-Tenant` header for specifying the EVE server.
    ///
    /// Defaults to `tranquility`
    ///
    /// # Arguments
    /// - `tenant`: The tenant ID (e.g., "tranquility")
    ///
    /// # Returns
    /// Updated instance with the tenant header set
    pub fn with_tenant(mut self, tenant: impl Into<String>) -> Self {
        self.headers.insert("X-Tenant".to_string(), tenant.into());
        self
    }

    /// Sets the `Accept-Language` header for localized responses.
    ///
    /// # Arguments
    /// - `lang`: The language for the response
    ///
    /// # Returns
    /// Updated instance with the language header set
    pub fn with_language(mut self, lang: Language) -> Self {
        self.headers
            .insert("Accept-Language".to_string(), lang.as_str().to_string());
        self
    }

    /// Sets the `If-Match` header for conditional requests.
    ///
    /// Only performs the request if the ETag matches. This is typically used
    /// for conditional updates (PUT/POST) to prevent lost updates.
    ///
    /// # Arguments
    /// - `etag`: The ETag that must match
    ///
    /// # Returns
    /// Updated instance with the If-Match header set
    pub fn with_if_match(mut self, etag: impl Into<String>) -> Self {
        self.headers.insert("If-Match".to_string(), etag.into());
        self
    }

    /// Sets a custom header for the request.
    ///
    /// Use this for headers not covered by the other methods.
    ///
    /// # Arguments
    /// - `key`: The header name
    /// - `value`: The header value
    ///
    /// # Returns
    /// Updated instance with the custom header set
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Returns the endpoint URL.
    ///
    /// # Returns
    /// Reference to the endpoint URL string
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Returns the access token if set.
    ///
    /// # Returns
    /// `Some(&str)`: Reference to the access token string if present
    /// `None`: No access token is set
    pub fn access_token(&self) -> Option<&str> {
        self.access_token.as_deref()
    }

    /// Sets the required OAuth2 scopes for authenticated requests.
    ///
    /// # Arguments
    /// - `scopes`: Vector of scope strings required for the endpoint
    ///
    /// # Returns
    /// Updated instance with the required scopes set
    pub fn with_required_scopes(mut self, scopes: Vec<String>) -> Self {
        self.required_scopes = scopes;
        self
    }

    /// Returns the required OAuth2 scopes.
    ///
    /// # Returns
    /// Reference to the vector of required scope strings
    pub fn required_scopes(&self) -> &Vec<String> {
        &self.required_scopes
    }

    /// Sets the JSON body for POST, PUT, or PATCH requests.
    ///
    /// # Arguments
    /// - `body`: The JSON value to send in the request body
    ///
    /// # Returns
    /// Updated instance with the body JSON set
    pub fn with_body_json(mut self, body: Value) -> Self {
        self.body_json = Some(body);
        self
    }

    /// Returns the JSON body if set.
    ///
    /// # Returns
    /// `Some(&Value)`: Reference to the JSON value if present
    /// `None`: No body is set
    pub fn body_json(&self) -> Option<&Value> {
        self.body_json.as_ref()
    }

    /// Returns a reference to all headers.
    ///
    /// # Returns
    /// Reference to the headers map
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Returns the HTTP method.
    ///
    /// # Returns
    /// The HTTP method for this request
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Consumes the [`EsiRequest`] and sends it using the stored [`Client`].
    ///
    /// This is a convenience method that allows for a fluent API where you build the request
    /// and then send it in a single chain. It delegates to the [`crate::esi::EsiApi::request`] method.
    ///
    /// For cached requests that handle 304 Not Modified responses, use [`send_cached`](Self::send_cached) instead.
    ///
    /// # Returns
    /// A Result containing the deserialized response data or an error
    pub async fn send(self) -> Result<T, Error> {
        self.client.esi().request(&self).await
    }

    /// Consumes the [`EsiRequest`] and sends it with caching headers using the stored [`Client`].
    ///
    /// This method handles conditional requests that may return 304 Not Modified responses.
    /// Use the [`CacheStrategy`] parameter to specify which conditional headers to send.
    ///
    /// # Arguments
    /// - `strategy`: The caching strategy specifying which conditional headers to use
    ///
    /// # Returns
    /// A Result containing a [`CachedResponse`] that may be either fresh data or not modified
    pub async fn send_cached(self, strategy: CacheStrategy) -> Result<CachedResponse<T>, Error> {
        let mut request = self;

        // Add the appropriate conditional headers based on strategy
        match strategy {
            CacheStrategy::IfNoneMatch(etag) => {
                request.headers.insert("If-None-Match".to_string(), etag);
            }
            CacheStrategy::IfModifiedSince(date) => {
                // Format DateTime to HTTP date format (RFC 2822)
                let http_date = date.to_rfc2822();
                request
                    .headers
                    .insert("If-Modified-Since".to_string(), http_date);
            }
            CacheStrategy::Both {
                etag,
                modified_since,
            } => {
                request.headers.insert("If-None-Match".to_string(), etag);
                // Format DateTime to HTTP date format (RFC 2822)
                let http_date = modified_since.to_rfc2822();
                request
                    .headers
                    .insert("If-Modified-Since".to_string(), http_date);
            }
        }

        request.client.esi().request_cached(&request).await
    }
}

/// Type-safe enum for ESI language headers.
///
/// Represents the supported languages for the `Accept-Language` header in ESI requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// English (en)
    English,
    /// German (de)
    German,
    /// French (fr)
    French,
    /// Japanese (ja)
    Japanese,
    /// Russian (ru)
    Russian,
    /// Chinese (zh)
    Chinese,
    /// Korean (ko)
    Korean,
    /// Spanish (es)
    Spanish,
}

impl Language {
    /// Returns the ISO 639-1 language code string.
    ///
    /// # Returns
    /// The two-letter language code used in ESI requests
    ///
    /// # Example
    /// ```rust
    /// use eve_esi::Language;
    ///
    /// assert_eq!(Language::English.as_str(), "en");
    /// assert_eq!(Language::German.as_str(), "de");
    /// ```
    pub fn as_str(&self) -> &str {
        match self {
            Self::English => "en",
            Self::German => "de",
            Self::French => "fr",
            Self::Japanese => "ja",
            Self::Russian => "ru",
            Self::Chinese => "zh",
            Self::Korean => "ko",
            Self::Spanish => "es",
        }
    }
}
