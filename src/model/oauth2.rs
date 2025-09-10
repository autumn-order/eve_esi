//! Data structures and types for representing oauth2 authentication data in EVE Online.
//!
//! This module defines the `AuthenticationData` struct, which models the authentication data needed to begin an OAuth2 authentication flow.
//!
//! See [EVE SSO documentation](https://developers.eveonline.com/docs/services/sso/)

use serde::{Deserialize, Serialize};

/// Represents the data needed to begin an OAuth2 authentication flow
///
/// This struct contains the URL where users should be redirected to login
/// and a random state parameter for CSRF protection.
///
/// # Documentation
/// See [EVE SSO documentation](https://developers.eveonline.com/docs/services/sso/)
/// for details related to the oauth2 authentication flow.
///
/// # Fields
/// - `login_url`: The URL where users should be redirected to login
/// - `state`: A random state parameter used to prevent CSRF attacks
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationData {
    /// The URL where users should be redirected to login
    pub login_url: String,
    /// A random state parameter used to prevent CSRF attacks
    pub state: String,
}

/// Represents the EVE Online JSON Web Token (JWT) keys used for validating authentication tokens
///
/// This struct contains a collection of JWT keys provided by EVE Online's SSO service,
/// used for verifying the authenticity of access tokens.
///
/// # Documentation
/// See [EVE SSO documentation](https://developers.eveonline.com/docs/services/sso/)
/// for details about JWT verification.
///
/// # Fields
/// - `skip_unresolved_json_web_keys`: Flag to determine whether to skip unresolved JWT keys
/// - `keys`: A collection of EVE Online JWT keys used for token validation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EveJwtKeys {
    /// Flag to determine whether to skip unresolved JWT keys
    #[serde(rename = "SkipUnresolvedJsonWebKeys")]
    pub skip_unresolved_json_web_keys: bool,
    /// A collection of EVE Online JWT keys used for token validation
    pub keys: Vec<EveJwtKey>,
}

/// Represents a single EVE Online JSON Web Token (JWT) key used for token validation
///
/// This enum represents different types of cryptographic keys used by EVE Online's SSO service
/// for signing JWTs. It supports both RSA (RS256) and Elliptic Curve (ES256) algorithms.
///
/// # Documentation
/// See [EVE SSO documentation](https://developers.eveonline.com/docs/services/sso/)
/// for details about JWT verification and key formats.
///
/// # Variants
/// - `RS256`: RSA SHA-256 signature algorithm key parameters
/// - `ES256`: Elliptic Curve P-256 signature algorithm key parameters
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "alg")]
pub enum EveJwtKey {
    /// RSA SHA-256 signature algorithm key parameters
    ///
    /// # Fields
    /// - `e`: The RSA public exponent as a Base64URL-encoded value
    /// - `kid`: Key ID used to match a specific key
    /// - `kty`: Key type (typically "RSA")
    /// - `n`: The RSA modulus as a Base64URL-encoded value
    /// - `use`: The intended use of the key (typically "sig" for signature)
    RS256 {
        /// The RSA public exponent as a Base64URL-encoded value
        e: String,
        /// Key ID used to match a specific key
        kid: String,
        /// Key type (typically "RSA")
        kty: String,
        /// The RSA modulus as a Base64URL-encoded value
        n: String,
        /// The intended use of the key (typically "sig" for signature)
        r#use: String,
    },

    /// Elliptic Curve P-256 signature algorithm key parameters
    ///
    /// # Fields
    /// - `crv`: The curve type (typically "P-256")
    /// - `kid`: Key ID used to match a specific key
    /// - `kty`: Key type (typically "EC" for Elliptic Curve)
    /// - `use`: The intended use of the key (typically "sig" for signature)
    /// - `x`: The x coordinate of the elliptic curve point as a Base64URL-encoded value
    /// - `y`: The y coordinate of the elliptic curve point as a Base64URL-encoded value
    ES256 {
        /// The curve type (typically "P-256")
        crv: String,
        /// Key ID used to match a specific key
        kid: String,
        /// Key type (typically "EC" for Elliptic Curve)
        kty: String,
        /// The intended use of the key (typically "sig" for signature)
        r#use: String,
        /// The x coordinate of the elliptic curve point as a Base64URL-encoded value
        x: String,
        /// The y coordinate of the elliptic curve point as a Base64URL-encoded value
        y: String,
    },
}

/// Represents the claims in an EVE Online JWT access token
///
/// This struct contains the standard JWT claims as well as EVE Online specific
/// claims that are used to identify the character and other information.
///
/// # Documentation
/// See [EVE SSO documentation](https://developers.eveonline.com/docs/services/sso/#validating-jwt-tokens)
/// for details about JWT claims.
#[derive(Debug, Serialize, Deserialize)]
pub struct EveJwtClaims {
    /// The issuer of the JWT token (EVE Online's login service URL)
    pub iss: String,
    /// ID for the authenticated user (Example: "CHARACTER:EVE:2114794365")
    pub sub: String,
    /// Audience the JWT token is intended for (your client_id, EVE Online)
    pub aud: Vec<String>,
    /// JWT token ID, a unique identifier for this specific token
    pub jti: String,
    /// Key ID identifying which key was used to sign this JWT
    pub kid: String,
    /// The EVE Online server the key is for (tranquility)
    pub tenant: String,
    /// The region from which the token was issued (world)
    pub region: String,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issued at time (Unix timestamp)
    pub iat: i64,
    /// The scopes granted by this token
    pub scp: Vec<String>,
    /// The character's name
    pub name: String,
    /// The character's ID
    pub owner: String,
    /// The character's organization (corporation/alliance) ID
    pub azp: String,
}
