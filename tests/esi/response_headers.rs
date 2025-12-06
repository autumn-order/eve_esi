//! Integration tests for EsiResponse header extraction

use eve_esi::Client;
use mockito::Server;
use reqwest::Method;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct TestResponse {
    value: String,
}

#[tokio::test]
async fn test_esi_response_cache_headers() {
    let mut server = Server::new_async().await;
    let user_agent = "TestAgent/1.0";
    let client = Client::new(user_agent).expect("Failed to create client");

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

    let url = format!("{}/test", server.url());
    let request = client
        .esi()
        .new_request::<TestResponse>(url)
        .with_method(Method::GET);

    let response = request.send().await.expect("Request failed");

    // Verify data
    assert_eq!(response.data.value, "test data");

    // Verify cache headers
    assert_eq!(
        response.cache.cache_control.as_deref(),
        Some("public, max-age=300")
    );
    assert_eq!(response.cache.etag.as_deref(), Some("test-etag-123"));
    assert!(response.cache.last_modified.is_some());

    mock.assert_async().await;
}

#[tokio::test]
async fn test_esi_response_rate_limit_headers() {
    let mut server = Server::new_async().await;
    let user_agent = "TestAgent/1.0";
    let client = Client::new(user_agent).expect("Failed to create client");

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

    let url = format!("{}/test", server.url());
    let request = client
        .esi()
        .new_request::<TestResponse>(url)
        .with_method(Method::GET);

    let response = request.send().await.expect("Request failed");

    // Verify data
    assert_eq!(response.data.value, "test data");

    // Verify rate limit headers
    assert_eq!(response.rate_limit.group.as_deref(), Some("esi-search"));
    assert_eq!(response.rate_limit.limit.as_deref(), Some("150/15m"));
    assert_eq!(response.rate_limit.remaining, Some(145));
    assert_eq!(response.rate_limit.used, Some(5));
    assert!(response.rate_limit.retry_after.is_none());

    mock.assert_async().await;
}

#[tokio::test]
async fn test_esi_response_retry_after_header() {
    let mut server = Server::new_async().await;
    let user_agent = "TestAgent/1.0";
    let client = Client::new(user_agent).expect("Failed to create client");

    // Mock endpoint that returns 429 with retry-after
    let mock = server
        .mock("GET", "/test")
        .with_status(429)
        .with_header("Retry-After", "60")
        .with_body(r#"{"error": "rate limited"}"#)
        .create_async()
        .await;

    let url = format!("{}/test", server.url());
    let request = client
        .esi()
        .new_request::<serde_json::Value>(url)
        .with_method(Method::GET);

    // This should fail with a status error, but that's expected for 429
    let result = request.send().await;
    assert!(result.is_err());

    mock.assert_async().await;
}

#[tokio::test]
async fn test_esi_response_deref() {
    let mut server = Server::new_async().await;
    let user_agent = "TestAgent/1.0";
    let client = Client::new(user_agent).expect("Failed to create client");

    server
        .mock("GET", "/test")
        .with_status(200)
        .with_body(r#"{"value": "test data"}"#)
        .create_async()
        .await;

    let url = format!("{}/test", server.url());
    let request = client
        .esi()
        .new_request::<TestResponse>(url)
        .with_method(Method::GET);

    let response = request.send().await.expect("Request failed");

    // Test Deref trait - we can access data fields directly
    assert_eq!(response.value, "test data");
}

#[tokio::test]
async fn test_cached_response_with_esi_response() {
    let mut server = Server::new_async().await;
    let user_agent = "TestAgent/1.0";
    let client = Client::new(user_agent).expect("Failed to create client");

    // Mock fresh response with headers
    let mock = server
        .mock("GET", "/test")
        .with_status(200)
        .with_header("ETag", "new-etag")
        .with_header("Cache-Control", "public, max-age=600")
        .with_header("X-Esi-Error-Limit-Remain", "100")
        .with_body(r#"{"value": "fresh data"}"#)
        .create_async()
        .await;

    let url = format!("{}/test", server.url());
    let request = client
        .esi()
        .new_request::<TestResponse>(url)
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
        assert_eq!(esi_response.cache.etag.as_deref(), Some("new-etag"));
        assert_eq!(
            esi_response.cache.cache_control.as_deref(),
            Some("public, max-age=600")
        );

        // Verify rate limit headers from EsiResponse
        assert_eq!(esi_response.rate_limit.remaining, Some(100));
    } else {
        panic!("Expected fresh response with data");
    }

    mock.assert_async().await;
}
