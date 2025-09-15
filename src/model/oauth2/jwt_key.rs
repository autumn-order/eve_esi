//! # EVE Online OAuth2 JWT Key Models
//!
//! Provides the [`EveJwtKeys`] struct & [`EveJwtKey`] enum to represent the JWT keys
//! used to validate tokens returned from EVE Online's OAuth2 API.
//!
//! For usage of OAuth2 in the `eve_esi` crate, please see the [`crate::oauth2`]
//! module documentation.
//!
//! ## EVE Online OAuth2 Documentation
//! - <https://developers.eveonline.com/docs/services/sso/>
//!
//! ## Models:
//! - [`EveJwtKeys`]: Represents the EVE Online JSON Web Token (JWT) keys used for validating authentication tokens
//! - [`EveJwtKey`]: Represents the types of EVE Online JSON Web Token (JWT) keys used for token validation

use serde::{Deserialize, Serialize};

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

/// Represents the types of EVE Online JSON Web Token (JWT) keys used for token validation
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
