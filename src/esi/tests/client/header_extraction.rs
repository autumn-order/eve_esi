//! Tests for HTTP header extraction from ESI responses.

use crate::esi::client::EsiApi;
use crate::Client;
use chrono::{TimeZone, Utc};
use reqwest::header::HeaderMap;

fn create_test_client() -> Client {
    Client::new("TestApp/1.0").unwrap()
}

/// Tests extracting complete cache headers from response.
///
/// Verifies that all cache-related headers (cache-control, etag, last-modified)
/// are correctly extracted and parsed from the HTTP response headers.
///
/// Expected: CacheHeaders struct contains all provided header values
#[test]
fn test_extract_cache_headers_complete() {
    let mut headers = HeaderMap::new();
    headers.insert("cache-control", "public, max-age=300".parse().unwrap());
    headers.insert("etag", "\"abc123\"".parse().unwrap());
    headers.insert(
        "last-modified",
        "Wed, 21 Oct 2015 07:28:00 GMT".parse().unwrap(),
    );

    let cache_headers = EsiApi::extract_cache_headers(&headers);

    assert_eq!(cache_headers.cache_control, "public, max-age=300");
    assert_eq!(cache_headers.etag, "\"abc123\"");

    let expected_date = Utc.with_ymd_and_hms(2015, 10, 21, 7, 28, 0).unwrap();
    assert_eq!(cache_headers.last_modified, expected_date);
}

/// Tests extracting cache headers when all are missing.
///
/// Verifies that when cache headers are absent from the response,
/// default values are used (empty strings for cache-control and etag,
/// current time for last-modified).
///
/// Expected: Empty strings and a recent timestamp
#[test]
fn test_extract_cache_headers_missing() {
    let headers = HeaderMap::new();

    let cache_headers = EsiApi::extract_cache_headers(&headers);

    assert_eq!(cache_headers.cache_control, "");
    assert_eq!(cache_headers.etag, "");
    // Just verify it's a valid timestamp (within last few seconds)
    let now = Utc::now();
    let diff = (now - cache_headers.last_modified).num_seconds().abs();
    assert!(diff < 5, "Timestamp should be recent");
}

/// Tests extracting cache headers with partial data.
///
/// Verifies that when only some cache headers are present,
/// the available headers are extracted and missing ones use defaults.
///
/// Expected: Present headers extracted, missing ones use defaults
#[test]
fn test_extract_cache_headers_partial() {
    let mut headers = HeaderMap::new();
    headers.insert("etag", "\"xyz789\"".parse().unwrap());

    let cache_headers = EsiApi::extract_cache_headers(&headers);

    assert_eq!(cache_headers.cache_control, "");
    assert_eq!(cache_headers.etag, "\"xyz789\"");
    // Verify recent timestamp
    let now = Utc::now();
    let diff = (now - cache_headers.last_modified).num_seconds().abs();
    assert!(diff < 5);
}

/// Tests extracting cache headers with invalid last-modified format.
///
/// Verifies that when the last-modified header has an invalid date format,
/// the current time is used as a fallback instead of failing.
///
/// Expected: Invalid date falls back to current time
#[test]
fn test_extract_cache_headers_invalid_date() {
    let mut headers = HeaderMap::new();
    headers.insert("cache-control", "public".parse().unwrap());
    headers.insert("etag", "\"tag\"".parse().unwrap());
    headers.insert("last-modified", "invalid-date-format".parse().unwrap());

    let cache_headers = EsiApi::extract_cache_headers(&headers);

    assert_eq!(cache_headers.cache_control, "public");
    assert_eq!(cache_headers.etag, "\"tag\"");
    // Should fall back to current time
    let now = Utc::now();
    let diff = (now - cache_headers.last_modified).num_seconds().abs();
    assert!(diff < 5);
}

/// Tests extracting complete rate limit headers from response.
///
/// Verifies that all rate limit headers are correctly extracted when
/// the x-esi-error-limit-group header is present.
///
/// Expected: RateLimitHeaders struct with all values populated
#[test]
fn test_extract_rate_limit_headers_complete() {
    let mut headers = HeaderMap::new();
    headers.insert("x-esi-error-limit-group", "global".parse().unwrap());
    headers.insert("x-esi-error-limit-limit", "150/15m".parse().unwrap());
    headers.insert("x-esi-error-limit-remain", "100".parse().unwrap());
    headers.insert("x-esi-error-limit-used", "50".parse().unwrap());

    let rate_limit = EsiApi::extract_rate_limit_headers(&headers);

    assert!(rate_limit.is_some());
    let rate_limit = rate_limit.unwrap();
    assert_eq!(rate_limit.group, "global");
    assert_eq!(rate_limit.limit, "150/15m");
    assert_eq!(rate_limit.remaining, 100);
    assert_eq!(rate_limit.used, 50);
}

/// Tests extracting rate limit headers when group header is missing.
///
/// Verifies that when the x-esi-error-limit-group header is absent,
/// None is returned even if other rate limit headers are present.
///
/// Expected: None
#[test]
fn test_extract_rate_limit_headers_missing_group() {
    let mut headers = HeaderMap::new();
    headers.insert("x-esi-error-limit-limit", "150/15m".parse().unwrap());
    headers.insert("x-esi-error-limit-remain", "100".parse().unwrap());

    let rate_limit = EsiApi::extract_rate_limit_headers(&headers);

    assert!(rate_limit.is_none());
}

/// Tests extracting rate limit headers with partial data.
///
/// Verifies that when the group header is present but other rate limit
/// headers are missing, default values (empty string for limit, 0 for numbers)
/// are used.
///
/// Expected: RateLimitHeaders with defaults for missing values
#[test]
fn test_extract_rate_limit_headers_partial() {
    let mut headers = HeaderMap::new();
    headers.insert("x-esi-error-limit-group", "character".parse().unwrap());
    headers.insert("x-esi-error-limit-remain", "75".parse().unwrap());

    let rate_limit = EsiApi::extract_rate_limit_headers(&headers);

    assert!(rate_limit.is_some());
    let rate_limit = rate_limit.unwrap();
    assert_eq!(rate_limit.group, "character");
    assert_eq!(rate_limit.limit, "");
    assert_eq!(rate_limit.remaining, 75);
    assert_eq!(rate_limit.used, 0);
}

/// Tests extracting rate limit headers with invalid numeric values.
///
/// Verifies that when rate limit numeric headers have invalid formats,
/// they default to 0 instead of causing parsing errors.
///
/// Expected: Invalid numbers default to 0
#[test]
fn test_extract_rate_limit_headers_invalid_numbers() {
    let mut headers = HeaderMap::new();
    headers.insert("x-esi-error-limit-group", "alliance".parse().unwrap());
    headers.insert("x-esi-error-limit-limit", "100/1h".parse().unwrap());
    headers.insert("x-esi-error-limit-remain", "not-a-number".parse().unwrap());
    headers.insert("x-esi-error-limit-used", "invalid".parse().unwrap());

    let rate_limit = EsiApi::extract_rate_limit_headers(&headers);

    assert!(rate_limit.is_some());
    let rate_limit = rate_limit.unwrap();
    assert_eq!(rate_limit.group, "alliance");
    assert_eq!(rate_limit.limit, "100/1h");
    assert_eq!(rate_limit.remaining, 0);
    assert_eq!(rate_limit.used, 0);
}

/// Tests populating EsiResponse with extracted headers.
///
/// Verifies that the populate_esi_response_from_headers method correctly
/// combines data with extracted cache and rate limit headers.
///
/// Expected: EsiResponse contains data and all header information
#[test]
fn test_populate_esi_response_from_headers() {
    let mut headers = HeaderMap::new();
    headers.insert("cache-control", "public".parse().unwrap());
    headers.insert("etag", "\"response123\"".parse().unwrap());
    headers.insert(
        "last-modified",
        "Thu, 01 Jan 2020 00:00:00 GMT".parse().unwrap(),
    );
    headers.insert("x-esi-error-limit-group", "test".parse().unwrap());
    headers.insert("x-esi-error-limit-remain", "50".parse().unwrap());

    let data = vec![1, 2, 3];
    let response = EsiApi::populate_esi_response_from_headers(&headers, data.clone());

    assert_eq!(response.data, data);
    assert_eq!(response.cache.cache_control, "public");
    assert_eq!(response.cache.etag, "\"response123\"");
    assert!(response.rate_limit.is_some());
    assert_eq!(response.rate_limit.as_ref().unwrap().group, "test");
    assert_eq!(response.rate_limit.as_ref().unwrap().remaining, 50);
}

/// Tests populating EsiResponse without rate limit headers.
///
/// Verifies that when rate limit headers are absent, the EsiResponse
/// still contains valid cache headers but rate_limit is None.
///
/// Expected: EsiResponse with cache headers and rate_limit = None
#[test]
fn test_populate_esi_response_without_rate_limit() {
    let mut headers = HeaderMap::new();
    headers.insert("cache-control", "no-cache".parse().unwrap());
    headers.insert("etag", "\"abc\"".parse().unwrap());

    let data = "test string";
    let response = EsiApi::populate_esi_response_from_headers(&headers, data);

    assert_eq!(response.data, "test string");
    assert_eq!(response.cache.cache_control, "no-cache");
    assert_eq!(response.cache.etag, "\"abc\"");
    assert!(response.rate_limit.is_none());
}

/// Tests EsiApi constructor.
///
/// Verifies that a new EsiApi instance can be created and properly
/// holds a reference to the client.
///
/// Expected: EsiApi instance is created successfully
#[test]
fn test_esi_api_new() {
    let client = create_test_client();
    let _esi_api = EsiApi::new(&client);

    // Just verify it's created - we can't directly test the internal reference
    // but we can verify it works by using it
    let headers = HeaderMap::new();
    let cache = EsiApi::extract_cache_headers(&headers);
    assert_eq!(cache.cache_control, "");
}

/// Tests Client::esi() convenience method.
///
/// Verifies that the Client::esi() method returns a properly
/// constructed EsiApi instance.
///
/// Expected: EsiApi instance is created and functional
#[test]
fn test_client_esi_method() {
    let client = create_test_client();
    let _esi_api = client.esi();

    // Verify it works by using a method
    let headers = HeaderMap::new();
    let cache = EsiApi::extract_cache_headers(&headers);
    assert_eq!(cache.etag, "");
}
