//! Response types for ESI requests.
//!
//! This module contains types that represent responses from ESI API calls,
//! particularly for cached requests that may return 304 Not Modified.

use chrono::{DateTime, Utc};
use std::ops::{Deref, DerefMut};

/// Response from an ESI request including response data & headers
///
/// Contains the deserialized response data along with all relevant HTTP headers
/// including caching directives and rate limiting information.
#[derive(Debug, Clone)]
pub struct EsiResponse<T> {
    /// The deserialized response data
    pub data: T,

    /// Caching headers
    pub cache: CacheHeaders,

    /// Rate limiting headers
    ///
    /// Only present when the `x-esi-error-limit-group` header is included in the response.
    pub rate_limit: Option<RateLimitHeaders>,
}

/// Caching-related HTTP headers from the ESI response.
///
/// All fields are always present on successful (200) responses.
#[derive(Debug, Clone)]
pub struct CacheHeaders {
    /// Cache-Control directives for caching mechanisms.
    ///
    /// Controls how the response can be cached, by whom, and for how long.
    pub cache_control: String,

    /// The ETag value of the response body.
    ///
    /// Use this with If-None-Match to check whether the resource has changed.
    pub etag: String,

    /// The last modified date of the response.
    ///
    /// Use this with If-Modified-Since to check whether the resource has changed.
    pub last_modified: DateTime<Utc>,
}

/// Rate limiting HTTP headers from the ESI response.
///
/// These headers are only present when `x-esi-error-limit-group` is included in the response.
#[derive(Debug, Clone)]
pub struct RateLimitHeaders {
    /// Route group identifier for this endpoint.
    pub group: String,

    /// Total tokens per window (e.g., "150/15m").
    ///
    /// Format: `<tokens>/<window>` where window uses:
    /// - `m`: minutes
    /// - `h`: hours
    pub limit: String,

    /// Available tokens remaining in the current window.
    pub remaining: u32,

    /// Tokens consumed by this request.
    pub used: u32,
}

impl<T> EsiResponse<T> {
    /// Creates a new EsiResponse with the given data and placeholder cache headers.
    ///
    /// Note: This is primarily for testing. Real responses should use actual cache headers.
    pub fn new(data: T) -> Self {
        Self {
            data,
            cache: CacheHeaders {
                cache_control: String::new(),
                etag: String::new(),
                last_modified: chrono::Utc::now(),
            },
            rate_limit: None,
        }
    }
}

impl<T> Deref for EsiResponse<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for EsiResponse<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

/// Response from a cached ESI request.
///
/// Represents the result of a request that may return 304 Not Modified
/// when conditional headers like `If-None-Match` or `If-Modified-Since` are used.
#[derive(Debug, Clone)]
pub enum CachedResponse<T> {
    /// Fresh data was returned (200 OK response)
    ///
    /// Contains the EsiResponse with deserialized data and all headers
    Fresh(T),

    /// Resource has not been modified (304 Not Modified response)
    ///
    /// The server indicates the cached version is still valid
    NotModified,
}

impl<T> CachedResponse<T> {
    /// Returns `true` if the response is fresh data.
    pub fn is_fresh(&self) -> bool {
        matches!(self, CachedResponse::Fresh(_))
    }

    /// Returns `true` if the response is not modified.
    pub fn is_not_modified(&self) -> bool {
        matches!(self, CachedResponse::NotModified)
    }
}
