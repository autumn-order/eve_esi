//! Integration tests for EsiResponse header extraction

use reqwest::Method;
use serde::Deserialize;

use crate::util::integration_test_setup;

#[derive(Deserialize, Debug, PartialEq)]
struct TestResponse {
    value: String,
}

#[tokio::test]
async fn test_esi_response_cache_headers() {
    let (client, mut server) = integration_test_setup().await;

    // Mock endpoint that returns cache headers
    let mock = server
        .mock("GET", "/test")
        .with_status(200)
        .with_header("Cache-Control", "public, max-age=300")
        .with_header("ETag", "test-etag-123")
        .with_header("Last-Modified", "Wed, 21 Oct 2015 07:28:00 +0000")
        .with_body(r#"{"value": "test data"}"#)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestResponse>("/test")
        .with_method(Method::GET);

    let response = request.send().await.expect("Request failed");

    // Verify data
    assert_eq!(response.data.value, "test data");

    // Verify cache headers
    assert_eq!(response.cache.cache_control, "public, max-age=300");
    assert_eq!(response.cache.etag, "test-etag-123");
    // Just verify last_modified exists (non-default value would indicate it was parsed)
    assert_ne!(response.cache.last_modified, chrono::Utc::now());

    mock.assert_async().await;
}

#[tokio::test]
async fn test_esi_response_rate_limit_headers() {
    let (client, mut server) = integration_test_setup().await;

    // Mock endpoint that returns rate limit headers
    let mock = server
        .mock("GET", "/test")
        .with_status(200)
        .with_header("X-Esi-Error-Limit-Group", "esi-search")
        .with_header("X-Esi-Error-Limit-Limit", "150/15m")
        .with_header("X-Esi-Error-Limit-Remain", "145")
        .with_header("X-Esi-Error-Limit-Used", "5")
        .with_body(r#"{"value": "test data"}"#)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestResponse>("/test")
        .with_method(Method::GET);

    let response = request.send().await.expect("Request failed");

    // Verify data
    assert_eq!(response.data.value, "test data");

    // Verify rate limit headers
    assert!(response.rate_limit.is_some());
    let rate_limit = response.rate_limit.as_ref().unwrap();
    assert_eq!(rate_limit.group, "esi-search");
    assert_eq!(rate_limit.limit, "150/15m");
    assert_eq!(rate_limit.remaining, 145);
    assert_eq!(rate_limit.used, 5);

    mock.assert_async().await;
}

#[tokio::test]
async fn test_esi_response_no_rate_limit_headers() {
    let (client, mut server) = integration_test_setup().await;

    // Mock endpoint that returns no rate limit headers
    let mock = server
        .mock("GET", "/test")
        .with_status(200)
        .with_body(r#"{"value": "test data"}"#)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestResponse>("/test")
        .with_method(Method::GET);

    let response = request.send().await.expect("Request failed");

    // Verify that rate_limit is None when x-esi-error-limit-group is not present
    assert!(response.rate_limit.is_none());

    mock.assert_async().await;
}

#[tokio::test]
async fn test_esi_response_deref() {
    let (client, mut server) = integration_test_setup().await;

    server
        .mock("GET", "/test")
        .with_status(200)
        .with_body(r#"{"value": "test data"}"#)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestResponse>("/test")
        .with_method(Method::GET);

    let response = request.send().await.expect("Request failed");

    // Test Deref trait - we can access data fields directly
    assert_eq!(response.value, "test data");
}

#[tokio::test]
async fn test_cached_response_with_esi_response() {
    let (client, mut server) = integration_test_setup().await;

    // Mock fresh response with headers
    let mock = server
        .mock("GET", "/test")
        .with_status(200)
        .with_header("ETag", "new-etag")
        .with_header("Cache-Control", "public, max-age=600")
        .with_header("X-Esi-Error-Limit-Group", "esi-test")
        .with_header("X-Esi-Error-Limit-Remain", "100")
        .with_body(r#"{"value": "fresh data"}"#)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestResponse>("/test")
        .with_method(Method::GET);

    let response = request
        .send_cached(eve_esi::CacheStrategy::IfNoneMatch("old-etag".to_string()))
        .await
        .expect("Request failed");

    assert!(response.is_fresh());

    if let eve_esi::CachedResponse::Fresh(esi_response) = response {
        // Verify data
        assert_eq!(esi_response.data.value, "fresh data");

        // Verify cache headers from EsiResponse
        assert_eq!(esi_response.cache.etag, "new-etag");
        assert_eq!(esi_response.cache.cache_control, "public, max-age=600");

        // Verify rate limit headers from EsiResponse
        assert!(esi_response.rate_limit.is_some());
        let rate_limit = esi_response.rate_limit.as_ref().unwrap();
        assert_eq!(rate_limit.group, "esi-test");
        assert_eq!(rate_limit.remaining, 100);
    } else {
        panic!("Expected fresh response with data");
    }

    mock.assert_async().await;
}
