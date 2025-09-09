use crate::oauth2::util::jwk_response::{
    get_jwk_internal_server_error_response, get_jwk_success_response,
};
use crate::oauth2::util::jwt::create_mock_token;
use crate::util::setup;
use eve_esi::model::oauth2::{EveJwtKey, EveJwtKeys};
use oauth2::TokenResponse;

/// Tests successful validation of a JWT token
///
/// Uses a mock token & mock JWT keys created with a test pair of RSA public & private
/// keys to emulate EVE API responses and how the ESI client would handle the validation.
///
/// # Test Setup
/// - Create an ESI Client configured with OAuth2 and a mock server
/// - Create a mock JWT key response the Client will fetch for the JWT key cache
/// - Create a mock token representing what we would get using the `get_token` method
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

    // Assert mock JWT keys were fetched for validation
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

/// Tests validation failure due to failure to fetch JWT keys used to validate
///
/// `validate_token` will call the `get_jwt_keys` function to get keys from cache or
/// fetch them if cache is empty. This tests error handling when the attempt to fetch
/// the keys for validation fails.
///
/// # Test Setup
/// - Create an ESI Client configured with OAuth2 and a mock server
/// - Create a mock JWT key response that will return an error
/// - Create a mock token representing what we would get using the `get_token` method
///
/// # Assertions
/// - Assert JWT key fetch was attempted
/// - Assert token validation resulted in an error
/// - Assert error is due to a ReqwestError internal server error
#[tokio::test]
async fn test_validate_token_get_jwt_key_failure() {
    // Create Client configured with OAuth2 & mock server
    let (client, mut mock_server) = setup().await;

    // Create a mock JWT key response that will return an error after 3 attempts
    let mock = get_jwk_internal_server_error_response(&mut mock_server, 3);

    // Create a mock token representing what we would get using the `get_token` method
    let token = create_mock_token();

    // Validate the token
    let result = client
        .oauth2()
        .validate_token(token.access_token().secret().to_string())
        .await;

    // Assert JWT key fetch was attempted
    mock.assert();

    // Assert token validation resulted in an error
    assert!(result.is_err(), "Expected error, got: {:#?}", result);

    // Assert error is reqwest error of type 500 internal server error
    match result {
        Err(eve_esi::Error::ReqwestError(err)) => {
            assert!(err.is_status());
            assert_eq!(
                err.status(),
                Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
            )
        }
        err => panic!(
            "Expected ReqwestError, got different error type: {:#?}",
            err
        ),
    }
}

/// Tests validation failure due to not cache having an ES256 key but not RS256
///
/// `validate_token` uses an RS256 key to validate tokens, EVE's OAuth2 API returns both
/// an RS256 & ES256. The ES256 key is not used but is included in the cache case it may
/// be needed later. In the event only the ES256 is available in the cache, a
/// OAuthError::NoValidKeyFound error would occur.
///
/// # Test Setup
/// - Create an ESI Client configured with OAuth2 and a mock server
/// - Create a mock JWT key response returning only an ES256 key
/// - Create a mock token representing what we would get using the `get_token` method
///
/// # Assertions
/// - Assert mock JWT keys were fetched for validation
/// - Assert token validation resulted in an error
/// - Assert error is OAuthError::NoValidKeyFound
#[tokio::test]
async fn test_validate_token_no_rs256_key() {
    // Create Client configured with OAuth2 & mock server
    let (client, mut mock_server) = setup().await;

    // Create a mock JWT key response that only returns an ES256 key
    let only_es256_key = EveJwtKeys {
        skip_unresolved_json_web_keys: false,
        keys: [EveJwtKey::ES256 {
            crv: "P-256".to_string(),
            kid: "JWT-Signature-Key-2".to_string(),
            kty: "EC".to_string(),
            r#use: "sig".to_string(),
            x: "ITcDYJ8WVpDO4QtZ169xXUt7GB1Y6-oMKIwJ3nK1tFU".to_string(),
            y: "ZAJr0f4V2Eu7xBgLMgQBdJ2DZ2mp8JykOhX4XgU_UEY".to_string(),
        }]
        .to_vec(),
    };

    // Create a mock JWT key response returning only an ES256 key
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&only_es256_key).unwrap())
        .expect(1)
        .create();

    // Create a mock token representing what we would get using the `get_token` method
    let token = create_mock_token();

    // Validate the token
    let result = client
        .oauth2()
        .validate_token(token.access_token().secret().to_string())
        .await;

    // Assert mock JWT keys were fetched for validation
    mock.assert();

    // Assert token validation resulted in an error
    assert!(result.is_err(), "Expected error, got: {:#?}", result);

    // Assert error is OAuthError::NoValidKeyFound
    match result {
        Err(eve_esi::Error::OAuthError(eve_esi::OAuthError::NoValidKeyFound(_))) => {}
        err => panic!(
            "Expected OAuthError::NoValidKeyFound, got different error type: {:#?}",
            err
        ),
    }
}

async fn test_validate_token_decoding_key_error() {}

async fn test_validate_token_validation_error() {}
