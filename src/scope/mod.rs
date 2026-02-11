//! # EVE ESI Scopes
//!
//! This module provides the [`ScopeBuilder`] & related modules with methods to build a list of scopes to request during
//! login in a type-safe manner.
//!
//! ## Usage Example
//!
//! ```rust
//! use eve_esi::ScopeBuilder;
//! use eve_esi::scope::CharactersScopes;
//!
//! // Create a new scope builder
//! let scopes = ScopeBuilder::new()
//!     // Add `publicData` scope
//!     .public_data()
//!     // Add character scopes
//!     .characters(
//!         CharactersScopes::new()
//!             .read_agents_research()
//!     )
//!     // Build the scopes into Vec<String>
//!     .build();
//!
//! // Use with `esi_client.oauth2().login_url(scopes)` method...
//! ```

pub mod builder;

pub mod alliances;
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

pub use alliances::AlliancesScopes;
pub use assets::AssetsScopes;
pub use calendar::CalendarScopes;
pub use characters::CharactersScopes;
pub use clones::ClonesScopes;
pub use contracts::ContractsScopes;
pub use corporations::CorporationsScopes;
pub use fittings::FittingsScopes;
pub use fleets::FleetsScopes;
pub use industry::IndustryScopes;
pub use killmails::KillmailsScopes;
pub use location::LocationScopes;
pub use mail::MailScopes;
pub use markets::MarketsScopes;
pub use planets::PlanetsScopes;
pub use search::SearchScopes;
pub use skills::SkillsScopes;
pub use ui::UiScopes;
pub use universe::UniverseScopes;
pub use wallet::WalletScopes;
