//! # EVE Online OAuth2 Models
//!
//! Data structures and types for EVE Online OAuth2-related endpoints.
//!
//! For usage of OAuth2 in the `eve_esi` crate, please see the [`crate::oauth2`]
//! module documentation.
//!
//! ## EVE Online OAuth2 Documentation
//! - <https://developers.eveonline.com/docs/services/sso/>
//!
//! ## Modules
//! - [`jwt_claims`]: Provides [`EveJwtClaims`], the body of the JWT tokens returned after login
//! - [`jwt_key`]: Provides [`EveJwtKeys`], the keys used to validate JWT tokens
//! - [`login`]: Provides [`AuthenticationData`] struct for login URL and state code for SSO (single sign-on)

pub mod jwt_claims;
pub mod jwt_key;
pub mod login;

pub use jwt_claims::EveJwtClaims;
pub use jwt_key::{EveJwtKey, EveJwtKeys};
pub use login::AuthenticationData;
