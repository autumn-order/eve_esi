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
