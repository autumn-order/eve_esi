//! # OAuth2 Config
//!
//! Configuration options & client for the OAuth2 portion of the EsiClient.
//!
//! Provides methods to override default OAuth2 behavior for more advanced
//! users via the [self::config::OAuth2Config] struct.
//!
//! Additionally provides the OAuth2 client which facilitates requests to
//! EVE Online's OAuth2 endpoints.
//!
//! - See [OAuth2 Config Builder docs](builder) for instructions on setting up a custom OAuth2 config
//! - See [EsiClientBuilder docs](crate::builder) for instructions on setting up OAuth2 for the eve_esi crate.
//!
//! ## Modules
//!
//! - [`config`]: Config to override default OAuth2 behavior
//! - [`builder`]: Builder methods to configure the OAuth2 config
//! - [`client`]: OAuth2 client used to interact with EVE's OAuth2 endpoints
//!
//! ## Usage
//!
//! ```
//! use eve_esi::EsiClient;
//! use eve_esi::oauth2::OAuth2Config;
//!
//! // Set 2 hour JWT key cache lifetime in seconds
//! let config = OAuth2Config::builder()
//!     .jwk_cache_ttl(7200)
//!     .build()
//!     .expect("Failed to build OAuth2Config");
//!
//! // Setup EsiClient with OAuth2 settings and custom OAuth2 config
//! let esi_client = EsiClient::builder()
//!     .user_agent("MyApp/1.0 (contact@example.com)")
//!     .client_id("client_id")
//!     .client_secret("client_secret")
//!     .callback_url("http://localhost:8080/callback")
//!     .oauth2_config(config)
//!     .build()
//!     .expect("Failed to build EsiClient");
//! ```

pub mod builder;
pub mod client;
pub mod config;
