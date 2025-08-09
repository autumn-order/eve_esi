use serde::{Deserialize, Serialize};

/// Represents the data needed to begin an OAuth2 authentication flow
///
/// This struct contains the URL where users should be redirected to login
/// and a random state parameter for CSRF protection.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationData {
    /// The URL where users should be redirected to login
    pub login_url: String,
    /// A random state parameter used to prevent CSRF attacks
    pub state: String,
}
