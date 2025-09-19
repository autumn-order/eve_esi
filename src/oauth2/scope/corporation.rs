//! # EVE Online OAuth2 Corporation Scopes
//!
//! This module provides a type-safe way to add corporation-related scopes for OAuth2 to the [`super::ScopeBuilder`]
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
/// Access to read roles & membership for a corporation
pub const READ_CORPORATION_MEMBERSHIP: &str = "esi-corporations.read_corporation_membership.v1";
/// Access to retrieve information on a corporation's NPC standings
pub const READ_STANDINGS: &str = "esi-corporations.read_standings.v1";
/// Access to retrieve information on a corporation's starbases (POSes)
pub const READ_STARBASES: &str = "esi-corporations.read_starbases.v1";
/// Access to retrieve information on corporation's structures
pub const READ_STRUCTURES: &str = "esi-corporations.read_structures.v1";

/// Struct with methods for listing corporation scopes to request for OAuth2
pub struct CorporationScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for CorporationScopes {
    /// Create a default instance of [`CorporationScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl CorporationScopes {
    /// Create a new instance of [`CorporationScopes`]
    pub fn new() -> Self {
        CorporationScopes { scopes: Vec::new() }
    }

    /// Create a new instance of [`CorporationScopes`] with all scopes applied
    pub fn all() -> Self {
        CorporationScopes::new()
            .read_blueprints()
            .read_container_logs()
            .read_divisions()
            .read_facilities()
            .read_medals()
            .track_members()
            .read_titles()
            .read_corporation_membership()
            .read_standings()
            .read_starbases()
            .read_structures()
    }

    /// Adds the `esi-corporations.read_blueprints.v1` scope
    ///
    /// Access to retrieve information on corporation's blueprints
    pub fn read_blueprints(mut self) -> Self {
        self.scopes.push(READ_BLUEPRINTS.to_string());
        self
    }

    /// Adds the `esi-corporations.read_container_logs.v1` scope
    ///
    /// Access to read information on corporation container logs
    pub fn read_container_logs(mut self) -> Self {
        self.scopes.push(READ_CONTAINER_LOGS.to_string());
        self
    }

    /// Adds the `esi-corporations.read_divisions.v1` scope
    ///
    /// Access to retrieve information on corporation's wallet & hangar divisions
    pub fn read_divisions(mut self) -> Self {
        self.scopes.push(READ_DIVISIONS.to_string());
        self
    }

    /// Adds the `esi-corporations.read_facilities.v1` scope
    ///
    /// Access to retrieve information on corporation's industry facilities
    pub fn read_facilities(mut self) -> Self {
        self.scopes.push(READ_FACILITIES.to_string());
        self
    }

    /// Adds the `esi-corporations.read_medals.v1` scope
    ///
    /// Access to retrieve information on corporation's medals
    pub fn read_medals(mut self) -> Self {
        self.scopes.push(READ_MEDALS.to_string());
        self
    }

    /// Adds the `esi-corporations.track_members.v1` scope
    ///
    /// Access to member tracking-related information for a corporation
    pub fn track_members(mut self) -> Self {
        self.scopes.push(TRACK_MEMBERS.to_string());
        self
    }

    /// Adds the `esi-corporations.read_titles.v1` scope
    ///
    /// Access to retrieve information on a corporation's member titles
    pub fn read_titles(mut self) -> Self {
        self.scopes.push(READ_TITLES.to_string());
        self
    }

    /// Adds the `esi-corporations.read_corporation_membership.v1` scope
    ///
    /// Access to read roles & membership for a corporation
    pub fn read_corporation_membership(mut self) -> Self {
        self.scopes.push(READ_CORPORATION_MEMBERSHIP.to_string());
        self
    }

    /// Adds the `esi-corporations.read_standings.v1` scope
    ///
    /// Access to retrieve information on a corporation's NPC standings
    pub fn read_standings(mut self) -> Self {
        self.scopes.push(READ_STANDINGS.to_string());
        self
    }

    /// Adds the `esi-corporations.read_starbases.v1` scope
    ///
    /// Access to retrieve information on a corporation's starbases (POSes)
    pub fn read_starbases(mut self) -> Self {
        self.scopes.push(READ_STARBASES.to_string());
        self
    }

    /// Adds the `esi-corporations.read_structures.v1` scope
    ///
    /// Access to retrieve information on corporation's structures
    pub fn read_structures(mut self) -> Self {
        self.scopes.push(READ_STRUCTURES.to_string());
        self
    }
}
