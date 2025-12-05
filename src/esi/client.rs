//! ESI API client implementation.
//!
//! This module contains the core logic for executing ESI requests,
//! including authentication, header management, and response handling.

use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;

use crate::{Client, Error};

use super::{CachedResponse, EsiRequest};

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

        let response = match response {
            Ok(r) => r,
            Err(err) => {
                log::error!(
                    "ESI Request failed: {} {} ({}ms) - {}",
                    method,
                    endpoint,
                    elapsed.as_millis(),
                    err
                );
                return Err(err.into());
            }
        };

        if let Err(err) = response.error_for_status_ref() {
            log::error!(
                "ESI Request failed: {} {} ({}ms) - HTTP {}",
                method,
                endpoint,
                elapsed.as_millis(),
                err
            );
            return Err(err.into());
        }

        // Deserialize and return the response
        let result: T = response.json().await?;

        log::info!(
            "ESI Request succeeded: {} {} ({}ms)",
            method,
            endpoint,
            elapsed.as_millis()
        );

        Ok(result)
    }

    /// Make a cached request to ESI using the provided [`EsiRequest`] configuration.
    ///
    /// This method is similar to [`request`](Self::request) but handles 304 Not Modified responses
    /// when conditional headers are present in the request. It returns a [`CachedResponse`] enum
    /// that distinguishes between fresh data and cached data that hasn't changed.
    ///
    /// **Note:** This method is typically called internally by [`EsiRequest::send_with_cache`].
    /// Most users should use that method instead for a more convenient API.
    ///
    /// # Arguments
    /// - `request`: The configured [`EsiRequest`] with conditional cache headers already set
    ///
    /// # Returns
    /// - `Ok(CachedResponse::Fresh)`: New data was received with optional ETag and Last-Modified
    /// - `Ok(CachedResponse::NotModified)`: Resource hasn't changed since the conditional header date/ETag
    /// - `Err(Error)`: Request failed
    pub async fn request_cached<T: DeserializeOwned>(
        &self,
        request: EsiRequest<T>,
    ) -> Result<CachedResponse<T>, Error> {
        let method = request.method().clone();
        let endpoint = request.endpoint().to_string();

        log::debug!("ESI Cached Request: {} {}", method, endpoint);

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

        let response = match response {
            Ok(r) => r,
            Err(err) => {
                log::error!(
                    "ESI Cached Request failed: {} {} ({}ms) - {}",
                    method,
                    endpoint,
                    elapsed.as_millis(),
                    err
                );
                return Err(err.into());
            }
        };

        // Check for 304 Not Modified
        if response.status() == reqwest::StatusCode::NOT_MODIFIED {
            log::info!(
                "ESI Cached Request succeeded (not modified): {} {} ({}ms)",
                method,
                endpoint,
                elapsed.as_millis()
            );
            return Ok(CachedResponse::NotModified);
        }

        // Check for other errors
        if let Err(err) = response.error_for_status_ref() {
            log::error!(
                "ESI Cached Request failed: {} {} ({}ms) - HTTP {}",
                method,
                endpoint,
                elapsed.as_millis(),
                err
            );
            return Err(err.into());
        }

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

        log::info!(
            "ESI Cached Request succeeded (fresh): {} {} ({}ms)",
            method,
            endpoint,
            elapsed.as_millis()
        );

        Ok(CachedResponse::Fresh {
            data: result,
            etag,
            last_modified,
        })
    }
}
