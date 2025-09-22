//! # EVE ESI Corporations Scopes
//!
//! This module provides a type-safe way to add corporation-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! - [`CorporationScopes::new`]: Creates a new instance of [`CorporationScopes`]
//! - [`CorporationScopes::all`]: Creates a new instance of [`CorporationScopes`] with all scopes applied
//! - [`CorporationScopes::read_blueprints`]: Access to retrieve information on corporation's blueprints
//! - [`CorporationScopes::read_container_logs`]: Access to read information on corporation container logs
//! - [`CorporationScopes::read_divisions`]: Access to retrieve information on corporation's wallet & hangar divisions
//! - [`CorporationScopes::read_facilities`]: Access to retrieve information on corporation's industry facilities
//! - [`CorporationScopes::read_medals`]: Access to retrieve information on corporation's medals
//! - [`CorporationScopes::track_members`]: Access to member tracking-related information for a corporation
//! - [`CorporationScopes::read_titles`]: Access to retrieve information on a corporation's member titles
//! - [`CorporationScopes::read_corporation_membership`]: Access to read roles & membership for a corporation
//! - [`CorporationScopes::read_standings`]: Access to retrieve information on a corporation's NPC standings
//! - [`CorporationScopes::read_starbases`]: Access to retrieve information on a corporation's starbases (POSes)
//! - [`CorporationScopes::read_structures`]: Access to retrieve information on corporation's Upwell structures

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
/// Access to retrieve information on corporation's Upwell structures
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

    /// Creates a new instance of [`CorporationScopes`] with all scopes applied
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

    /// Access to retrieve information on corporation's blueprints
    ///
    /// Adds the `esi-corporations.read_blueprints.v1` scope
    pub fn read_blueprints(mut self) -> Self {
        self.scopes.push(READ_BLUEPRINTS.to_string());
        self
    }

    /// Access to read information on corporation container logs
    ///
    /// Adds the `esi-corporations.read_container_logs.v1` scope
    pub fn read_container_logs(mut self) -> Self {
        self.scopes.push(READ_CONTAINER_LOGS.to_string());
        self
    }

    /// Access to retrieve information on corporation's wallet & hangar divisions
    ///
    /// Adds the `esi-corporations.read_divisions.v1` scope
    pub fn read_divisions(mut self) -> Self {
        self.scopes.push(READ_DIVISIONS.to_string());
        self
    }

    /// Access to retrieve information on corporation's industry facilities
    ///
    /// Adds the `esi-corporations.read_facilities.v1` scope
    pub fn read_facilities(mut self) -> Self {
        self.scopes.push(READ_FACILITIES.to_string());
        self
    }

    /// Access to retrieve information on corporation's medals
    ///
    /// Adds the `esi-corporations.read_medals.v1` scope
    pub fn read_medals(mut self) -> Self {
        self.scopes.push(READ_MEDALS.to_string());
        self
    }

    /// Access to member tracking-related information for a corporation
    ///
    /// Adds the `esi-corporations.track_members.v1` scope
    pub fn track_members(mut self) -> Self {
        self.scopes.push(TRACK_MEMBERS.to_string());
        self
    }

    /// Access to retrieve information on a corporation's member titles
    ///
    /// Adds the `esi-corporations.read_titles.v1` scope
    pub fn read_titles(mut self) -> Self {
        self.scopes.push(READ_TITLES.to_string());
        self
    }

    /// Access to read roles & membership for a corporation
    ///
    /// Adds the `esi-corporations.read_corporation_membership.v1` scope
    pub fn read_corporation_membership(mut self) -> Self {
        self.scopes.push(READ_CORPORATION_MEMBERSHIP.to_string());
        self
    }
    /// Access to retrieve information on a corporation's NPC standings
    ///
    /// Adds the `esi-corporations.read_standings.v1` scope
    pub fn read_standings(mut self) -> Self {
        self.scopes.push(READ_STANDINGS.to_string());
        self
    }

    /// Access to retrieve information on a corporation's starbases (POSes)
    ///
    /// Adds the `esi-corporations.read_starbases.v1` scope
    pub fn read_starbases(mut self) -> Self {
        self.scopes.push(READ_STARBASES.to_string());
        self
    }

    /// Access to retrieve information on corporation's Upwell structures
    ///
    /// Adds the `esi-corporations.read_structures.v1` scope
    pub fn read_structures(mut self) -> Self {
        self.scopes.push(READ_STRUCTURES.to_string());
        self
    }
}

#[cfg(test)]
mod corporation_scopes_tests {
    use crate::scope::CorporationScopes;

    /// Tests initializing a default instance of [`CorporationScopes`]
    #[test]
    fn test_corporation_scopes_default() {
        let corporation_scopes = CorporationScopes::default();

        assert_eq!(corporation_scopes.scopes.len(), 0)
    }
}
