//! # ESI Request Models
//!
//! This module provides types for building ESI requests with configurable headers and authentication.
//!
//! ## Types
//! - [`EsiRequest`]: Builder for ESI API requests with optional headers and authentication
//! - [`EsiLanguage`]: Type-safe enum for ESI language headers
//!
//! ## Example
//! ```no_run
//! use eve_esi::{EsiRequest, Client};
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct ServerStatus {
//!     players: i32,
//!     server_version: String,
//!     start_time: String,
//! }
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let user_agent = "MyApp/1.0 (contact@example.com; +https://github.com/your/repository)";
//! let client = Client::new(user_agent)?;
//!
//! let status: ServerStatus = EsiRequest::new("https://esi.evetech.net/latest/status/")
//!     .send(&client)
//!     .await?;
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;

use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{Client, Error};

/// Builder for ESI API requests with configurable headers and authentication.
///
/// Provides a fluent interface for setting endpoint URLs, authentication tokens,
/// and ESI-specific HTTP headers like compatibility date, language, and caching headers.
///
/// For a full overview and usage examples, see the [module-level documentation](self).
pub struct EsiRequest {
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
}

impl EsiRequest {
    /// Creates a new [`EsiRequest`] with the specified endpoint.
    ///
    /// For a full overview and usage examples, see the [module-level documentation](self).
    ///
    /// # Arguments
    /// - `endpoint`: The ESI API endpoint URL to request
    ///
    /// # Returns
    /// New instance with the endpoint set and all other fields at default values
    ///
    /// # Example
    /// ```rust
    /// use eve_esi::EsiRequest;
    ///
    /// let request = EsiRequest::new("https://esi.evetech.net/latest/status/");
    /// ```
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            method: Method::GET,
            access_token: None,
            required_scopes: Vec::new(),
            body_json: None,
            headers: HashMap::new(),
        }
    }

    /// Sets the HTTP method for the request.
    ///
    /// # Arguments
    /// - `method`: The HTTP method to use
    ///
    /// # Returns
    /// Updated instance with the HTTP method set
    ///
    /// # Example
    /// ```rust
    /// use eve_esi::EsiRequest;
    /// use reqwest::Method;
    ///
    /// let request = EsiRequest::new("https://esi.evetech.net/latest/status/")
    ///     .with_method(Method::POST);
    /// ```
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
    pub fn with_language(mut self, lang: EsiLanguage) -> Self {
        self.headers
            .insert("Accept-Language".to_string(), lang.as_str().to_string());
        self
    }

    /// Sets the `If-None-Match` header for conditional requests.
    ///
    /// Returns a 304 Not Modified response if the ETag matches.
    ///
    /// # Arguments
    /// - `etag`: The ETag from a previous request
    ///
    /// # Returns
    /// Updated instance with the If-None-Match header set
    pub fn with_if_none_match(mut self, etag: impl Into<String>) -> Self {
        self.headers
            .insert("If-None-Match".to_string(), etag.into());
        self
    }

    /// Sets the `If-Modified-Since` header for conditional requests.
    ///
    /// Returns a 304 Not Modified response if the resource hasn't changed since the specified date.
    ///
    /// # Arguments
    /// - `date`: The date in HTTP-date format
    ///
    /// # Returns
    /// Updated instance with the If-Modified-Since header set
    pub fn with_if_modified_since(mut self, date: impl Into<String>) -> Self {
        self.headers
            .insert("If-Modified-Since".to_string(), date.into());
        self
    }

    /// Sets the `If-Match` header for conditional requests.
    ///
    /// Only performs the request if the ETag matches.
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
    ///
    /// # Example
    /// ```rust
    /// use eve_esi::EsiRequest;
    ///
    /// let request = EsiRequest::new("https://esi.evetech.net/latest/characters/12345/")
    ///     .with_access_token("token")
    ///     .with_required_scopes(vec!["publicData".to_string()]);
    /// ```
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
    ///
    /// # Example
    /// ```rust
    /// use eve_esi::EsiRequest;
    /// use serde_json::json;
    /// use reqwest::Method;
    ///
    /// let request = EsiRequest::new("https://esi.evetech.net/latest/characters/affiliation/")
    ///     .with_method(Method::POST)
    ///     .with_body_json(json!([2114794365]));
    /// ```
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

    /// Consumes the [`EsiRequest`] and sends it using the provided [`Client`].
    ///
    /// This is a convenience method that allows for a fluent API where you build the request
    /// and then send it in a single chain. It delegates to the [`crate::esi::EsiApi::request`] method.
    ///
    /// # Arguments
    /// - `client`: Reference to the [`Client`] to use for sending the request
    ///
    /// # Returns
    /// A Result containing the deserialized response data or an error
    ///
    /// # Type Parameters
    /// - `T` - The expected return type that implements `DeserializeOwned`
    pub async fn send<T: DeserializeOwned>(self, client: &Client) -> Result<T, Error> {
        client.esi().request(self).await
    }
}

/// Type-safe enum for ESI language headers.
///
/// Represents the supported languages for the `Accept-Language` header in ESI requests.
///
/// For a full overview and usage examples, see the [module-level documentation](self).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EsiLanguage {
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

impl EsiLanguage {
    /// Returns the ISO 639-1 language code string.
    ///
    /// # Returns
    /// The two-letter language code used in ESI requests
    ///
    /// # Example
    /// ```rust
    /// use eve_esi::EsiLanguage;
    ///
    /// assert_eq!(EsiLanguage::English.as_str(), "en");
    /// assert_eq!(EsiLanguage::German.as_str(), "de");
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
