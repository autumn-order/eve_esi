//! # EVE ESI Character Scopes
//!
//! This module provides a type-safe way to add character-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! - [`CharacterScopes::new`]: Create a new instance of [`CharacterScopes`]
//! - [`CharacterScopes::all`]: Creates a new instance of [`CharacterScopes`] with all scopes applied
//! - [`CharacterScopes::read_agents_research`]: Access to retrieve information on character's research agents
//! - [`CharacterScopes::read_blueprints`]: Access to retrieve information on character's blueprints
//! - [`CharacterScopes::read_contacts`]: Access to read a character's contacts
//! - [`CharacterScopes::read_fatigue`]: Access to retrieve information on character's jump fatigue status
//! - [`CharacterScopes::read_medals`]: Access to retrieve information on character's medals
//! - [`CharacterScopes::read_notifications`]: Access to retrieve the character's notifications
//! - [`CharacterScopes::read_corporation_roles`]: Access to read the character's corporation roles
//! - [`CharacterScopes::read_standings`]: Access to read the character's standings
//! - [`CharacterScopes::read_titles`]: Access to read the character's corporation titles

/// Access to retrieve information on character's research agents
pub const READ_AGENTS_RESEARCH: &str = "esi-characters.read_agents_research.v1";
/// Access to retrieve information on character's blueprints
pub const READ_BLUEPRINTS: &str = "esi-characters.read_blueprints.v1";
/// Access to read a character's contacts
pub const READ_CONTACTS: &str = "esi-characters.read_contacts.v1";
/// Access to retrieve information on character's jump fatigue status
pub const READ_FATIGUE: &str = "esi-characters.read_fatigue.v1";
/// Access to retrieve information on character's medals
pub const READ_MEDALS: &str = "esi-characters.read_medals.v1";
/// Access to retrieve the character's notifications
pub const READ_NOTIFICATIONS: &str = "esi-characters.read_notifications.v1";
/// Access to read the character's corporation roles
pub const READ_CORPORATION_ROLES: &str = "esi-characters.read_corporation_roles.v1";
/// Access to read the character's standings
pub const READ_STANDINGS: &str = "esi-characters.read_standings.v1";
/// Access to read the character's corporation titles
pub const READ_TITLES: &str = "esi-characters.read_titles.v1";

/// Struct with methods for listing character scopes to request for OAuth2
pub struct CharacterScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for CharacterScopes {
    /// Create a default instance of [`CharacterScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl CharacterScopes {
    /// Create a new instance of [`CharacterScopes`]
    pub fn new() -> Self {
        CharacterScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`CharacterScopes`] with all scopes applied
    pub fn all() -> Self {
        CharacterScopes::new()
            .read_agents_research()
            .read_blueprints()
            .read_contacts()
            .read_fatigue()
            .read_medals()
            .read_notifications()
            .read_corporation_roles()
            .read_standings()
            .read_titles()
    }

    /// Access to retrieve information on character's research agents
    ///
    /// Adds the `esi-characters.read_agents_research.v1` scope
    pub fn read_agents_research(mut self) -> Self {
        self.scopes.push(READ_AGENTS_RESEARCH.to_string());
        self
    }

    /// Access to retrieve information on character's blueprints
    ///
    /// Adds the `esi-characters.read_blueprints.v1` scope
    pub fn read_blueprints(mut self) -> Self {
        self.scopes.push(READ_BLUEPRINTS.to_string());
        self
    }

    /// Access to read a character's contacts
    ///
    /// Adds the `esi-characters.read_contacts.v1` scope
    pub fn read_contacts(mut self) -> Self {
        self.scopes.push(READ_CONTACTS.to_string());
        self
    }

    /// Access to retrieve information on character's jump fatigue status
    ///
    /// Adds the `esi-characters.read_fatigue.v1` scope
    pub fn read_fatigue(mut self) -> Self {
        self.scopes.push(READ_FATIGUE.to_string());
        self
    }

    /// Access to retrieve information on character's medals
    ///
    /// Adds the `esi-characters.read_medals.v1` scope
    pub fn read_medals(mut self) -> Self {
        self.scopes.push(READ_MEDALS.to_string());
        self
    }

    /// Access to retrieve the character's notifications
    ///
    /// Adds the `esi-characters.read_notifications.v1` scope
    pub fn read_notifications(mut self) -> Self {
        self.scopes.push(READ_NOTIFICATIONS.to_string());
        self
    }

    /// Access to read the character's corporation roles
    ///
    /// Adds the `esi-characters.read_corporation_roles.v1` scope
    pub fn read_corporation_roles(mut self) -> Self {
        self.scopes.push(READ_CORPORATION_ROLES.to_string());
        self
    }

    /// Access to read the character's standings
    ///
    /// Adds the `esi-characters.read_standings.v1` scope
    pub fn read_standings(mut self) -> Self {
        self.scopes.push(READ_STANDINGS.to_string());
        self
    }

    /// Access to read the character's corporation titles
    ///
    /// Adds the `esi-characters.read_titles.v1` scope
    pub fn read_titles(mut self) -> Self {
        self.scopes.push(READ_TITLES.to_string());
        self
    }
}

#[cfg(test)]
mod character_scopes_tests {
    use crate::oauth2::scope::CharacterScopes;

    /// Tests initializing a default instance of [`CharacterScopes`]
    #[test]
    fn test_character_scopes_default() {
        let character_scopes = CharacterScopes::default();

        assert_eq!(character_scopes.scopes.len(), 0)
    }
}
