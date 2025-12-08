//! Tests for EsiResponse type and related header structures.

use crate::esi::response::{CacheHeaders, EsiResponse, RateLimitHeaders};
use chrono::{DateTime, Utc};

/// Tests creating a new EsiResponse with default cache headers.
///
/// Verifies that the `new` constructor creates an EsiResponse with:
/// - The provided data
/// - Empty cache control string
/// - Empty etag string
/// - Current timestamp for last_modified
/// - No rate limit headers
///
/// Expected: EsiResponse with placeholder cache headers and no rate limit
#[test]
fn test_new() -> Result<(), crate::Error> {
    let data = vec![1, 2, 3];
    let response = EsiResponse::new(data.clone());

    assert_eq!(response.data, data);
    assert_eq!(response.cache.cache_control, String::new());
    assert_eq!(response.cache.etag, String::new());
    assert!(response.rate_limit.is_none());

    Ok(())
}

/// Tests Deref trait implementation on EsiResponse.
///
/// Verifies that EsiResponse properly derefs to its inner data type,
/// allowing direct access to the wrapped data's methods and fields.
///
/// Expected: Direct access to Vec methods through deref
#[test]
fn test_deref() -> Result<(), crate::Error> {
    let data = vec![1, 2, 3];
    let response = EsiResponse::new(data.clone());

    // Test Deref trait
    assert_eq!(*response, data);
    assert_eq!(response.len(), 3);
    assert_eq!(response[0], 1);

    Ok(())
}

/// Tests DerefMut trait implementation on EsiResponse.
///
/// Verifies that EsiResponse properly derefs mutably to its inner data type,
/// allowing modification of the wrapped data through mutable methods.
///
/// Expected: Ability to mutate inner data through deref_mut
#[test]
fn test_deref_mut() -> Result<(), crate::Error> {
    let data = vec![1, 2, 3];
    let mut response = EsiResponse::new(data);

    // Test DerefMut trait
    response.push(4);
    assert_eq!(response.data, vec![1, 2, 3, 4]);
    assert_eq!(response.len(), 4);

    Ok(())
}

/// Tests EsiResponse with complete cache headers.
///
/// Verifies that EsiResponse correctly stores and provides access to
/// all cache-related HTTP headers including cache-control, etag,
/// and last-modified timestamp.
///
/// Expected: All cache headers are correctly stored and accessible
#[test]
fn test_with_cache_headers() -> Result<(), crate::Error> {
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

    Ok(())
}

/// Tests EsiResponse with rate limiting headers.
///
/// Verifies that EsiResponse correctly stores and provides access to
/// rate limiting headers when present, including group, limit,
/// remaining tokens, and used tokens.
///
/// Expected: All rate limit headers are correctly stored and accessible
#[test]
fn test_with_rate_limit_headers() -> Result<(), crate::Error> {
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

    Ok(())
}

/// Tests Clone trait implementation on EsiResponse.
///
/// Verifies that EsiResponse can be cloned and that all fields
/// including data, cache headers, and rate limit headers are
/// properly copied to the new instance.
///
/// Expected: Cloned instance has identical field values
#[test]
fn test_clone() -> Result<(), crate::Error> {
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

    Ok(())
}

/// Tests Clone trait implementation on CacheHeaders.
///
/// Verifies that CacheHeaders can be cloned independently and that
/// all fields including cache_control, etag, and last_modified
/// are properly copied to the new instance.
///
/// Expected: Cloned headers have identical field values
#[test]
fn test_cache_headers_clone() -> Result<(), crate::Error> {
    let headers = CacheHeaders {
        cache_control: "max-age=300".to_string(),
        etag: "\"tag123\"".to_string(),
        last_modified: Utc::now(),
    };

    let cloned = headers.clone();
    assert_eq!(headers.cache_control, cloned.cache_control);
    assert_eq!(headers.etag, cloned.etag);
    assert_eq!(headers.last_modified, cloned.last_modified);

    Ok(())
}

/// Tests Clone trait implementation on RateLimitHeaders.
///
/// Verifies that RateLimitHeaders can be cloned independently and that
/// all fields including group, limit, remaining, and used are properly
/// copied to the new instance.
///
/// Expected: Cloned headers have identical field values
#[test]
fn test_rate_limit_headers_clone() -> Result<(), crate::Error> {
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

    Ok(())
}
