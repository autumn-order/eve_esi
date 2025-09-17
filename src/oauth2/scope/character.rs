//! # EVE Online OAuth2 Character Scopes
//!
//! This module provides a type-safe way to add character releated scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! # Methods
//! - [`CharacterScopes::read_agents_research`]: Adds the `esi-characters.read_agents_research.v1` scope

/// Access to retrieve information on character's research agents
pub const READ_AGENTS_RESEARCH: &str = "esi-characters.read_agents_research.v1";
/// Access to retrieve information on character's blueprints
pub const READ_BLUEPRINTS: &str = "esi-characters.read_blueprints.v1";
/// Access to retrieve information on character's jump fatigue status
pub const READ_FATIGUE: &str = "esi-characters.read_fatigue.v1";
/// Access to retrieve information on character's medals
pub const READ_MEDALS: &str = "esi-characters.read_medals.v1";
/// Access to retrieve the character's notifications
pub const READ_NOTIFICATIONS: &str = "esi-characters.read_notifications.v1";
/// Access to read the character's corporation roles
pub const READ_CORPORATION_ROLES: &str = "esi-characters.read_corporation_roles.v1";

/// Struct with methods for listing character scopes to request for OAuth2
pub struct CharacterScopes {
    pub(super) scopes: Vec<String>,
}

impl CharacterScopes {
    /// Create a new instance of [`CharacterScopes`]
    pub fn new() -> Self {
        CharacterScopes { scopes: Vec::new() }
    }

    /// Adds the `esi-characters.read_agents_research.v1` scope
    pub fn read_agents_research(mut self) -> Self {
        self.scopes.push(READ_AGENTS_RESEARCH.to_string());
        self
    }

    /// Adds the `esi-characters.read_blueprints.v1` scope
    pub fn read_blueprints(mut self) -> Self {
        self.scopes.push(READ_BLUEPRINTS.to_string());
        self
    }

    /// Adds the `esi-characters.read_fatigue.v1` scope
    pub fn read_fatigue(mut self) -> Self {
        self.scopes.push(READ_FATIGUE.to_string());
        self
    }

    /// Adds the `esi-characters.read_medals.v1` scope
    pub fn read_medals(mut self) -> Self {
        self.scopes.push(READ_MEDALS.to_string());
        self
    }

    /// Adds the `esi-characters.read_notifications.v1` scope
    pub fn read_notifications(mut self) -> Self {
        self.scopes.push(READ_NOTIFICATIONS.to_string());
        self
    }

    /// Adds the `esi-characters.read_corporation_roles.v1` scope
    pub fn read_corporation_roles(mut self) -> Self {
        self.scopes.push(READ_CORPORATION_ROLES.to_string());
        self
    }
}
