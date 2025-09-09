use std::time::Duration;

use eve_esi::model::oauth2::{EveJwtKey, EveJwtKeys};
use oauth2::TokenResponse;

use crate::constant::TEST_CLIENT_ID;
use crate::oauth2::util::jwk_response::{
    get_jwk_internal_server_error_response, get_jwk_success_response,
};
use crate::oauth2::util::jwt::{create_mock_token, create_mock_token_keys, RSA_KEY_ID};
use crate::util::setup;

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
    let token = create_mock_token(false);

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
/// `validate_token` will call the `get_jwt_keys` function internally to get keys from cache or
/// fetch them if cache is empty. This tests error handling when the attempt to fetch
/// the keys for validation fails.
///
/// If the keys are older than 60 seconds, they will be refreshed and a validation will be attempted again,
/// in this instance they are fresh keys so no retry will be made here.
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
    let token = create_mock_token(false);

    // Validate the token
    let result = client
        .oauth2()
        .validate_token(token.access_token().secret().to_string())
        .await;

    // Assert JWT key fetch was attempted
    mock.assert();

    // Assert token validation resulted in an error
    assert!(result.is_err(), "Expected error, got: {:#?}", result);

    // Assert error is reqwest error of type OAuthError::JwtKeyRefreshFailure
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
/// If the keys are older than 60 seconds, they will be refreshed and validation will be attempted again,
/// in this instance they are fresh keys so no retry will be made here.
///
/// # Test Setup
/// - Create an ESI Client configured with OAuth2 and a mock server
/// - Create a mock EveJwtKeys struct that only contains an ES256 key
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

    // Create a mock EveJwtKeys struct that only contains an ES256 key
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
    let token = create_mock_token(false);

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

/// Tests validation failure due to an issue with the decoding key
///
/// `validate_token` uses a decoding key prior to decoding the token, if the RS256
/// modulus is malformed then an error will be returned as result. Error will be of type
/// [`OAuthError::ValidateTokenError`], more precisely it will be of the type
/// [`jsonwebtoken::errors::ErrorKind::InvalidSignature`].
///
/// Generally applications would just return an internal server error when
/// [`OAuthError::ValidateTokenError`] is returned and refresh the JWT key cache in this case.
///
/// If the keys are older than 60 seconds, they will be refreshed and validation will be attempted again,
/// in this instance they are fresh keys so no retry will be made here.
///
/// # Test Setup
/// - Create an ESI Client configured with OAuth2 and a mock server
/// - Create a mock EveJwtKeys struct that only contains a malformed RS256 key (empty modulus)
/// - Create a mock JWT key response returning the malformed key
/// - Create a mock token representing what we would get using the `get_token` method
///
/// # Assertions
/// - Assert mock JWT keys were fetched for validation
/// - Assert token validation resulted in an error
/// - Assert error is of type base64::DecodeError::InvalidByte
#[tokio::test]
async fn test_validate_token_decoding_key_error() {
    // Create Client configured with OAuth2 & mock server
    let (client, mut mock_server) = setup().await;

    // Create a mock EveJwtKeys struct that only contains a malformed RS256 key (invalid modulus)
    let malformed_rs256 = EveJwtKeys {
        skip_unresolved_json_web_keys: false,
        keys: [EveJwtKey::RS256 {
            e: "ABCD".to_string(),
            kid: RSA_KEY_ID.to_string(),
            kty: "RSA".to_string(),
            n: "invalid base64!@#$%^&*()".to_string(), // Invalid base64 that will cause decode to fail
            r#use: "sig".to_string(),
        }]
        .to_vec(),
    };

    // Create a mock JWT key response returning the malformed key
    let mock = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&malformed_rs256).unwrap())
        .expect(1)
        .create();

    // Create a mock token representing what we would get using the `get_token` method
    let token = create_mock_token(false);

    // Validate the token
    let result = client
        .oauth2()
        .validate_token(token.access_token().secret().to_string())
        .await;

    // Assert mock JWT keys were fetched for validation
    mock.assert();

    // Assert token validation resulted in an error
    assert!(result.is_err(), "Expected error, got: {:#?}", result);

    // Assert error is of type base64::DecodeError::InvalidByte
    match result {
        Err(eve_esi::Error::OAuthError(eve_esi::OAuthError::ValidateTokenError(err))) => {
            assert_eq!(
                err.kind(),
                &jsonwebtoken::errors::ErrorKind::Base64(base64::DecodeError::InvalidByte(7, 32))
            )
        }
        err => panic!(
            "Expected OAuthError::ValidateTokenError, got different error type: {:#?}",
            err
        ),
    }
}

/// Tests an issue validating the key due to a different private key
///
/// If the RSA private key used to sign the token is rotated, either being different than the
/// corresponding public key stored in cache or fetched from EVE Online's OAuth2 API, a validation
/// error will occur.
///
/// If the keys are older than 60 seconds, they will be refreshed and validation will be attempted again,
/// in this instance they are fresh keys so no retry will be made here.
///
/// # Test Setup
/// - Create an ESI Client configured with OAuth2 and a mock server
/// - Create a mock JWT key response the Client will fetch for the JWT key cache
/// - Create a mock token but with a different private key which will cause a validation error
///
/// # Assertions
/// - Assert mock JWT keys were fetched for validation
/// - Assert token validation resulted in an error
/// - Assert error is of type ErrorKind::InvalidSignature
#[tokio::test]
async fn test_validate_token_validation_error() {
    // Create Client configured with OAuth2 & mock server
    let (client, mut mock_server) = setup().await;

    // Create a mock JWT key response the Client will fetch for the JWT key cache
    let mock = get_jwk_success_response(&mut mock_server, 1);

    // Create a mock token but with a different private key which will cause a validation error
    let token = create_mock_token(true);

    // Validate the token
    let result = client
        .oauth2()
        .validate_token(token.access_token().secret().to_string())
        .await;

    // Assert mock JWT keys were fetched for validation
    mock.assert();

    // Assert token validation resulted in an error
    assert!(result.is_err(), "Expected error, got: {:#?}", result);

    // Assert error is of type ErrorKind::InvalidSignature
    match result {
        Err(eve_esi::Error::OAuthError(eve_esi::OAuthError::ValidateTokenError(err))) => {
            assert_eq!(
                err.kind(),
                &jsonwebtoken::errors::ErrorKind::InvalidSignature,
                "Expected ErrorKind::InvalidSignature, got different type: {:#?}",
                err
            )
        }
        err => panic!(
            "Expected OAuthError::ValidateTokenError, got different error type: {:#?}",
            err
        ),
    }
}

/// JWT keys stored in cache have been rotated out, a retry will be needed to pass validation
///
/// JWT key cache does have keys for validation but they are outdated due to EVE Online having rotated their keys.
/// Validation will fail prompting a refresh and will succeed on retry.
///
/// - The first validation attempt will fail due to the JWT key cache not being expired but holding keys which are
///   out of date due to having been rotated.
/// - Usually the cache would not be cleared since the keys were fetched within the 60 second refresh cooldown
///   period (default), in this instance we've disabled the cooldown to clear cache and refresh immediately.
/// - Each validation attempt calls the `get_jwt_keys` method internally to get the JWT keys from cache
///   or refresh if the cache is empty.
/// - The cache will be refreshed with the latest keys & the 2nd validation attempt will be successful.
///
/// # Test Setup
/// - Create Client configured with no refresh cooldown to immediately clear & refresh cache on validation failure
/// - Create a mock JWT key response with JWT keys that will fail validation
/// - Create a mock JWT key response with the correct JWT keys
/// - Pre-fill the cache with the first set of keys that will fail validation using `get_jwt_keys`
/// - Create a mock token representing what we would get using the `get_token` method
///
/// # Assertions
/// - Assert cache was pre-filled successfully
/// - Assert 1 fetch attempt for each mock JWT keys was made pre-fill & validation
/// - Assert token validation was successful
#[tokio::test]
async fn test_validate_token_key_rotation() {
    // Create Client configured with no refresh cooldown to immediately clear & refresh cache on validation failure
    let (_, mut mock_server) = setup().await;

    let config = eve_esi::Config::builder()
        // Set endpoint to mock server
        .jwk_url(&format!("{}/oauth/jwks", mock_server.url()))
        // Disable refresh cooldown so cache always clears
        .jwk_refresh_cooldown(Duration::from_secs(0))
        .build()
        .expect("Failed to build Config");

    let client = eve_esi::Client::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .client_id(TEST_CLIENT_ID)
        .client_secret("client_secret")
        .callback_url("http://localhost:8000/callback")
        .config(config)
        .build()
        .expect("Failed to build Client");

    // Create a mock JWT key response with JWT keys that will fail validation
    let mock_keys = create_mock_token_keys(true);

    let mock_1 = mock_server
        .mock("GET", "/oauth/jwks")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&mock_keys).unwrap())
        .expect(1)
        .create();

    // Create a mock JWT key response with the correct JWT keys
    let mock_2 = get_jwk_success_response(&mut mock_server, 1);

    // Pre-fill the cache with the first set of keys using `get_jwt_keys`
    let result = client.oauth2().jwk().get_jwt_keys().await;

    // Assert cache was pre-filled successfully
    assert!(result.is_ok());

    // Create a mock token representing what we would get using the `get_token` method
    let token = create_mock_token(false);

    // Validate token
    let result = client
        .oauth2()
        .validate_token(token.access_token().secret().to_string())
        .await;

    // Assert 1 fetch attempt for each mock JWT keys was made pre-fill & validation
    mock_1.assert();
    mock_2.assert();

    // Assert token validation was successful
    assert!(result.is_ok(), "Token validation failed: {:#?}", result);
}
