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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_esi_response_new() {
        let data = vec![1, 2, 3];
        let response = EsiResponse::new(data.clone());

        assert_eq!(response.data, data);
        assert_eq!(response.cache.cache_control, String::new());
        assert_eq!(response.cache.etag, String::new());
        assert!(response.rate_limit.is_none());
    }

    #[test]
    fn test_esi_response_deref() {
        let data = vec![1, 2, 3];
        let response = EsiResponse::new(data.clone());

        // Test Deref trait
        assert_eq!(*response, data);
        assert_eq!(response.len(), 3);
        assert_eq!(response[0], 1);
    }

    #[test]
    fn test_esi_response_deref_mut() {
        let data = vec![1, 2, 3];
        let mut response = EsiResponse::new(data);

        // Test DerefMut trait
        response.push(4);
        assert_eq!(response.data, vec![1, 2, 3, 4]);
        assert_eq!(response.len(), 4);
    }

    #[test]
    fn test_esi_response_with_cache_headers() {
        let data = "test data";
        let response = EsiResponse {
            data,
            cache: CacheHeaders {
                cache_control: "public, max-age=300".to_string(),
                etag: "\"abc123\"".to_string(),
                last_modified: DateTime::parse_from_rfc2822("Wed, 21 Oct 2015 07:28:00 GMT")
                    .unwrap()
                    .with_timezone(&Utc),
            },
            rate_limit: None,
        };

        assert_eq!(response.data, "test data");
        assert_eq!(response.cache.cache_control, "public, max-age=300");
        assert_eq!(response.cache.etag, "\"abc123\"");
    }

    #[test]
    fn test_esi_response_with_rate_limit_headers() {
        let data = 42;
        let response = EsiResponse {
            data,
            cache: CacheHeaders {
                cache_control: "public, max-age=60".to_string(),
                etag: "\"xyz789\"".to_string(),
                last_modified: Utc::now(),
            },
            rate_limit: Some(RateLimitHeaders {
                group: "global".to_string(),
                limit: "150/15m".to_string(),
                remaining: 100,
                used: 50,
            }),
        };

        assert_eq!(response.data, 42);
        assert!(response.rate_limit.is_some());

        let rate_limit = response.rate_limit.unwrap();
        assert_eq!(rate_limit.group, "global");
        assert_eq!(rate_limit.limit, "150/15m");
        assert_eq!(rate_limit.remaining, 100);
        assert_eq!(rate_limit.used, 50);
    }

    #[test]
    fn test_cached_response_fresh() {
        let response = EsiResponse::new(vec![1, 2, 3]);
        let cached = CachedResponse::Fresh(response);

        assert!(cached.is_fresh());
        assert!(!cached.is_not_modified());
    }

    #[test]
    fn test_cached_response_not_modified() {
        let cached: CachedResponse<EsiResponse<Vec<i32>>> = CachedResponse::NotModified;

        assert!(!cached.is_fresh());
        assert!(cached.is_not_modified());
    }

    #[test]
    fn test_cached_response_pattern_matching() {
        let response = EsiResponse::new("test");
        let cached = CachedResponse::Fresh(response);

        match cached {
            CachedResponse::Fresh(data) => {
                assert_eq!(data.data, "test");
            }
            CachedResponse::NotModified => {
                panic!("Expected Fresh variant");
            }
        }
    }

    #[test]
    fn test_cached_response_not_modified_pattern_matching() {
        let cached: CachedResponse<EsiResponse<String>> = CachedResponse::NotModified;

        match cached {
            CachedResponse::Fresh(_) => {
                panic!("Expected NotModified variant");
            }
            CachedResponse::NotModified => {
                // Success
            }
        }
    }

    #[test]
    fn test_cache_headers_clone() {
        let headers = CacheHeaders {
            cache_control: "max-age=300".to_string(),
            etag: "\"tag123\"".to_string(),
            last_modified: Utc::now(),
        };

        let cloned = headers.clone();
        assert_eq!(headers.cache_control, cloned.cache_control);
        assert_eq!(headers.etag, cloned.etag);
        assert_eq!(headers.last_modified, cloned.last_modified);
    }

    #[test]
    fn test_rate_limit_headers_clone() {
        let headers = RateLimitHeaders {
            group: "test_group".to_string(),
            limit: "100/1h".to_string(),
            remaining: 75,
            used: 25,
        };

        let cloned = headers.clone();
        assert_eq!(headers.group, cloned.group);
        assert_eq!(headers.limit, cloned.limit);
        assert_eq!(headers.remaining, cloned.remaining);
        assert_eq!(headers.used, cloned.used);
    }

    #[test]
    fn test_esi_response_clone() {
        let response = EsiResponse {
            data: vec![1, 2, 3],
            cache: CacheHeaders {
                cache_control: "public".to_string(),
                etag: "\"etag\"".to_string(),
                last_modified: Utc::now(),
            },
            rate_limit: Some(RateLimitHeaders {
                group: "group1".to_string(),
                limit: "150/15m".to_string(),
                remaining: 50,
                used: 100,
            }),
        };

        let cloned = response.clone();
        assert_eq!(response.data, cloned.data);
        assert_eq!(response.cache.cache_control, cloned.cache.cache_control);
        assert_eq!(response.cache.etag, cloned.cache.etag);
        assert_eq!(
            response.rate_limit.as_ref().unwrap().group,
            cloned.rate_limit.as_ref().unwrap().group
        );
    }

    #[test]
    fn test_cached_response_clone() {
        let response = EsiResponse::new(42);
        let cached = CachedResponse::Fresh(response);
        let cloned = cached.clone();

        assert!(cloned.is_fresh());
    }

    #[test]
    fn test_cached_response_not_modified_clone() {
        let cached: CachedResponse<EsiResponse<i32>> = CachedResponse::NotModified;
        let cloned = cached.clone();

        assert!(cloned.is_not_modified());
    }
}
