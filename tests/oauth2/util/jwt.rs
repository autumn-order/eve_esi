use std::fs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use eve_esi::model::oauth2::{EveJwtClaims, EveJwtKey, EveJwtKeys};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use oauth2::basic::BasicTokenType;
use oauth2::{AccessToken, EmptyExtraTokenFields, RefreshToken, StandardTokenResponse};
use openssl::rsa::Rsa;

use crate::constant::TEST_CLIENT_ID;

const RSA_KEY_ID: &str = "JWT-Signature-Key-1";

/// Create mock token keys to validate tokens
///
/// Uses a test RSA public key to create a mock of the [`EveJwtKeys`] struct used
/// to validate the token created by [`create_mock_token`].
///
/// # Returns
/// - [`EveJwtKeys`]: Struct containing an RS256 key based upon a mock public RSA key and a
///   ES256 included to mimic what ESI is expected to return but not intended to be used by
///   this crate.
pub fn create_mock_token_keys() -> EveJwtKeys {
    // Load the public key PEM file
    let public_key_pem = fs::read("tests/oauth2/util/public_test_rsa_key.pem").unwrap();

    // Extract RSA components
    let rsa = Rsa::public_key_from_pem(&public_key_pem).unwrap();

    // Get the modulus and exponent as raw bytes
    let n_bytes = rsa.n().to_vec();
    let e_bytes = rsa.e().to_vec();

    // Base64URL encode them (no padding)
    let n = URL_SAFE_NO_PAD.encode(n_bytes);
    let e = URL_SAFE_NO_PAD.encode(e_bytes);

    EveJwtKeys {
        skip_unresolved_json_web_keys: false,
        keys: vec![
            EveJwtKey::RS256 {
                e: e,
                kid: RSA_KEY_ID.to_string(),
                kty: "RSA".to_string(),
                n: n,
                r#use: "sig".to_string(),
            },
            // Not actually used but EVE's API does return an ES256 key alongside the RS256 so it is included
            EveJwtKey::ES256 {
                crv: "P-256".to_string(),
                kid: "JWT-Signature-Key-2".to_string(),
                kty: "EC".to_string(),
                r#use: "sig".to_string(),
                x: "ITcDYJ8WVpDO4QtZ169xXUt7GB1Y6-oMKIwJ3nK1tFU".to_string(),
                y: "ZAJr0f4V2Eu7xBgLMgQBdJ2DZ2mp8JykOhX4XgU_UEY".to_string(),
            },
        ],
    }
}

/// Creates a mock token with mock [`EveJwtClaims`] expected from EVE servers for the purposes of testing.
///
/// Uses a test RS256 private key for the purposes of validation with the keys created by the
/// [`create_mock_token_keys`] function. Contains mock [`EveJwtClaims`] similar to what EVE servers would
/// return.
///
/// # Returns
/// - [`StandardTokenResponse`]<[`EmptyExtraTokenFields`], [`BasicTokenType`]>: A token which
///   contains [`EveJwtClaims`] and is encoded with a test RS256 private key for testing validation.
pub fn create_mock_token() -> StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType> {
    // Create header with algorithm and key id
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(RSA_KEY_ID.to_string());

    // Create JWT claims matching what EVE Online would return
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
        exp: now + 900, // Valid for 15 minutes
        iat: now,
        scp: Some("publicData".to_string()),
        name: "Test Character".to_string(),
        owner: "123456789".to_string(),
        azp: TEST_CLIENT_ID.to_string(),
    };

    // Encode the access token
    let private_key = include_bytes!("./private_test_rsa_key.pem");

    let encoding_key =
        EncodingKey::from_rsa_pem(private_key).expect("Failed to create encoding key");
    let access_token_secret =
        encode(&header, &claims, &encoding_key).expect("Failed to encode token");

    // Create the token components
    let access_token = AccessToken::new(access_token_secret);
    let token_type = BasicTokenType::Bearer;
    let expires_in = Some(&Duration::from_secs(3600)); // 1 hour
    let refresh_token = Some(RefreshToken::new("mock_refresh_token_value".to_string()));

    // Create empty extra fields
    let extra_fields = EmptyExtraTokenFields {};

    // Create the token response
    let mut token = StandardTokenResponse::new(access_token, token_type, extra_fields);

    // Set optional fields
    token.set_expires_in(expires_in);
    token.set_refresh_token(refresh_token);

    token
}
