use std::time::{SystemTime, UNIX_EPOCH};

use eve_esi::model::oauth2::EveJwtClaims;
use oauth2::TokenResponse;

use crate::constant::TEST_CLIENT_ID;
use crate::oauth2::util::jwk_response::get_jwk_success_response;
use crate::oauth2::util::jwt::{create_mock_token, create_mock_token_with_claims, RSA_KEY_ID};
use crate::util::setup;

/// Ensures that a new token does not return as expired
///
/// Uses a mock token & mock JWT keys created with a test pair of RSA public & private
/// keys. The function under test checks the expiration by validating the mock token to get the
/// claims first using the mock JWT keys.
///
/// # Test Setup
/// - Create an ESI Client configured with OAuth2 and a mock server
/// - Create a mock JWT key response the Client will fetch for the JWT key cache
/// - Create a mock token representing what we would get using the `get_token` method
///
/// # Assertions
/// - Assert mock JWT keys were fetched and cached for validation
/// - Assert token expiry check did not error
/// - Assert token is not expired
#[tokio::test]
pub async fn test_check_token_expiration_not_expired() {
    // Create Client configured with OAuth2 & mock server
    let (client, mut mock_server) = setup().await;

    // Create a mock JWT key response the Client will fetch when validating token prior
    // to checking expiration
    let mock = get_jwk_success_response(&mut mock_server, 1);

    // Create a mock token representing what we would get using the `get_token` method
    let token = create_mock_token(false);

    // Check token expiry
    let result = client
        .oauth2()
        .check_token_expiration(&token.access_token().secret().to_string())
        .await;

    // Assert mock JWT keys were fetched for validation
    mock.assert();

    // Assert token expiry check did not error
    assert!(
        result.is_ok(),
        "Failed to check token expiry due to error: {:#?}",
        result
    );

    // Assert token is not expired
    let token_expired = result.unwrap();
    assert_eq!(token_expired, false)
}

/// Ensures that an expired token returns as expired.
///
/// Uses a mock token & mock JWT keys created with a test pair of RSA public & private
/// keys. The function under test checks the expiration by validating the mock token to get the
/// claims first using the mock JWT keys.
///
/// # Test Setup
/// - Create an ESI Client configured with OAuth2 and a mock server
/// - Create a mock JWT key response the Client will fetch for the JWT key cache
/// - Create a mock token representing what we would get using the `get_token` method
///
/// # Assertions
/// - Assert mock JWT keys were fetched and cached for validation
/// - Assert token expiry check did not error
/// - Assert token is expired
#[tokio::test]
async fn test_check_token_expiration_expired() {
    // Create Client configured with OAuth2 & mock server
    let (client, mut mock_server) = setup().await;

    // Create a mock JWT key response the Client will fetch when validating token prior
    // to checking expiration
    let mock = get_jwk_success_response(&mut mock_server, 1);

    // Create a mock token representing what we would get using the `get_token` method
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let claims = EveJwtClaims {
        iss: "https://login.eveonline.com".to_string(),
        sub: "CHARACTER:EVE:123456789".to_string(),
        aud: vec![TEST_CLIENT_ID.to_string(), "EVE Online".to_string()],
        jti: "abc123def456".to_string(),
        kid: RSA_KEY_ID.to_string(),
        tenant: "tranquility".to_string(),
        region: "world".to_string(),
        exp: now - 60,  // Expired 1 minute ago
        iat: now - 960, // Created 16 minutes ago
        scp: vec![
            "publicData".to_string(),
            "esi-characters.read_agents_research.v1".to_string(),
        ],
        name: "Test Character".to_string(),
        owner: "123456789".to_string(),
        azp: TEST_CLIENT_ID.to_string(),
    };

    let token = create_mock_token_with_claims(false, claims);

    // Check token expiry
    let result = client
        .oauth2()
        .check_token_expiration(&token.access_token().secret().to_string())
        .await;

    // Assert mock JWT keys were fetched for validation
    mock.assert();

    // Assert token expiry check did not error
    assert!(
        result.is_ok(),
        "Failed to check token expiry due to error: {:#?}",
        result
    );

    // Assert token is expired
    let token_expired = result.unwrap();
    assert_eq!(token_expired, true)
}

/// Tests error handling when token validation fails
///
/// Ensures that error is handled correctly when token validation fails due to the
/// public key used to validate the token differs from the private key used to
/// create the token.
///
/// # Test Setup
/// - Create an ESI Client configured with OAuth2 and a mock server
/// - Create a mock JWT key response the Client will fetch for the JWT key cache
/// - Create a mock token created with a different private key than the JWT keys
///
/// # Assertions
/// - Assert mock JWT keys were fetched and cached for validation
/// - Assert token expiry check returned an error
/// - Assert error is of type OAuthError::ValidateTokenError
#[tokio::test]
async fn test_check_token_expiration_error() {
    // Create Client configured with OAuth2 & mock server
    let (client, mut mock_server) = setup().await;

    // Create a mock JWT key response the Client will fetch when validating token prior
    // to checking expiration
    let mock = get_jwk_success_response(&mut mock_server, 1);

    // Create a mock token created with a different private key than the JWT keys
    let token = create_mock_token(true);

    // Check token expiry
    let result = client
        .oauth2()
        .check_token_expiration(&token.access_token().secret().to_string())
        .await;

    // Assert mock JWT keys were fetched for validation
    mock.assert();

    // Assert token expiry check returned an error
    assert!(
        result.is_err(),
        "Expected an error checking token for expiration, instead got: {:#?}",
        result
    );

    // Assert error is of type OAuthError::ValidateTokenError
    assert!(matches!(
        result,
        Err(eve_esi::Error::OAuthError(
            eve_esi::OAuthError::ValidateTokenError(_)
        ))
    ))
}
