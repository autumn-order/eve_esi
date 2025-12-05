//! Response types for ESI requests.
//!
//! This module contains types that represent responses from ESI API calls,
//! particularly for cached requests that may return 304 Not Modified.

use chrono::{DateTime, Utc};
use std::ops::{Deref, DerefMut};
use std::time::Duration;

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
    pub rate_limit: RateLimitHeaders,
}

/// Caching-related HTTP headers from the ESI response.
#[derive(Debug, Clone, Default)]
pub struct CacheHeaders {
    /// Cache-Control directives for caching mechanisms.
    ///
    /// Controls how the response can be cached, by whom, and for how long.
    pub cache_control: Option<String>,

    /// The ETag value of the response body.
    ///
    /// Use this with If-None-Match to check whether the resource has changed.
    pub etag: Option<String>,

    /// The last modified date of the response.
    ///
    /// Use this with If-Modified-Since to check whether the resource has changed.
    pub last_modified: Option<DateTime<Utc>>,
}

/// Rate limiting HTTP headers from the ESI response.
#[derive(Debug, Clone, Default)]
pub struct RateLimitHeaders {
    /// Route group identifier for this endpoint.
    pub group: Option<String>,

    /// Total tokens per window (e.g., "150/15m").
    ///
    /// Format: `<tokens>/<window>` where window uses:
    /// - `m`: minutes
    /// - `h`: hours
    pub limit: Option<String>,

    /// Available tokens remaining in the current window.
    pub remaining: Option<u32>,

    /// Tokens consumed by this request.
    pub used: Option<u32>,

    /// If rate-limited (429), indicates in seconds when to try again.
    pub retry_after: Option<Duration>,
}

impl<T> EsiResponse<T> {
    /// Creates a new EsiResponse with the given data and default headers.
    pub fn new(data: T) -> Self {
        Self {
            data,
            cache: CacheHeaders::default(),
            rate_limit: RateLimitHeaders::default(),
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
