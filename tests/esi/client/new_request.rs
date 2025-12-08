//! Integration tests for the `new_request` method.
//!
//! Tests that the `new_request` convenience method properly creates
//! EsiRequest instances with the correct endpoint URL.

use crate::util::integration_test_setup;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct TestData {
    message: String,
}

/// Tests that new_request creates a valid EsiRequest.
///
/// Verifies that the new_request method on EsiApi properly creates
/// an EsiRequest with the correct endpoint URL and that the request
/// can be successfully executed.
///
/// Expected: EsiRequest is created with proper endpoint and executes successfully
#[tokio::test]
async fn test_new_request_creates_valid_request() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/convenience")
        .with_status(200)
        .with_body(r#"{"message": "success"}"#)
        .create_async()
        .await;

    // Use the convenience method
    let request = client.esi().new_request::<TestData>("/test/convenience");

    let response = request.send().await?;

    assert_eq!(response.data.message, "success");

    mock.assert_async().await;

    Ok(())
}

/// Tests that new_request works with leading slash.
///
/// Verifies that endpoints can be specified with a leading slash.
///
/// Expected: Request succeeds with leading slash in endpoint
#[tokio::test]
async fn test_new_request_with_leading_slash() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/leading")
        .with_status(200)
        .with_body(r#"{"message": "with slash"}"#)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("/test/leading");
    let response = request.send().await?;

    assert_eq!(response.data.message, "with slash");

    mock.assert_async().await;

    Ok(())
}

/// Tests that new_request works without leading slash.
///
/// Verifies that endpoints can be specified without a leading slash.
///
/// Expected: Request succeeds without leading slash in endpoint
#[tokio::test]
async fn test_new_request_without_leading_slash() -> Result<(), eve_esi::Error> {
    let (client, mut server) = integration_test_setup().await;

    let mock = server
        .mock("GET", "/test/noslash")
        .with_status(200)
        .with_body(r#"{"message": "no slash"}"#)
        .create_async()
        .await;

    let request = client.esi().new_request::<TestData>("test/noslash");
    let response = request.send().await?;

    assert_eq!(response.data.message, "no slash");

    mock.assert_async().await;

    Ok(())
}
