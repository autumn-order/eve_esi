use oauth2::TokenResponse;

use crate::oauth2::util::jwk_response::get_jwk_success_response;
use crate::oauth2::util::jwt::create_mock_token;
use crate::util::setup;

/// Tests successful validation of a JWT token
///
/// Uses a mock token & mock JWT keys created with a test pair of RSA public & private
/// keys to emulate EVE API responses and how the ESI client would handle the validation.
///
/// # Test Setup
/// - Create a Client configured with OAuth2 and a mock server
/// - Create a mock JWT key response the Client will fetch for the JWT key cache
///
/// # Assertions
/// - Assert mock JWT keys were fetched and cached for validation
/// - Assert token validation was successful
/// - Assert character_id from token claims matches the mock claims
#[tokio::test]
pub async fn test_validate_token_success() {
    // Create Client configured with OAuth2 & mock server
    let (client, mut mock_server) = setup().await;

    // Create a mock JWT key response the Client will fetch for the JWT key cache
    let mock = get_jwk_success_response(&mut mock_server, 1);

    // Create a mock token representing what we would get using the `get_token` method
    let token = create_mock_token();

    // Validate the token
    let result = client
        .oauth2()
        .validate_token(token.access_token().secret().to_string())
        .await;

    // Assert mock JWT keys were fetched and cached for validation
    mock.assert();

    // Assert token validation was successful
    assert!(result.is_ok(), "Token validation failed: {:#?}", result);

    // Try to get character ID from token
    let claims = result.unwrap();
    let id_str = claims.sub.split(':').collect::<Vec<&str>>()[2];

    let character_id: i32 = id_str.parse().expect("Failed to parse id to i32");

    // Assert character_id from token claims matches the mock claims
    assert_eq!(character_id, 123456789)
}
