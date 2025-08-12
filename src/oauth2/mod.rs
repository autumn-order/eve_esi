//! Methods for OAuth2 authentication with EVE Online SSO
//!
//! This module provides methods for initiating and managing the EVE Online OAuth2 authentication process
//! It includes functionality for generating login URLs to initiate the authentication process and building scopes for authorization.
//!
//! # References
//! - [EVE SSO Documentation](https://developers.eveonline.com/docs/services/sso/)
//!
//! # Modules
//!
//! - [`auth`](crate::endpoints::auth)
//! - [`scope`](crate::endpoints::scope)
//!
//! # Example
//! ```
//! let esi_client = eve_esi::EsiClient::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com)")
//!     .client_id("client_id")
//!     .client_secret("client_secret")
//!     .callback_url("http://localhost:8080/callback")
//!     .build()
//!     .expect("Failed to build EsiClient");
//!
//! let scopes = eve_esi::oauth2::ScopeBuilder::new()
//!     .public_data()
//!     .build();
//! let auth_data = esi_client
//!     .initiate_oauth_login(scopes)
//!     .expect("Failed to initiate OAuth login");
//!
//! println!("Login URL: {}", auth_data.login_url);
//! ```

pub mod auth;
pub(crate) mod client;
pub mod scope;

pub use scope::ScopeBuilder;
