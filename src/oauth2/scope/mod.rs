//! # EVE Online OAuth2 Scopes
//!
//! This module provides the [`ScopeBuilder`] & related modules with methods to build a list of scopes to request during
//! login in a type-safe manner.
//!
//! # Modules
//!
//! - [`builder`]: Provides the [`ScopeBuilder`] to build a list of scopes
//! - [`character`]: Provides the [`CharacterScopes`] struct to be used with the [`ScopeBuilder::character`] method
//!
//! # Usage
//!
//! ```rust
//! use eve_esi::ScopeBuilder;
//! use eve_esi::oauth2::scope::CharacterScopes;
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

pub use builder::ScopeBuilder;

pub use character::CharacterScopes;
pub use corporation::CorporationScopes;
