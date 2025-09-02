//! Methods for OAuth2 authentication with EVE Online SSO
//!
//! This module provides methods for initiating and managing the EVE Online OAuth2 authentication process
//! It includes functionality for generating login URLs to initiate the authentication process, building scopes for authorization, and managing tokens.
//!
//! Default settings for OAuth2 such as JWT key cache handling used to validate tokens or
//! the endpoints used for EVE OAuth2 can be overridden using the
//! [`OAuth2Config`](crate::oauth2::OAuth2Config).
//!
//! # References
//! - [EVE SSO Documentation](https://developers.eveonline.com/docs/services/sso/)
//!
//! # Modules
//!
//! - [`login`]: Methods to begin the OAuth2 login process
//! - [`token`]: Methods to retrieve, validate, & refresh OAuth2 tokens
//! - [`scope`]: Builder to create scopes to request during the login process
//! - [`config`]: Config to override default OAuth2 behavior
//! - [`jwk`]: Methods to handle JSON web keys used to validate authentication tokens
//! - [`error`]: Error enum for any OAuth2 related errors.
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
//!     .oauth2()
//!     .initiate_oauth_login(scopes)
//!     .expect("Failed to initiate OAuth login");
//!
//! println!("Login URL: {}", auth_data.login_url);
//! ```

pub mod config;
pub mod error;
pub mod jwk;
pub mod login;
pub mod oauth2;
pub mod scope;
pub mod token;

pub use config::config::OAuth2Config;
pub use oauth2::OAuth2Api;
pub use scope::ScopeBuilder;
