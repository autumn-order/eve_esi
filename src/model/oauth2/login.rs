//! # EVE Online OAuth2 Login Model
//!
//! Provides the [`AuthenticationData`] struct to represent the login_url & state
//! string returned from the [`crate::oauth2::OAuth2Endpoints::login_url`] method used to
//! initiate the SSO (single sign-on) login flow with EVE Online.
//!
//! For usage of OAuth2 in the `eve_esi` crate, please see the [`crate::oauth2`]
//! module documentation.
//!
//! ## EVE Online OAuth2 Documentation
//! - <https://developers.eveonline.com/docs/services/sso/>

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationData {
    /// The URL where users should be redirected to login
    pub login_url: String,
    /// A random state parameter used to prevent CSRF attacks
    pub state: String,
}
