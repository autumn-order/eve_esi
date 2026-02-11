//! Integration tests for ESI request retry logic.
//!
//! Tests that the retry logic properly handles 5xx server errors with exponential
//! backoff while immediately returning on 4xx client errors and successful responses.

use eve_esi::Error;
use serde::Deserialize;

use crate::util::integration_test_setup;

#[derive(Deserialize, Debug, PartialEq)]
struct TestData {
    message: String,
}

/// Tests that 5xx errors trigger retry attempts up to max_retries.
///
/// Verifies that when the server returns a 500 error, the client retries
/// the request according to the configured max_retries setting.
///
/// Expected: Request is retried the configured number of times
#[tokio::test]
async fn test_retry_on_500_error() -> Result<(), Error> {
    let (client, mut server) = integration_test_setup().await;

    // Mock server to return 500 three times
    let mock = server
        .mock("GET", "/test/retry")
        .with_status(500)
        .with_body(r#"{"error": "Internal server error"}"#)
        .expect(3) // Should be called 3 times (1 initial + 2 retries)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/retry");
    let result = request.send().await;

    // Should fail after all retries
    assert!(result.is_err());
    if let Error::EsiError(esi_err) = result.unwrap_err() {
        assert_eq!(esi_err.status, 500);
    } else {
        panic!("Expected EsiError");
    }

    mock.assert_async().await;
    Ok(())
}

/// Tests that retry eventually succeeds if server recovers.
///
/// Verifies that if the server returns errors initially but then succeeds,
/// the retry logic handles it correctly.
///
/// Expected: Request succeeds after retries
#[tokio::test]
async fn test_retry_succeeds_after_failures() -> Result<(), Error> {
    let (client, mut server) = integration_test_setup().await;

    // First two attempts fail with 503, third succeeds
    let error_mock = server
        .mock("GET", "/test/recover")
        .with_status(503)
        .with_body(r#"{"error": "Service unavailable"}"#)
        .expect(2)
        .create_async()
        .await;

    let success_mock = server
        .mock("GET", "/test/recover")
        .with_status(200)
        .with_body(r#"{"message": "success"}"#)
        .expect(1)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/recover");
    let result = request.send().await?;

    assert_eq!(result.data.message, "success");

    error_mock.assert_async().await;
    success_mock.assert_async().await;
    Ok(())
}

/// Tests that 4xx errors do NOT trigger retries.
///
/// Verifies that client errors (4xx) return immediately without
/// any retry attempts.
///
/// Expected: Request fails immediately without retries
#[tokio::test]
async fn test_no_retry_on_4xx_error() -> Result<(), Error> {
    let (client, mut server) = integration_test_setup().await;

    // Mock should only be called once (no retries)
    let mock = server
        .mock("GET", "/test/notfound")
        .with_status(404)
        .with_body(r#"{"error": "Not found"}"#)
        .expect(1) // Should only be called once
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/notfound");
    let result = request.send().await;

    assert!(result.is_err());
    if let Error::EsiError(esi_err) = result.unwrap_err() {
        assert_eq!(esi_err.status, 404);
    }

    mock.assert_async().await;
    Ok(())
}

/// Tests that successful responses do NOT trigger retries.
///
/// Verifies that 2xx success responses return immediately.
///
/// Expected: Request succeeds on first attempt
#[tokio::test]
async fn test_no_retry_on_success() -> Result<(), Error> {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/success")
        .with_status(200)
        .with_body(r#"{"message": "success"}"#)
        .expect(1)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/success");
    let result = request.send().await?;

    assert_eq!(result.data.message, "success");

    mock.assert_async().await;
    Ok(())
}

/// Tests that different 5xx status codes all trigger retries.
///
/// Verifies retry logic works for 500, 502, 503, 504, etc.
///
/// Expected: All 5xx errors trigger retries
#[tokio::test]
async fn test_various_5xx_errors_retry() -> Result<(), Error> {
    for status in [500, 502, 503, 504] {
        let (client, mut server) = integration_test_setup().await;

        let endpoint = format!("/test/{}", status);
        let mock = server
            .mock("GET", endpoint.as_str())
            .with_status(status)
            .with_body(r#"{"error": "Server error"}"#)
            .expect(3) // 1 initial + 2 retries
            .create_async()
            .await;

        let request = client
            .esi()
            .new_request::<TestData>(&format!("/test/{}", status));
        let _result = request.send().await;

        mock.assert_async().await;
    }

    Ok(())
}
