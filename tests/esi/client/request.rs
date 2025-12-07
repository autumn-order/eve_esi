//! Integration tests for the `request` method.
//!
//! Tests that the `request` method properly handles various HTTP methods,
//! error responses, authorization, custom headers, and response deserialization.

use crate::oauth2::util::jwk_response::get_jwk_success_response;
use crate::oauth2::util::jwt::create_mock_token;
use crate::util::integration_test_setup;
use oauth2::TokenResponse;
use reqwest::Method;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct TestData {
    message: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct TestArray {
    items: Vec<String>,
}

/// Tests successful GET request with deserialized response.
///
/// Verifies that the client can successfully make a GET request,
/// deserialize the JSON response, and return the data wrapped in EsiResponse.
///
/// Expected: Request succeeds with correctly deserialized data
#[tokio::test]
async fn test_successful_get_request() {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/endpoint")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "success"}"#)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestData>("/test/endpoint")
        .with_method(Method::GET);

    let response = request.send().await.expect("Request should succeed");

    assert_eq!(response.data.message, "success");

    mock.assert_async().await;
}

/// Tests successful POST request with JSON body.
///
/// Verifies that the client can make a POST request with a JSON body
/// and receive a successful response.
///
/// Expected: POST request with body succeeds
#[tokio::test]
async fn test_successful_post_request_with_body() {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("POST", "/test/create")
        .match_header("content-type", "application/json")
        .match_body(r#"{"name":"test"}"#)
        .with_status(201)
        .with_body(r#"{"message": "created"}"#)
        .create_async()
        .await;

    let body = serde_json::json!({"name": "test"});

    let request = client
        .esi()
        .new_request::<TestData>("/test/create")
        .with_method(Method::POST)
        .with_body_json(body);

    let response = request.send().await.expect("Request should succeed");

    assert_eq!(response.data.message, "created");

    mock.assert_async().await;
}

/// Tests PUT request with body.
///
/// Verifies that the client can make PUT requests with a JSON body
/// for update operations.
///
/// Expected: PUT request with body succeeds
#[tokio::test]
async fn test_put_request_with_body() {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("PUT", "/test/update")
        .match_body(r#"{"message":"updated"}"#)
        .with_status(200)
        .with_body(r#"{"message": "update successful"}"#)
        .create_async()
        .await;

    let body = serde_json::json!({"message": "updated"});

    let request = client
        .esi()
        .new_request::<TestData>("/test/update")
        .with_method(Method::PUT)
        .with_body_json(body);

    let response = request.send().await.expect("Request should succeed");

    assert_eq!(response.data.message, "update successful");

    mock.assert_async().await;
}

/// Tests DELETE request.
///
/// Verifies that the client can make DELETE requests successfully.
///
/// Expected: DELETE request succeeds
#[tokio::test]
async fn test_delete_request() {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("DELETE", "/test/delete/123")
        .with_status(200)
        .with_body(r#"{"message": "deleted"}"#)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestData>("/test/delete/123")
        .with_method(Method::DELETE);

    let response = request.send().await.expect("Request should succeed");

    assert_eq!(response.data.message, "deleted");

    mock.assert_async().await;
}

/// Tests request with custom headers.
///
/// Verifies that custom headers set via with_header are properly
/// sent in the HTTP request.
///
/// Expected: Custom headers are included in the request
#[tokio::test]
async fn test_request_with_custom_headers() {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/headers")
        .match_header("X-Custom-Header", "custom-value")
        .match_header("X-Tenant", "tranquility")
        .with_status(200)
        .with_body(r#"{"message": "headers received"}"#)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestData>("/test/headers")
        .with_header("X-Custom-Header", "custom-value")
        .with_tenant("tranquility");

    let response = request.send().await.expect("Request should succeed");

    assert_eq!(response.data.message, "headers received");

    mock.assert_async().await;
}

/// Tests request with authorization header.
///
/// Verifies that when an access token is provided, the Authorization
/// header is properly set with the Bearer token. This also mocks the
/// JWK endpoint for token validation.
///
/// Expected: Authorization header is included with Bearer token
#[tokio::test]
async fn test_request_with_authorization() {
    let (client, mut server) = integration_test_setup().await;

    // Mock JWK endpoint for token validation
    let _jwk_mock = get_jwk_success_response(&mut server, 1);

    // Create a valid mock token
    let token = create_mock_token(false);
    let access_token = token.access_token().secret();

    let mock = server
        .mock("GET", "/test/auth")
        .match_header("authorization", format!("Bearer {}", access_token).as_str())
        .with_status(200)
        .with_body(r#"{"message": "authenticated"}"#)
        .create_async()
        .await;

    let request = client
        .esi()
        .new_request::<TestData>("/test/auth")
        .with_access_token(access_token);

    let response = request.send().await.expect("Request should succeed");

    assert_eq!(response.data.message, "authenticated");

    mock.assert_async().await;
}

/// Tests 404 error response handling.
///
/// Verifies that the client properly handles 404 Not Found responses
/// by returning an error with the appropriate status and message.
///
/// Expected: Error with status 404 and error message
#[tokio::test]
async fn test_404_error_response() {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/notfound")
        .with_status(404)
        .with_body(r#"{"error": "Resource not found"}"#)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/notfound");

    let result = request.send().await;

    assert!(result.is_err());
    let err = result.unwrap_err();

    // Check if it's an ESI response error
    if let eve_esi::Error::EsiResponseError(esi_err) = err {
        assert_eq!(esi_err.status, 404);
        assert!(esi_err.message.contains("Resource not found"));
    } else {
        panic!("Expected EsiResponseError, got: {:?}", err);
    }

    mock.assert_async().await;
}

/// Tests 500 server error response handling.
///
/// Verifies that the client properly handles 500 Internal Server Error
/// responses by returning an error with the appropriate status.
///
/// Expected: Error with status 500
#[tokio::test]
async fn test_500_server_error_response() {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/error")
        .with_status(500)
        .with_body(r#"{"error": "Internal server error"}"#)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/error");

    let result = request.send().await;

    assert!(result.is_err());
    let err = result.unwrap_err();

    if let eve_esi::Error::EsiResponseError(esi_err) = err {
        assert_eq!(esi_err.status, 500);
        assert!(esi_err.message.contains("Internal server error"));
    } else {
        panic!("Expected EsiResponseError, got: {:?}", err);
    }

    mock.assert_async().await;
}

/// Tests error response includes cache headers.
///
/// Verifies that even when an error response is returned, the cache
/// headers are still properly extracted and available in the error.
///
/// Expected: Error contains cache headers
#[tokio::test]
async fn test_error_response_includes_cache_headers() {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/error")
        .with_status(404)
        .with_header("cache-control", "no-cache")
        .with_header("etag", "\"error-etag\"")
        .with_body(r#"{"error": "Not found"}"#)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/error");

    let result = request.send().await;

    assert!(result.is_err());
    if let eve_esi::Error::EsiResponseError(esi_err) = result.unwrap_err() {
        assert_eq!(esi_err.cache.cache_control, "no-cache");
        assert_eq!(esi_err.cache.etag, "\"error-etag\"");
    } else {
        panic!("Expected EsiResponseError");
    }

    mock.assert_async().await;
}

/// Tests deserializing array response.
///
/// Verifies that the client can properly deserialize responses
/// containing arrays of data.
///
/// Expected: Array data is correctly deserialized
#[tokio::test]
async fn test_deserialize_array_response() {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/array")
        .with_status(200)
        .with_body(r#"{"items": ["item1", "item2", "item3"]}"#)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestArray>("/test/array");

    let response = request.send().await.expect("Request should succeed");

    assert_eq!(response.data.items.len(), 3);
    assert_eq!(response.data.items[0], "item1");
    assert_eq!(response.data.items[2], "item3");

    mock.assert_async().await;
}

/// Tests that cache headers are extracted from successful responses.
///
/// Verifies that ETag, Cache-Control, and Last-Modified headers are
/// properly extracted and included in the EsiResponse.
///
/// Expected: Cache headers are present in EsiResponse
#[tokio::test]
async fn test_response_includes_cache_headers() {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/cache")
        .with_status(200)
        .with_header("etag", "\"test-etag\"")
        .with_header("cache-control", "public, max-age=300")
        .with_header("last-modified", "Wed, 21 Oct 2015 07:28:00 GMT")
        .with_body(r#"{"message": "cached"}"#)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/cache");

    let response = request.send().await.expect("Request should succeed");

    assert_eq!(response.cache.etag, "\"test-etag\"");
    assert_eq!(response.cache.cache_control, "public, max-age=300");
    assert_eq!(response.data.message, "cached");

    mock.assert_async().await;
}

/// Tests that rate limit headers are extracted when present.
///
/// Verifies that X-ESI-Error-Limit headers are properly extracted
/// and included in the EsiResponse when the ESI server provides them.
///
/// Expected: Rate limit headers are present in EsiResponse
#[tokio::test]
async fn test_response_includes_rate_limit_headers() {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/ratelimit")
        .with_status(200)
        .with_header("x-esi-error-limit-group", "global")
        .with_header("x-esi-error-limit-limit", "100")
        .with_header("x-esi-error-limit-remain", "95")
        .with_header("x-esi-error-limit-used", "5")
        .with_body(r#"{"message": "rate limited"}"#)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/ratelimit");

    let response = request.send().await.expect("Request should succeed");

    assert!(response.rate_limit.is_some());
    let rate_limit = response.rate_limit.unwrap();
    assert_eq!(rate_limit.group, "global");
    assert_eq!(rate_limit.limit, "100");
    assert_eq!(rate_limit.remaining, 95);
    assert_eq!(rate_limit.used, 5);

    mock.assert_async().await;
}

/// Tests deserialization failure handling.
///
/// Verifies that when the response body cannot be deserialized into the
/// expected struct, an appropriate error is returned.
///
/// Expected: Error due to deserialization failure
#[tokio::test]
async fn test_deserialization_failure() {
    let (client, mut server) = integration_test_setup().await;

    // Return a response that doesn't match TestData structure
    let mock = server
        .mock("GET", "/test/bad-format")
        .with_status(200)
        .with_body(r#"{"wrong_field": "value", "another": 123}"#)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/bad-format");

    let result = request.send().await;

    assert!(result.is_err());
    // Should be a serde_json error
    if let Err(eve_esi::Error::SerdeJsonError(_)) = result {
        // Expected error type
    } else {
        panic!("Expected SerdeJsonError, got: {:?}", result);
    }

    mock.assert_async().await;
}

/// Tests URL parse failure handling.
///
/// Verifies that when an invalid URL is constructed, the request fails
/// with a UrlParseError before attempting to send the HTTP request.
///
/// Expected: Error due to URL parsing failure
#[tokio::test]
async fn test_url_parse_failure() {
    // Create a client with an invalid base URL that will cause URL parse errors
    let config = eve_esi::Config::builder()
        .esi_url("ht!tp://invalid url with spaces")
        .build()
        .expect("Failed to build Config");

    let client = eve_esi::Client::builder()
        .user_agent("MyApp/1.0")
        .config(config)
        .build()
        .expect("Failed to build Client");

    // Create a request - the URL construction will warn but not fail
    let request = client.esi().new_request::<TestData>("/test/endpoint");

    // The actual error should occur when we try to send the request
    let result = request.send().await;

    assert!(result.is_err());
    // Should be a URL parse error
    if let Err(eve_esi::Error::UrlParseError(_)) = result {
        // Expected error type
    } else {
        panic!("Expected UrlParseError, got: {:?}", result);
    }
}

/// Tests network/connection error handling in execute_request.
///
/// Verifies that when a network error occurs (connection refused, timeout, etc.),
/// the request fails with a ReqwestError.
///
/// Expected: Error due to connection failure
#[tokio::test]
async fn test_network_error() {
    // Create a client pointing to a non-existent host/port to trigger connection error
    let config = eve_esi::Config::builder()
        .esi_url("http://localhost:1") // Port 1 is typically not used and will refuse connection
        .build()
        .expect("Failed to build Config");

    let client = eve_esi::Client::builder()
        .user_agent("MyApp/1.0")
        .config(config)
        .build()
        .expect("Failed to build Client");

    let request = client.esi().new_request::<TestData>("/test/endpoint");

    let result = request.send().await;

    assert!(result.is_err());
    // Should be a reqwest error (connection refused or similar)
    if let Err(eve_esi::Error::ReqwestError(_)) = result {
        // Expected error type
    } else {
        panic!("Expected ReqwestError, got: {:?}", result);
    }
}
