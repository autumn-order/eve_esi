//! # EVE ESI Scopes
//!
//! This module provides the [`ScopeBuilder`] & related modules with methods to build a list of scopes to request during
//! login in a type-safe manner.
//!
//! For an overview & usage examples of OAuth2 with the `eve_esi` crate, see the [module-level documentation](super)
//!
//! ## Modules
//!
//! - [`builder`]: Provides the [`ScopeBuilder`] to build a list of scopes
//! - [`character`]: Provides the [`CharacterScopes`] struct to be used with the [`ScopeBuilder::character`] method
//! - [`corporation`]: Provides the [`CorporationScopes`] struct to be used with the [`ScopeBuilder::corporation`] method
//! - [`wallet`]: Provides the [`WalletScopes`] struct to be used with the [`ScopeBuilder::wallet`] method
//!
//! ## Usage Example
//!
//! ```rust
//! use eve_esi::ScopeBuilder;
//! use eve_esi::scope::CharacterScopes;
//!
//! // Create a new scope builder
//! let scopes = ScopeBuilder::new()
//!     // Add `publicData` scope
//!     .public_data()
//!     // Add character scopes
//!     .character(
//!         CharacterScopes::new()
//!             .read_agents_research()
//!     )
//!     // Build the scopes into Vec<String>
//!     .build();
//!
//! // Use with `esi_client.oauth2().login_url(scopes)` method...
//! ```
//!
//! See the [`super::login`] module for an example of usage of the [`ScopeBuilder`] with the
//! [`login_url`](crate::oauth2::OAuth2Api::login_url) method.

pub mod builder;

pub mod character;
pub mod corporation;
pub mod market;
pub mod wallet;

pub use builder::ScopeBuilder;

pub use character::CharacterScopes;
pub use corporation::CorporationScopes;
pub use market::MarketScopes;
pub use wallet::WalletScopes;
