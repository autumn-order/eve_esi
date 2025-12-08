//! Integration tests for the `request_cached` method.
//!
//! Tests that the `request_cached` method properly handles cached requests,
//! 304 Not Modified responses, and various cache strategies (ETag, If-Modified-Since, Both).

use crate::util::integration_test_setup;
use eve_esi::{CacheStrategy, CachedResponse};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct TestData {
    message: String,
}

/// Tests cached request returning fresh data.
///
/// Verifies that when a cached request is made with conditional headers
/// and the server returns 200 OK, the response is properly wrapped in
/// CachedResponse::Fresh.
///
/// Expected: CachedResponse::Fresh with data
#[tokio::test]
async fn test_cached_request_returns_fresh_data() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/cached")
        .match_header("if-none-match", "\"old-etag\"")
        .with_status(200)
        .with_header("etag", "\"new-etag\"")
        .with_body(r#"{"message": "fresh data"}"#)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/cached");

    let response = request
        .send_cached(CacheStrategy::IfNoneMatch("\"old-etag\"".to_string()))
        .await?;

    assert!(response.is_fresh());
    assert!(!response.is_not_modified());

    if let CachedResponse::Fresh(esi_response) = response {
        assert_eq!(esi_response.data.message, "fresh data");
        assert_eq!(esi_response.cache.etag, "\"new-etag\"");
    } else {
        panic!("Expected Fresh response");
    }

    mock.assert_async().await;

    Ok(())
}

/// Tests cached request returning 304 Not Modified.
///
/// Verifies that when a cached request is made with conditional headers
/// and the server returns 304 Not Modified, the response is properly
/// wrapped in CachedResponse::NotModified.
///
/// Expected: CachedResponse::NotModified
#[tokio::test]
async fn test_cached_request_returns_not_modified() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/cached")
        .match_header("if-none-match", "\"current-etag\"")
        .with_status(304)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/cached");

    let response = request
        .send_cached(CacheStrategy::IfNoneMatch("\"current-etag\"".to_string()))
        .await?;

    assert!(!response.is_fresh());
    assert!(response.is_not_modified());

    match response {
        CachedResponse::NotModified => {
            // Success - this is what we expect
        }
        CachedResponse::Fresh(_) => {
            panic!("Expected NotModified response");
        }
    }

    mock.assert_async().await;

    Ok(())
}

/// Tests cached request with If-Modified-Since header.
///
/// Verifies that the If-Modified-Since header is properly formatted
/// and sent in the request when using IfModifiedSince cache strategy.
///
/// Expected: Request includes properly formatted If-Modified-Since header
#[tokio::test]
async fn test_cached_request_with_if_modified_since() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/cached")
        .match_header("if-modified-since", mockito::Matcher::Any)
        .with_status(304)
        .create_async()
        .await;

    let date = chrono::Utc::now();
    let request = client.esi().new_request::<TestData>("/test/cached");

    let response = request
        .send_cached(CacheStrategy::IfModifiedSince(date))
        .await?;

    assert!(response.is_not_modified());

    mock.assert_async().await;

    Ok(())
}

/// Tests cached request with both ETag and If-Modified-Since.
///
/// Verifies that when using the Both cache strategy, both conditional
/// headers are properly sent in the request.
///
/// Expected: Both headers are included in the request
#[tokio::test]
async fn test_cached_request_with_both_headers() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/cached")
        .match_header("if-none-match", "\"etag-value\"")
        .match_header("if-modified-since", mockito::Matcher::Any)
        .with_status(304)
        .create_async()
        .await;

    let date = chrono::Utc::now();
    let request = client.esi().new_request::<TestData>("/test/cached");

    let response = request
        .send_cached(CacheStrategy::Both {
            etag: "\"etag-value\"".to_string(),
            modified_since: date,
        })
        .await?;

    assert!(response.is_not_modified());

    mock.assert_async().await;

    Ok(())
}

/// Tests cached request with IfNoneMatch returns fresh data on different ETag.
///
/// Verifies that when the server returns a different ETag, the response
/// is treated as fresh data.
///
/// Expected: CachedResponse::Fresh with new ETag
#[tokio::test]
async fn test_cached_request_if_none_match_different_etag() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/cached")
        .match_header("if-none-match", "\"old-etag\"")
        .with_status(200)
        .with_header("etag", "\"new-etag\"")
        .with_body(r#"{"message": "updated content"}"#)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/cached");

    let response = request
        .send_cached(CacheStrategy::IfNoneMatch("\"old-etag\"".to_string()))
        .await?;

    assert!(response.is_fresh());

    if let CachedResponse::Fresh(esi_response) = response {
        assert_eq!(esi_response.data.message, "updated content");
        assert_eq!(esi_response.cache.etag, "\"new-etag\"");
    } else {
        panic!("Expected Fresh response");
    }

    mock.assert_async().await;

    Ok(())
}

/// Tests cached request with IfModifiedSince returns fresh data when modified.
///
/// Verifies that when content has been modified after the specified date,
/// the server returns 200 OK with fresh data.
///
/// Expected: CachedResponse::Fresh with updated data
#[tokio::test]
async fn test_cached_request_if_modified_since_content_modified() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/cached")
        .match_header("if-modified-since", mockito::Matcher::Any)
        .with_status(200)
        .with_header("last-modified", "Thu, 22 Oct 2015 08:30:00 GMT")
        .with_body(r#"{"message": "modified content"}"#)
        .create_async()
        .await;

    let old_date = chrono::DateTime::parse_from_rfc2822("Wed, 21 Oct 2015 07:28:00 GMT")
        .unwrap()
        .with_timezone(&chrono::Utc);

    let request = client.esi().new_request::<TestData>("/test/cached");

    let response = request
        .send_cached(CacheStrategy::IfModifiedSince(old_date))
        .await?;

    assert!(response.is_fresh());

    if let CachedResponse::Fresh(esi_response) = response {
        assert_eq!(esi_response.data.message, "modified content");
    } else {
        panic!("Expected Fresh response");
    }

    mock.assert_async().await;

    Ok(())
}

/// Tests cached request error handling.
///
/// Verifies that cached requests properly handle error responses (4xx/5xx)
/// and return errors appropriately.
///
/// Expected: Error is returned for 404 response
#[tokio::test]
async fn test_cached_request_handles_errors() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/cached")
        .match_header("if-none-match", "\"some-etag\"")
        .with_status(404)
        .with_body(r#"{"error": "Not found"}"#)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/cached");

    let result = request
        .send_cached(CacheStrategy::IfNoneMatch("\"some-etag\"".to_string()))
        .await;

    assert!(result.is_err());

    if let Err(eve_esi::Error::EsiResponseError(esi_err)) = result {
        assert_eq!(esi_err.status, 404);
        assert!(esi_err.message.contains("Not found"));
    } else {
        panic!("Expected EsiResponseError");
    }

    mock.assert_async().await;

    Ok(())
}

/// Tests cached request preserves cache headers in fresh responses.
///
/// Verifies that when a cached request returns fresh data (200 OK),
/// all cache headers are properly extracted and included.
///
/// Expected: Fresh response includes all cache headers
#[tokio::test]
async fn test_cached_request_fresh_includes_cache_headers() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/cached")
        .match_header("if-none-match", "\"old-etag\"")
        .with_status(200)
        .with_header("etag", "\"new-etag\"")
        .with_header("cache-control", "public, max-age=600")
        .with_header("last-modified", "Thu, 22 Oct 2015 09:00:00 GMT")
        .with_body(r#"{"message": "fresh with headers"}"#)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/cached");

    let response = request
        .send_cached(CacheStrategy::IfNoneMatch("\"old-etag\"".to_string()))
        .await?;

    if let CachedResponse::Fresh(esi_response) = response {
        assert_eq!(esi_response.cache.etag, "\"new-etag\"");
        assert_eq!(esi_response.cache.cache_control, "public, max-age=600");
        assert_eq!(esi_response.data.message, "fresh with headers");
    } else {
        panic!("Expected Fresh response");
    }

    mock.assert_async().await;

    Ok(())
}

/// Tests cached request with Both strategy returning 304.
///
/// Verifies that when using the Both cache strategy (ETag + If-Modified-Since),
/// a 304 Not Modified response is properly handled.
///
/// Expected: CachedResponse::NotModified
#[tokio::test]
async fn test_cached_request_both_strategy_not_modified() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/cached")
        .match_header("if-none-match", "\"current-etag\"")
        .match_header("if-modified-since", mockito::Matcher::Any)
        .with_status(304)
        .create_async()
        .await;

    let date = chrono::Utc::now();
    let request = client.esi().new_request::<TestData>("/test/cached");

    let response = request
        .send_cached(CacheStrategy::Both {
            etag: "\"current-etag\"".to_string(),
            modified_since: date,
        })
        .await?;

    assert!(response.is_not_modified());

    mock.assert_async().await;

    Ok(())
}

/// Tests cached request with Both strategy returning fresh data.
///
/// Verifies that when using the Both cache strategy and the content has changed,
/// the server returns 200 OK with fresh data.
///
/// Expected: CachedResponse::Fresh with updated data and new headers
#[tokio::test]
async fn test_cached_request_both_strategy_fresh_data() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/cached")
        .match_header("if-none-match", "\"old-etag\"")
        .match_header("if-modified-since", mockito::Matcher::Any)
        .with_status(200)
        .with_header("etag", "\"updated-etag\"")
        .with_header("last-modified", "Fri, 23 Oct 2015 10:00:00 GMT")
        .with_body(r#"{"message": "both strategy fresh"}"#)
        .create_async()
        .await;

    let old_date = chrono::DateTime::parse_from_rfc2822("Wed, 21 Oct 2015 07:28:00 GMT")
        .unwrap()
        .with_timezone(&chrono::Utc);

    let request = client.esi().new_request::<TestData>("/test/cached");

    let response = request
        .send_cached(CacheStrategy::Both {
            etag: "\"old-etag\"".to_string(),
            modified_since: old_date,
        })
        .await?;

    assert!(response.is_fresh());

    if let CachedResponse::Fresh(esi_response) = response {
        assert_eq!(esi_response.data.message, "both strategy fresh");
        assert_eq!(esi_response.cache.etag, "\"updated-etag\"");
    } else {
        panic!("Expected Fresh response");
    }

    mock.assert_async().await;

    Ok(())
}

/// Tests deserialization failure handling in cached requests.
///
/// Verifies that when a cached request returns fresh data (200 OK) but
/// the response body cannot be deserialized into the expected struct,
/// an appropriate error is returned.
///
/// Expected: Error due to deserialization failure
#[tokio::test]
async fn test_cached_request_deserialization_failure() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    // Return a 200 response with body that doesn't match TestData structure
    let mock = server
        .mock("GET", "/test/cached")
        .match_header("if-none-match", "\"some-etag\"")
        .with_status(200)
        .with_header("etag", "\"new-etag\"")
        .with_body(r#"{"wrong_field": "value", "another": 123}"#)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/cached");

    let result = request
        .send_cached(CacheStrategy::IfNoneMatch("\"some-etag\"".to_string()))
        .await;

    assert!(result.is_err());
    // Should be a serde_json error
    if let Err(eve_esi::Error::SerdeJsonError(_)) = result {
        // Expected error type
    } else {
        panic!("Expected SerdeJsonError, got: {:?}", result);
    }

    mock.assert_async().await;

    Ok(())
}
