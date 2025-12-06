//! ESI API error response types.
//!
//! This module contains error types for handling error responses from the ESI API,
//! including HTTP error status codes, error messages, and relevant headers.

use std::time::Duration;
use thiserror::Error;

use crate::esi::{CacheHeaders, RateLimitHeaders};

/// Error response from the ESI API.
///
/// This structure represents error responses (4xx or 5xx status codes) returned by ESI,
/// including all relevant headers and error information.
#[derive(Error, Debug, Clone)]
#[error("ESI error (status {status}): {data}")]
pub struct EsiResponseError {
    /// HTTP status code of the error response
    pub status: u16,

    /// Error data from the response body
    pub data: EsiResponseErrorData,

    /// Caching headers from the error response
    pub cache: CacheHeaders,

    /// Rate limiting headers from the error response
    ///
    /// Only present when the `x-esi-error-limit-group` header is included in the response.
    pub rate_limit: Option<RateLimitHeaders>,

    /// Duration in seconds until tokens are replenished enough for another request
    ///
    /// Only present on 429 (Too Many Requests) responses.
    pub retry_after: Option<Duration>,
}

/// Error data from an ESI error response body.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct EsiResponseErrorData {
    /// The error message from ESI
    pub error: String,
}

impl std::fmt::Display for EsiResponseErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}
