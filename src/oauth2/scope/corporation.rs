//! # EVE Online OAuth2 Corporation Scopes
//!
//! This module provides a type-safe way to add corporation-releated scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! # Methods
//! - [`CorporationScopes::new`]: Creates a new instance of [`CorporationScopes`]

/// Access to retrieve information on corporation's blueprints
pub const READ_BLUEPRINTS: &str = "esi-corporations.read_blueprints.v1";
/// Access to read information on corporation container logs
pub const READ_CONTAINER_LOGS: &str = "esi-corporations.read_container_logs.v1";
/// Access to retrieve information on corporation's wallet & hangar divisions
pub const READ_DIVISIONS: &str = "esi-corporations.read_divisions.v1";
/// Access to retrieve information on corporation's industry facilities
pub const READ_FACILITIES: &str = "esi-corporations.read_facilities.v1";
/// Access to retrieve information on corporation's medals
pub const READ_MEDALS: &str = "esi-corporations.read_medals.v1";
/// Access to member tracking-related information for a corporation
pub const TRACK_MEMBERS: &str = "esi-corporations.track_members.v1";
/// Access to retrieve information on a corporation's member titles
pub const READ_TITLES: &str = "esi-corporations.read_titles.v1";

/// Struct with methods for listing corporation scopes to request for OAuth2
pub struct CorporationScopes {
    pub(super) scopes: Vec<String>,
}

impl CorporationScopes {
    /// Create a new instance of [`CorporationScopes`]
    pub fn new() -> Self {
        CorporationScopes { scopes: Vec::new() }
    }

    /// Adds the `esi-corporations.read_blueprints.v1` scope
    pub fn read_blueprints(mut self) -> Self {
        self.scopes.push(READ_BLUEPRINTS.to_string());
        self
    }

    /// Adds the `esi-corporations.read_container_logs.v1` scope
    pub fn read_container_logs(mut self) -> Self {
        self.scopes.push(READ_CONTAINER_LOGS.to_string());
        self
    }

    /// Adds the `esi-corporations.read_divisions.v1` scope
    pub fn read_divisions(mut self) -> Self {
        self.scopes.push(READ_DIVISIONS.to_string());
        self
    }

    /// Adds the `esi-corporations.read_facilities.v1` scope
    pub fn read_facilities(mut self) -> Self {
        self.scopes.push(READ_FACILITIES.to_string());
        self
    }

    /// Adds the `esi-corporations.read_medals.v1` scope
    pub fn read_medals(mut self) -> Self {
        self.scopes.push(READ_MEDALS.to_string());
        self
    }

    /// Adds the `esi-corporations.track_members.v1` scope
    pub fn track_members(mut self) -> Self {
        self.scopes.push(TRACK_MEMBERS.to_string());
        self
    }

    /// Adds the `esi-corporations.read_titles.v1` scope
    pub fn read_titles(mut self) -> Self {
        self.scopes.push(READ_TITLES.to_string());
        self
    }
}
