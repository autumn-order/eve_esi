//! Methods for OAuth2 authentication with EVE Online SSO
//!
//! This module provides methods for initiating and managing the EVE Online OAuth2 authentication process
//! It includes functionality for generating login URLs to initiate the authentication process and building scopes for authorization.
//!
//! # References
//! - [EVE SSO Documentation](https://developers.eveonline.com/docs/services/sso/)
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
//!     .unwrap();
//!
//! println!("Login URL: {}", auth_data.login_url);
//! ```

pub mod auth;
pub mod scope;

pub use scope::ScopeBuilder;
