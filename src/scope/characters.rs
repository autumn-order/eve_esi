//! # EVE ESI Characters Scopes
//!
//! This module provides a type-safe way to add character-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                       | Description                                                            |
//! | -------------------------------------------- | ---------------------------------------------------------------------- |
//! | [`CharactersScopes::new`]                    | Create a new instance of [`CharactersScopes`]                          |
//! | [`CharactersScopes::all`]                    | Creates a new instance of [`CharactersScopes`] with all scopes applied |
//! | [`CharactersScopes::read_agents_research`]   | Access to retrieve information on character's research agents          |
//! | [`CharactersScopes::read_blueprints`]        | Access to retrieve information on character's blueprints               |
//! | [`CharactersScopes::read_contacts`]          | Access to read a character's contacts                                  |
//! | [`CharactersScopes::read_fatigue`]           | Access to retrieve information on character's jump fatigue status      |
//! | [`CharactersScopes::read_medals`]            | Access to retrieve information on character's medals                   |
//! | [`CharactersScopes::read_notifications`]     | Access to retrieve the character's notifications                       |
//! | [`CharactersScopes::read_corporation_roles`] | Access to read the character's corporation roles                       |
//! | [`CharactersScopes::read_standings`]         | Access to read the character's standings                               |
//! | [`CharactersScopes::read_titles`]            | Access to read the character's corporation titles                      |
//! | [`CharactersScopes::read_chat_channels`]     | Access to read chat channels character is in                           |
//! | [`CharactersScopes::read_fw_stats`]          | Access to retrieve character's faction warfare stats                   |
//! | [`CharactersScopes::read_loyalty`]           | Access to retrieve character's loyalty point information               |
//! | [`CharactersScopes::write_contacts`]         | Access to add/modify character contacts                                |

/// Access to retrieve information on character's research agents
pub const READ_AGENTS_RESEARCH: &str = "esi-characters.read_agents_research.v1";
/// Access to retrieve information on character's blueprints
pub const READ_BLUEPRINTS: &str = "esi-characters.read_blueprints.v1";
/// Access to read chat channels character is in (does not include channel messages)
pub const READ_CHAT_CHANNELS: &str = "esi-characters.read_chat_channels.v1";
/// Access to read a character's contacts
pub const READ_CONTACTS: &str = "esi-characters.read_contacts.v1";
/// Access to read the character's corporation roles
pub const READ_CORPORATION_ROLES: &str = "esi-characters.read_corporation_roles.v1";
/// Access to retrieve information on character's jump fatigue status
pub const READ_FATIGUE: &str = "esi-characters.read_fatigue.v1";
/// Access to retrieve character's faction warfare stats
pub const READ_FW_STATS: &str = "esi-characters.read_fw_stats.v1";
/// Access to retrieve character's loyalty point information
pub const READ_LOYALTY: &str = "esi-characters.read_loyalty.v1";
/// Access to retrieve information on character's medals
pub const READ_MEDALS: &str = "esi-characters.read_medals.v1";
/// Access to retrieve the character's notifications
pub const READ_NOTIFICATIONS: &str = "esi-characters.read_notifications.v1";
/// Access to read the character's standings
pub const READ_STANDINGS: &str = "esi-characters.read_standings.v1";
/// Access to read the character's corporation titles
pub const READ_TITLES: &str = "esi-characters.read_titles.v1";
/// Access to add/modify character contacts
pub const WRITE_CONTACTS: &str = "esi-characters.write_contacts.v1";

/// Struct with methods for listing character scopes to request for OAuth2
pub struct CharactersScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for CharactersScopes {
    /// Create a default instance of [`CharactersScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl CharactersScopes {
    /// Create a new instance of [`CharactersScopes`]
    pub fn new() -> Self {
        CharactersScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`CharactersScopes`] with all scopes applied
    pub fn all() -> Self {
        CharactersScopes::new()
            .read_agents_research()
            .read_blueprints()
            .read_chat_channels()
            .read_contacts()
            .read_corporation_roles()
            .read_fatigue()
            .read_fw_stats()
            .read_loyalty()
            .read_medals()
            .read_notifications()
            .read_standings()
            .read_titles()
            .write_contacts()
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

    /// Access to read chat channels character is in (does not include channel messages)
    ///
    /// Adds the `esi-characters.read_chat_channels.v1` scope
    pub fn read_chat_channels(mut self) -> Self {
        self.scopes.push(READ_CHAT_CHANNELS.to_string());
        self
    }

    /// Access to read a character's contacts
    ///
    /// Adds the `esi-characters.read_contacts.v1` scope
    pub fn read_contacts(mut self) -> Self {
        self.scopes.push(READ_CONTACTS.to_string());
        self
    }

    /// Access to read the character's corporation roles
    ///
    /// Adds the `esi-characters.read_corporation_roles.v1` scope
    pub fn read_corporation_roles(mut self) -> Self {
        self.scopes.push(READ_CORPORATION_ROLES.to_string());
        self
    }

    /// Access to retrieve information on character's jump fatigue status
    ///
    /// Adds the `esi-characters.read_fatigue.v1` scope
    pub fn read_fatigue(mut self) -> Self {
        self.scopes.push(READ_FATIGUE.to_string());
        self
    }

    /// Access to retrieve character's faction warfare stats
    ///
    /// Adds the `esi-characters.read_fw_stats.v1` scope
    pub fn read_fw_stats(mut self) -> Self {
        self.scopes.push(READ_FW_STATS.to_string());
        self
    }

    /// Access to retrieve character's loyalty point information
    ///
    /// Adds the `esi-characters.read_loyalty.v1` scope
    pub fn read_loyalty(mut self) -> Self {
        self.scopes.push(READ_LOYALTY.to_string());
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

    /// Access to add/modify character contacts
    ///
    /// Adds the `esi-characters.write_contacts.v1` scope
    pub fn write_contacts(mut self) -> Self {
        self.scopes.push(WRITE_CONTACTS.to_string());
        self
    }
}

#[cfg(test)]
mod character_scopes_tests {
    use crate::scope::CharactersScopes;

    /// Tests initializing a default instance of [`CharactersScopes`]
    #[test]
    fn test_character_scopes_default() {
        let characters_scopes = CharactersScopes::default();

        assert_eq!(characters_scopes.scopes.len(), 0)
    }
}
