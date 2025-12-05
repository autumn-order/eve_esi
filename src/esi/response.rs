//! Response types for ESI requests.
//!
//! This module contains types that represent responses from ESI API calls,
//! particularly for cached requests that may return 304 Not Modified.

use chrono::{DateTime, Utc};

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
