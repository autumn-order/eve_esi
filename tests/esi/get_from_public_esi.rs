use eve_esi::EsiClient;
use mockito::Server;
use serde_json::Value;

/// Tests the successful retrieval of data from a public ESI endpoint.
///
/// # Test Setup
/// - Creates a mock server to simulate an ESI endpoint
/// - Configures a mock response with expected JSON data
/// - Initializes an ESI client with appropriate user agent
/// - Makes a request to the mocked endpoint
///
/// # Assertions
/// - Verifies that the mock was called exactly as expected
/// - Confirms that the returned JSON data matches the expected response
#[tokio::test]
async fn test_get_from_public_esi() {
    // Setup mock server
    let mut mock_server = Server::new_async().await;
    let mock_server_url = mock_server.url();

    // Create mock response
    let mock = mock_server
        .mock("GET", "/test")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Hello, world!"}"#)
        .create();

    // Initialize ESI client
    let esi_client = EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .build()
        .expect("Failed to build EsiClient");
    let url = &format!("{}/test", mock_server_url);

    // Call the get_from_public_esi method
    let result: Value = esi_client.get_from_public_esi(url).await.unwrap();

    // Assert that the mock was called
    mock.assert();

    // Verify response data
    assert_eq!(result["message"], "Hello, world!");
}
