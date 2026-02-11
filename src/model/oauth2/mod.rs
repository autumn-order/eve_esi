//! # EVE Online OAuth2 Models
//!
//! Data structures and types for EVE Online OAuth2-related endpoints.
//!
//! For usage of OAuth2 in the `eve_esi` crate, please see the [`crate::oauth2`]
//! module documentation.

pub mod jwt_claims;
pub mod jwt_key;
pub mod login;

pub use jwt_claims::EveJwtClaims;
pub use jwt_key::{EveJwtKey, EveJwtKeys};
pub use login::AuthenticationData;
