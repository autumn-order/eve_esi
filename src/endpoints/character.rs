//! # EVE ESI Character Endpoints
//!
//! This module provides the [`CharacterApi`] struct and associated methods for accessing
//! character-related ESI endpoints.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! ## Endpoints (11)
//! ### Public (3)
//! - [`CharacterApi::get_character_public_information`]: Retrieves the public information of a specific character
//! - [`CharacterApi::get_corporation_history`]: Retrieves the public corporation history of the provided character ID
//! - [`CharacterApi::get_character_portraits`]: Retrieves the image URLs of a chacter's portraits with various dimensions
//!
//! ### Authenticated (9)
//! - [`CharacterApi::get_agents_research`]: Retrieves character's research agents using the character's ID
//! - [`CharacterApi::get_blueprints`]: Retrieves character's blueprints using the character's ID & page to fetch of the blueprint list
//! - [`CharacterApi::calculate_a_cspa_charge_cost`]: Calculates CSPA cost for evemailing a list of characters with the provided character ID
//! - [`CharacterApi::get_jump_fatigue`]: Retrieves jump fatigue for the provided character's ID
//! - [`CharacterApi::get_medals`]: Retrieves a list of medals for the provided character ID
//! - [`CharacterApi::get_character_notifications`]: Retrieves a list of character's notifications
//! - [`CharacterApi::get_character_corporation_roles`]: Retrieves a list of the provided character ID's corporation roles
//! - [`CharacterApi::get_standings`]: Retrieves a paginated list of NPC standing entries for the provided character ID
//! - [`CharacterApi::get_character_corporation_titles`]: Retrieves a list of the provided character ID's corporation titles

use crate::error::Error;
use crate::model::universe::Standing;
use crate::oauth2::scope::CharacterScopes;
use crate::{Client, ScopeBuilder};

use crate::model::asset::Blueprint;
use crate::model::character::{
    Character, CharacterAffiliation, CharacterCorporationHistory, CharacterCorporationRole,
    CharacterCorporationTitle, CharacterJumpFatigue, CharacterMedal,
    CharacterNewContactNotification, CharacterNotification, CharacterPortraits,
    CharacterResearchAgent,
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
        /// - `character_ids` (Vec<[`i64`]>): A vec of character IDs to retrieve affiliations for.
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - Vec<[`CharacterAffiliation`]>: The affiliations of the characters if successfully retrieved
        /// - [`Error`]: An error if the fetch request fails
        pub_post character_affiliation(
            character_ids: Vec<i64>,
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
        /// - `character_id` (`i64`): The ID of the character to retrieve blueprints for
        /// - `page`         (`i32`): The page of blueprints to retrieve, page numbers start at `1`
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
        /// Calculates CSPA cost for evemailing a list of characters with the provided character ID
        ///
        /// This ESI route is used to calculate the CSPA cost for a list of characters based upon the
        /// contacts of the provided character ID which could affect the cost based upon standing.
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <developers.eveonline.com/api-explorer#/operations/PostCharactersCharacterIdCspa>
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format
        /// - `character_ids` (`Vec<i64>`): List of character IDs to calculate the CSPA cost to
        ///   evemail.
        /// - `character_id` (`i64`): ID of the character who would be sending the evemails
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `f64`: An f64 representing the total cost to evemail the list of characters
        /// - [`Error`]: An error if the fetch request fails
        auth_post calculate_a_cspa_charge_cost(
            access_token: &str,
            character_ids: Vec<i64>,
            character_id: i64
        ) -> Result<f64, Error>
        url = "{}/characters/{}/cspa";
        label = "CSPA charge cost";
        required_scopes = ScopeBuilder::new().character(CharacterScopes::new().read_contacts()).build();
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
        /// Retrieves a list of character's notifications about being added to someone's contact list
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdNotificationsContacts>
        ///
        /// # Required Scopes
        /// - [`CharacterScopes::read_notifications`](crate::oauth2::scope::CharacterScopes::read_notifications):
        ///   `esi-characters.read_notifications.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve added as contact notifications
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CharacterNewContactNotification`]`>`: A list of character's notifications about being added to
        ///   someone's contact list
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_new_contact_notifications(
            access_token: &str,
            character_id: i64
        ) -> Result<Vec<CharacterNewContactNotification>, Error>
        url = "{}/characters/{}/notifications/contacts";
        label = "new contact notifications";
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
        /// - [`CharacterCorporationRole`]: Struct containing entries for the provided character ID's corporation roles.
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_character_corporation_roles(
            access_token: &str,
            character_id: i64
        ) -> Result<CharacterCorporationRole, Error>
        url = "{}/characters/{}/roles";
        label = "corporation roles";
        required_scopes = ScopeBuilder::new().character(CharacterScopes::new().read_corporation_roles()).build();
    }

    define_endpoint! {
        /// Retrieves a paginated list of NPC standing entries for the provided character ID
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
        /// - `Vec<`[`Standing`]`>`: Paginated list of NPC standing entries for the provided character ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_standings(
            access_token: &str,
            character_id: i64
        ) -> Result<Vec<Standing>, Error>
        url = "{}/characters/{}/standings";
        label = "NPC standings";
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
