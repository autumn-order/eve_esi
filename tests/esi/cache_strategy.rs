//! Integration tests for CacheStrategy and send_with_cache API

use chrono::{DateTime, Utc};
use eve_esi::CacheStrategy;
use reqwest::Method;
use serde::Deserialize;

use crate::util::integration_test_setup;

#[derive(Deserialize, Debug, PartialEq)]
struct TestResponse {
    value: String,
}

#[tokio::test]
async fn test_cache_strategy_if_none_match() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    // Mock endpoint that returns 304 Not Modified when If-None-Match matches
    let mock = server
        .mock("GET", "/test")
        .match_header("If-None-Match", "test-etag-123")
        .with_status(304)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestResponse>("/test")
        .with_method(Method::GET);

    let response = request
        .send_cached(CacheStrategy::IfNoneMatch("test-etag-123".to_string()))
        .await?;

    assert!(response.is_not_modified());
    assert!(matches!(response, eve_esi::CachedResponse::NotModified));

    mock.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_cache_strategy_if_modified_since() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    // Parse a specific date for testing
    let test_date: DateTime<Utc> = "2015-10-21T07:28:00Z".parse().unwrap();
    let expected_header = test_date.to_rfc2822(); // Will be "Wed, 21 Oct 2015 07:28:00 +0000"

    // Mock endpoint that returns 304 Not Modified when If-Modified-Since matches
    let mock = server
        .mock("GET", "/test")
        .match_header("If-Modified-Since", expected_header.as_str())
        .with_status(304)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestResponse>("/test")
        .with_method(Method::GET);

    let response = request
        .send_cached(CacheStrategy::IfModifiedSince(test_date))
        .await?;

    assert!(response.is_not_modified());

    mock.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_cache_strategy_both_headers() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    // Parse a specific date for testing
    let test_date: DateTime<Utc> = "2015-10-22T08:00:00Z".parse().unwrap();
    let expected_header = test_date.to_rfc2822();

    // Mock endpoint that expects both headers
    let mock = server
        .mock("GET", "/test")
        .match_header("If-None-Match", "test-etag-456")
        .match_header("If-Modified-Since", expected_header.as_str())
        .with_status(304)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestResponse>("/test")
        .with_method(Method::GET);

    let response = request
        .send_cached(CacheStrategy::Both {
            etag: "test-etag-456".to_string(),
            modified_since: test_date,
        })
        .await?;

    assert!(response.is_not_modified());

    mock.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_cache_strategy_fresh_data_with_etag() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    // Mock endpoint that returns fresh data with ETag
    let mock = server
        .mock("GET", "/test")
        .match_header("If-None-Match", "old-etag")
        .with_status(200)
        .with_header("ETag", "new-etag-789")
        .with_body(r#"{"value": "fresh data"}"#)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestResponse>("/test")
        .with_method(Method::GET);

    let response = request
        .send_cached(CacheStrategy::IfNoneMatch("old-etag".to_string()))
        .await?;

    assert!(response.is_fresh());
    let eve_esi::CachedResponse::Fresh(esi_response) = response else {
        panic!("Expected fresh response");
    };
    assert_eq!(
        &esi_response.data,
        &TestResponse {
            value: "fresh data".to_string()
        }
    );
    assert_eq!(esi_response.cache.etag, "new-etag-789");

    mock.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_send_without_cache_no_conditional_headers() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    // Mock endpoint that should not receive cache headers
    let mock = server
        .mock("GET", "/test")
        .with_status(200)
        .with_body(r#"{"value": "normal data"}"#)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestResponse>("/test")
        .with_method(Method::GET);

    let response = request.send().await?;

    assert_eq!(
        response.data,
        TestResponse {
            value: "normal data".to_string()
        }
    );

    mock.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_cached_response_into_data() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    // Mock fresh response
    server
        .mock("GET", "/test")
        .with_status(200)
        .with_header("ETag", "test-etag")
        .with_body(r#"{"value": "test"}"#)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestResponse>("/test")
        .with_method(Method::GET);

    let response = request
        .send_cached(CacheStrategy::IfNoneMatch("wrong-etag".to_string()))
        .await?;

    let eve_esi::CachedResponse::Fresh(esi_response) = response else {
        panic!("Expected fresh response with data");
    };
    assert_eq!(esi_response.data.value, "test");

    Ok(())
}

#[tokio::test]
async fn test_cached_response_not_modified_into_data() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    // Mock 304 response
    server
        .mock("GET", "/test")
        .with_status(304)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestResponse>("/test")
        .with_method(Method::GET);

    let response = request
        .send_cached(CacheStrategy::IfNoneMatch("matching-etag".to_string()))
        .await?;

    assert!(matches!(response, eve_esi::CachedResponse::NotModified));

    Ok(())
}

#[tokio::test]
async fn test_fresh_response_with_last_modified() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    // Parse a specific date for the Last-Modified header
    let test_date: DateTime<Utc> = "2024-01-15T10:30:00Z".parse().unwrap();
    let last_modified_header = test_date.to_rfc2822();

    // Mock fresh response with both ETag and Last-Modified
    server
        .mock("GET", "/test")
        .with_status(200)
        .with_header("ETag", "test-etag-789")
        .with_header("Last-Modified", last_modified_header.as_str())
        .with_body(r#"{"value": "fresh data"}"#)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestResponse>("/test")
        .with_method(Method::GET);

    let response = request
        .send_cached(CacheStrategy::IfNoneMatch("old-etag".to_string()))
        .await?;

    assert!(response.is_fresh());
    let eve_esi::CachedResponse::Fresh(ref esi_response) = response else {
        panic!("Expected fresh response");
    };
    assert_eq!(esi_response.cache.etag, "test-etag-789");
    assert_eq!(esi_response.cache.last_modified, test_date);

    // Extract the EsiResponse by destructuring
    let eve_esi::CachedResponse::Fresh(esi_response) = response else {
        panic!("Expected fresh response");
    };
    assert_eq!(esi_response.data.value, "fresh data");
    assert_eq!(esi_response.cache.etag, "test-etag-789");
    assert_eq!(esi_response.cache.last_modified, test_date);

    Ok(())
}

#[tokio::test]
async fn test_use_last_modified_for_next_request() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    // First request - get fresh data with Last-Modified header
    let test_date: DateTime<Utc> = "2024-01-15T10:30:00Z".parse().unwrap();
    let last_modified_header = test_date.to_rfc2822();

    let mock1 = server
        .mock("GET", "/test")
        .with_status(200)
        .with_header("Last-Modified", last_modified_header.as_str())
        .with_body(r#"{"value": "initial data"}"#)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestResponse>("/test")
        .with_method(Method::GET);

    let response = request.send().await?;
    assert_eq!(response.value, "initial data");

    mock1.assert_async().await;

    // Second request - use Last-Modified for caching
    let mock2 = server
        .mock("GET", "/test")
        .match_header("If-Modified-Since", last_modified_header.as_str())
        .with_status(304)
        .create_async()
        .await;

    let request2 = client
        .esi()
        .new_request::<TestResponse>("/test")
        .with_method(Method::GET);
    let cached_response = request2
        .send_cached(CacheStrategy::IfModifiedSince(test_date))
        .await?;

    assert!(cached_response.is_not_modified());
    mock2.assert_async().await;

    Ok(())
}
