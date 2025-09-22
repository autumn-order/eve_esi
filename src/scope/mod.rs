//! # EVE ESI Scopes
//!
//! This module provides the [`ScopeBuilder`] & related modules with methods to build a list of scopes to request during
//! login in a type-safe manner.
//!
//! For an overview & usage examples of OAuth2 with the `eve_esi` crate, see the [module-level documentation](super)
//!
//! ## Modules
//! | Module           | Description                                                                                        |
//! | ---------------- | -------------------------------------------------------------------------------------------------- |
//! | [`builder`]      | Provides the [`ScopeBuilder`] to build a list of scopes                                            |
//! | [`characters`]   | Provides the [`CharacterScopes`] struct to be used with the [`ScopeBuilder::character`] method     |
//! | [`corporations`] | Provides the [`CorporationScopes`] struct to be used with the [`ScopeBuilder::corporation`] method |
//! | [`wallet`]       | Provides the [`WalletScopes`] struct to be used with the [`ScopeBuilder::wallet`] method           |
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
//! See the [`crate::oauth2::login`] module documentation for an example of usage of the [`ScopeBuilder`] with the
//! [`login_url`](crate::oauth2::OAuth2Api::login_url) method.

pub mod builder;

pub mod alliance;
pub mod assets;
pub mod calendar;
pub mod characters;
pub mod clones;
pub mod contracts;
pub mod corporations;
pub mod fittings;
pub mod fleets;
pub mod industry;
pub mod killmails;
pub mod location;
pub mod mail;
pub mod markets;
pub mod planets;
pub mod search;
pub mod skills;
pub mod ui;
pub mod universe;
pub mod wallet;

pub use builder::ScopeBuilder;

pub use alliance::AllianceScopes;
pub use assets::AssetScopes;
pub use calendar::CalendarScopes;
pub use characters::CharacterScopes;
pub use clones::CloneScopes;
pub use contracts::ContractScopes;
pub use corporations::CorporationScopes;
pub use fittings::FittingScopes;
pub use fleets::FleetScopes;
pub use industry::IndustryScopes;
pub use killmails::KillmailScopes;
pub use location::LocationScopes;
pub use mail::MailScopes;
pub use markets::MarketScopes;
pub use planets::PlanetScopes;
pub use search::SearchScopes;
pub use skills::SkillScopes;
pub use ui::UserInterfaceScopes;
pub use universe::UniverseScopes;
pub use wallet::WalletScopes;
