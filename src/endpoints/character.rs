//! # EVE ESI Character Endpoints
//!
//! This module provides the [`CharacterApi`] struct and associated methods for accessing
//! character-related ESI endpoints.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
//!
//! # ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! # Methods
//! - [`CharacterApi::get_character_public_information`]: Retrieves the public information of a specific character
//! - [`CharacterApi::character_affiliation`]: Retrieve affiliations for a list of characters
//! - [`CharacterApi::get_agents_research`]: Retrieves character's research agents using the character's ID

use crate::error::Error;
use crate::oauth2::scope::CharacterScopes;
use crate::{Client, ScopeBuilder};

use crate::model::character::{
    Blueprint, Character, CharacterAffiliation, CharacterCorporationHistory,
    CharacterCorporationRole, CharacterCorporationTitle, CharacterJumpFatigue, CharacterMedal,
    CharacterNotification, CharacterPortraits, CharacterResearchAgent, CharacterStanding,
};

/// Provides methods for accessing character-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct CharacterApi<'a> {
    client: &'a Client,
}

impl<'a> CharacterApi<'a> {
    /// Creates a new instance of `CharacterApi`.
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    ///
    /// # Returns
    /// - [`CharacterApi`]: Struct providing methods to interact with character ESI endpoints
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    define_endpoint! {
        /// Retrieves the public information of the provided character ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterId>
        ///
        /// # Arguments
        /// - `character_id` (`i64`): The ID of the character to retrieve information for.
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - [`Character`]: The character's information if successfully retrieved
        /// - [`Error`]: An error if the fetch request fails
        pub_get get_character_public_information(
            character_id: i64
        ) -> Result<Character, Error>
        url = "{}/characters/{}";
        label = "public information";
    }

    define_endpoint! {
        /// Retrieve affiliations for a list of characters
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/PostCharactersAffiliation>
        ///
        /// # Arguments
        /// - `body` (Vec<[`i64`]>): A vec of character IDs to retrieve affiliations for.
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - Vec<[`CharacterAffiliation`]>: The affiliations of the characters if successfully retrieved
        /// - [`Error`]: An error if the fetch request fails
        pub_post character_affiliation(
            body: Vec<i64>,
        ) -> Result<Vec<CharacterAffiliation>, Error>
        url = "{}/characters/affiliation";
        label = "character affiliation";
    }

    define_endpoint! {
        /// Retrieves character's research agents using the character's ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdAgentsResearchGet>
        ///
        /// # Required Scopes
        /// - [`CharacterScopes::read_agents_research`](crate::oauth2::scope::CharacterScopes::read_agents_research):
        ///   `esi-characters.read_agents_research.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve research agent information for.
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - Vec<[`CharacterResearchAgent`]>: A Vec of the character's research agents
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_agents_research(
            access_token: &str,
            character_id: i64,
        ) -> Result<Vec<CharacterResearchAgent>, Error>
        url = "{}/characters/{}/agents_research";
        label = "research agents";
        required_scopes = ScopeBuilder::new()
            .character(CharacterScopes::new().read_agents_research())
            .build();
    }

    define_endpoint! {
        /// Retrieves character's blueprints using the character's ID & page to fetch of the blueprint list
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdBlueprints>
        ///
        /// # Required Scopes
        /// - [`CharacterScopes::read_blueprints`](crate::oauth2::scope::CharacterScopes::read_blueprints):
        ///   `esi-characters.read_blueprints.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve research agent information for.
        /// - `page`         (`i32`): The page of blueprints to retrieve
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - Vec<[`Blueprint`]>: A Vec of the character's blueprints
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_blueprints(
            access_token: &str,
            character_id: i64,
            page: i32,
        ) -> Result<Vec<Blueprint>, Error>
        url = "{}/characters/{}/blueprints?page={}";
        label = "blueprints";
        required_scopes = ScopeBuilder::new().character(CharacterScopes::new().read_blueprints()).build();
    }

    define_endpoint! {
        /// Retrieves the public corporation history of the provided character ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdCorporationhistory>
        ///
        /// # Arguments
        /// - `character_id` (`i64`): The ID of the character to retrieve corporation history for.
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - [Vec<`CharacterCorporationHistory`>]: The character's corporation history if request is successful
        /// - [`Error`]: An error if the fetch request fails
        pub_get get_corporation_history(
            character_id: i64
        ) -> Result<Vec<CharacterCorporationHistory>, Error>
        url = "{}/characters/{}/corporationhistory";
        label = "corporation history";
    }

    define_endpoint! {
        /// Retrieves the CSPA charge cost for evemailing the provided character ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <developers.eveonline.com/api-explorer#/operations/PostCharactersCharacterIdCspa>
        ///
        /// # Arguments
        /// - `character_id` (`i64`): The ID of the character to retrieve the CSPA charge cost for
        ///   evemailing the character.
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<i64>`: The CSPA charge cost for evemailing the character
        /// - [`Error`]: An error if the fetch request fails
        pub_get calculate_a_cspa_charge_cost(
            character_id: i64
        ) -> Result<Vec<i64>, Error>
        url = "{}/characters/{}/cspa";
        label = "CSPA charge cost";
    }

    define_endpoint! {
        /// Retrieves jump fatigue for the provided character's ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdFatigue>
        ///
        /// # Required Scopes
        /// - [`CharacterScopes::read_fatigue`](crate::oauth2::scope::CharacterScopes::read_fatigue):
        ///   `esi-characters.read_fatigue.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve jump fatigue for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - [`CharacterJumpFatigue`]: The character's jump fatigue status
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_jump_fatigue(
            access_token: &str,
            character_id: i64
        ) -> Result<CharacterJumpFatigue, Error>
        url = "{}/characters/{}/fatigue";
        label = "jump fatigue";
        required_scopes = ScopeBuilder::new().character(CharacterScopes::new().read_fatigue()).build();
    }

    define_endpoint! {
        /// Retrieves a list of medals for the provided character ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdMedals>
        ///
        /// # Required Scopes
        /// - [`CharacterScopes::read_medals`](crate::oauth2::scope::CharacterScopes::read_medals):
        ///   `esi-characters.read_medals.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve jump fatigue for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CharacterMedal`]`>`: A list of the character's medals
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_medals(
            access_token: &str,
            character_id: i64
        ) -> Result<Vec<CharacterMedal>, Error>
        url = "{}/characters/{}/medals";
        label = "medals";
        required_scopes = ScopeBuilder::new().character(CharacterScopes::new().read_medals()).build();
    }

    define_endpoint! {
        /// Retrieves a list of character's notifications
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdNotifications>
        ///
        /// # Required Scopes
        /// - [`CharacterScopes::read_notifications`](crate::oauth2::scope::CharacterScopes::read_notifications):
        ///   `esi-characters.read_notifications.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve notifications for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CharacterNotification`]`>`: A list of the character's notifications
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_character_notifications(
            access_token: &str,
            character_id: i64
        ) -> Result<Vec<CharacterNotification>, Error>
        url = "{}/characters/{}/notifications";
        label = "notifications";
        required_scopes = ScopeBuilder::new().character(CharacterScopes::new().read_notifications()).build();
    }

    define_endpoint! {
        /// Retrieves the image URLs of a chacter's portraits with various dimensions
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdPortrait>
        ///
        /// # Arguments
        /// - `character_id` (`i64`): The ID of the character to retrieve portrait image URLs for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - [`CharacterPortraits`]: Struct of character's portrait URLs with various dimensions
        /// - [`Error`]: An error if the fetch request fails
        pub_get get_character_portraits(
            character_id: i64
        ) -> Result<CharacterPortraits, Error>
        url = "{}/characters/{}/portrait";
        label = "portraits";
    }

    define_endpoint! {
        /// Retrieves a list of the provided character ID's corporation roles
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdRoles>
        ///
        /// # Required Scopes
        /// - [`CharacterScopes::read_corporation_roles`](crate::oauth2::scope::CharacterScopes::read_corporation_roles):
        ///   `esi-characters.read_corporation_roles.v1`
        ///
        /// # Arguments
        /// - `character_id` (`i64`): The ID of the character to retrieve corporation roles for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CharacterCorporationRole`]`>`: List of entires for the provided character ID's corporation roles.
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_character_corporation_roles(
            access_token: &str,
            character_id: i64
        ) -> Result<Vec<CharacterCorporationRole>, Error>
        url = "{}/characters/{}/roles";
        label = "corporation roles";
        required_scopes = ScopeBuilder::new().character(CharacterScopes::new().read_corporation_roles()).build();
    }

    define_endpoint! {
        /// Retrieves a list of the provided character ID's standings
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdStandings>
        ///
        /// # Required Scopes
        /// - [`CharacterScopes::read_standings`](crate::oauth2::scope::CharacterScopes::read_standings):
        ///   `esi-characters.read_standings.v1`
        ///
        /// # Arguments
        /// - `character_id` (`i64`): The ID of the character to retrieve standings for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CharacterStanding`]`>`: List of entries for the provided character ID's standings
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_standings(
            access_token: &str,
            character_id: i64
        ) -> Result<Vec<CharacterStanding>, Error>
        url = "{}/characters/{}/standings";
        label = "standings";
        required_scopes = ScopeBuilder::new().character(CharacterScopes::new().read_standings()).build();
    }

    define_endpoint! {
        /// Retrieves a list of the provided character ID's corporation titles
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdTitles>
        ///
        /// # Required Scopes
        /// - [`CharacterScopes::read_titles`](crate::oauth2::scope::CharacterScopes::read_titles):
        ///   `esi-characters.read_titles.v1`
        ///
        /// # Arguments
        /// - `character_id` (`i64`): The ID of the character to retrieve corporation titles for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CharacterCorporationTitle`]`>`: List of entries for the provided character ID's
        ///   corporation titles
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_character_corporation_titles(
            access_token: &str,
            character_id: i64
        ) -> Result<Vec<CharacterCorporationTitle>, Error>
        url = "{}/characters/{}/titles";
        label = "standings";
        required_scopes = ScopeBuilder::new().character(CharacterScopes::new().read_titles()).build();
    }
}
