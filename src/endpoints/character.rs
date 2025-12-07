//! # EVE ESI Character Endpoints
//!
//! This module provides the [`CharacterEndpoints`] struct and associated methods for accessing
//! character-related ESI endpoints.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! ## Endpoints (11)
//! ### Public (3)
//! |                       Endpoint                           |                               Description                                 |
//! | -------------------------------------------------------- | ------------------------------------------------------------------------- |
//! | [`CharacterEndpoints::get_character_public_information`] | Retrieves the public information of a specific character                  |
//! | [`CharacterEndpoints::get_corporation_history`]          | Retrieves the public corporation history of the provided character ID     |
//! | [`CharacterEndpoints::get_character_portraits`]          | Retrieves the image URLs of a chacter's portraits with various dimensions |
//!
//! ### Authenticated (9)
//! |                         Endpoint                         |                                          Description                                            |
//! | -------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
//! | [`CharacterEndpoints::get_agents_research`]              | Retrieves character's research agents using the character's ID                                  |
//! | [`CharacterEndpoints::get_blueprints`]                   | Retrieves character's blueprints using the character's ID & page to fetch of the blueprint list |
//! | [`CharacterEndpoints::calculate_a_cspa_charge_cost`]     | Calculates CSPA cost for evemailing a list of characters with the provided character ID         |
//! | [`CharacterEndpoints::get_jump_fatigue`]                 | Retrieves jump fatigue for the provided character's ID                                          |
//! | [`CharacterEndpoints::get_medals`]                       | Retrieves a list of medals for the provided character ID                                        |
//! | [`CharacterEndpoints::get_character_notifications`]      | Retrieves a list of character's notifications                                                   |
//! | [`CharacterEndpoints::get_character_corporation_roles`]  | Retrieves a list of the provided character ID's corporation roles                               |
//! | [`CharacterEndpoints::get_standings`]                    | Retrieves a paginated list of NPC standing entries for the provided character ID                |
//! | [`CharacterEndpoints::get_character_corporation_titles`] | Retrieves a list of the provided character ID's corporation titles                              |

use crate::esi::EsiRequest;
use crate::model::standing::Standing;
use crate::scope::CharactersScopes;
use crate::{Client, ScopeBuilder};

use crate::model::asset::Blueprint;
use crate::model::character::{
    Character, CharacterAffiliation, CharacterCorporationHistory, CharacterCorporationRole,
    CharacterCorporationTitle, CharacterJumpFatigue, CharacterMedal,
    CharacterNewContactNotification, CharacterNotification, CharacterPortraits,
    CharacterResearchAgent,
};
use reqwest::Method;

/// Provides methods for accessing character-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct CharacterEndpoints<'a> {
    client: &'a Client,
}

impl<'a> CharacterEndpoints<'a> {
    /// Creates a new instance of [`CharacterEndpoints`].
    ///
    /// For an overview & usage examples, see the [endpoints module documentation](super)e
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    define_esi_endpoint! {
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
        /// An ESI request builder that returns the character's public information when sent.
        pub fn get_character_public_information(
            character_id: i64
        ) -> EsiRequest<Character>
        method = Method::GET;
        path = "/characters/{}";
    }

    define_esi_endpoint! {
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
        /// An ESI request builder that returns a list of character affiliations including corporation and alliance IDs when sent.
        pub fn character_affiliation(
        ) -> EsiRequest<Vec<CharacterAffiliation>>
        method = Method::POST;
        path = "/characters/affiliation";
        body = character_ids: Vec<i64>;
    }

    define_esi_endpoint! {
        /// Retrieves character's research agents using the character's ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdAgentsResearchGet>
        ///
        /// # Required Scopes
        /// - [`CharactersScopes::read_agents_research`](crate::scope::CharactersScopes::read_agents_research):
        ///   `esi-characters.read_agents_research.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve research agent information for.
        ///
        /// # Returns
        /// An ESI request builder that returns a list of the character's research agents when sent.
        auth fn get_agents_research(
            access_token: &str,
            character_id: i64
        ) -> EsiRequest<Vec<CharacterResearchAgent>>
        method = Method::GET;
        path = "/characters/{}/agents_research";
        required_scopes = ScopeBuilder::new()
            .characters(CharactersScopes::new().read_agents_research())
            .build();
    }

    define_esi_endpoint! {
        /// Retrieves character's blueprints using the character's ID & page to fetch of the blueprint list
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdBlueprints>
        ///
        /// # Required Scopes
        /// - [`CharactersScopes::read_blueprints`](crate::scope::CharactersScopes::read_blueprints):
        ///   `esi-characters.read_blueprints.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve blueprints for
        /// - `page`         (`i32`): The page of blueprints to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a paginated list of the character's blueprints when sent.
        auth fn get_blueprints(
            access_token: &str,
            character_id: i64;
            page: i32
        ) -> EsiRequest<Vec<Blueprint>>
        method = Method::GET;
        path = "/characters/{}/blueprints";
        required_scopes = ScopeBuilder::new().characters(CharactersScopes::new().read_blueprints()).build();
    }

    define_esi_endpoint! {
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
        /// An ESI request builder that returns the character's corporation history when sent.
        pub fn get_corporation_history(
            character_id: i64
        ) -> EsiRequest<Vec<CharacterCorporationHistory>>
        method = Method::GET;
        path = "/characters/{}/corporationhistory";
    }

    define_esi_endpoint! {
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
        /// - `character_id` (`i64`): ID of the character who would be sending the evemails
        /// - `character_ids` (`Vec<i64>`): List of character IDs to calculate the CSPA cost to
        ///   evemail.
        ///
        /// # Returns
        /// An ESI request builder that returns the calculated CSPA charge cost for evemailing the provided characters when sent.
        auth fn calculate_a_cspa_charge_cost(
            access_token: &str,
            character_id: i64
        ) -> EsiRequest<f64>
        method = Method::POST;
        path = "/characters/{}/cspa";
        required_scopes = ScopeBuilder::new().characters(CharactersScopes::new().read_contacts()).build();
        body = character_ids: Vec<i64>;
    }

    define_esi_endpoint! {
        /// Retrieves jump fatigue for the provided character's ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdFatigue>
        ///
        /// # Required Scopes
        /// - [`CharactersScopes::read_fatigue`](crate::scope::CharactersScopes::read_fatigue):
        ///   `esi-characters.read_fatigue.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve jump fatigue for
        ///
        /// # Returns
        /// An ESI request builder that returns the character's jump fatigue information when sent.
        auth fn get_jump_fatigue(
            access_token: &str,
            character_id: i64
        ) -> EsiRequest<CharacterJumpFatigue>
        method = Method::GET;
        path = "/characters/{}/fatigue";
        required_scopes = ScopeBuilder::new().characters(CharactersScopes::new().read_fatigue()).build();
    }

    define_esi_endpoint! {
        /// Retrieves a list of medals for the provided character ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdMedals>
        ///
        /// # Required Scopes
        /// - [`CharactersScopes::read_medals`](crate::scope::CharactersScopes::read_medals):
        ///   `esi-characters.read_medals.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve medals for
        ///
        /// # Returns
        /// An ESI request builder that returns a list of medals awarded to the character when sent.
        auth fn get_medals(
            access_token: &str,
            character_id: i64
        ) -> EsiRequest<Vec<CharacterMedal>>
        method = Method::GET;
        path = "/characters/{}/medals";
        required_scopes = ScopeBuilder::new().characters(CharactersScopes::new().read_medals()).build();
    }

    define_esi_endpoint! {
        /// Retrieves a list of character's notifications
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdNotifications>
        ///
        /// # Required Scopes
        /// - [`CharactersScopes::read_notifications`](crate::scope::CharactersScopes::read_notifications):
        ///   `esi-characters.read_notifications.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve notifications for
        ///
        /// # Returns
        /// An ESI request builder that returns a list of the character's notifications when sent.
        auth fn get_character_notifications(
            access_token: &str,
            character_id: i64
        ) -> EsiRequest<Vec<CharacterNotification>>
        method = Method::GET;
        path = "/characters/{}/notifications";
        required_scopes = ScopeBuilder::new().characters(CharactersScopes::new().read_notifications()).build();
    }

    define_esi_endpoint! {
        /// Retrieves a list of character's notifications about being added to someone's contact list
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdNotificationsContacts>
        ///
        /// # Required Scopes
        /// - [`CharactersScopes::read_notifications`](crate::scope::CharactersScopes::read_notifications):
        ///   `esi-characters.read_notifications.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve added as contact notifications
        ///
        /// # Returns
        /// An ESI request builder that returns a list of notifications about being added to someone's contact list when sent.
        auth fn get_new_contact_notifications(
            access_token: &str,
            character_id: i64
        ) -> EsiRequest<Vec<CharacterNewContactNotification>>
        method = Method::GET;
        path = "/characters/{}/notifications/contacts";
        required_scopes = ScopeBuilder::new().characters(CharactersScopes::new().read_notifications()).build();
    }

    define_esi_endpoint! {
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
        /// An ESI request builder that returns the character's portrait image URLs in various sizes when sent.
        pub fn get_character_portraits(
            character_id: i64
        ) -> EsiRequest<CharacterPortraits>
        method = Method::GET;
        path = "/characters/{}/portrait";
    }

    define_esi_endpoint! {
        /// Retrieves a list of the provided character ID's corporation roles
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdRoles>
        ///
        /// # Required Scopes
        /// - [`CharactersScopes::read_corporation_roles`](crate::scope::CharactersScopes::read_corporation_roles):
        ///   `esi-characters.read_corporation_roles.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve corporation roles for
        ///
        /// # Returns
        /// An ESI request builder that returns the character's corporation roles when sent.
        auth fn get_character_corporation_roles(
            access_token: &str,
            character_id: i64
        ) -> EsiRequest<CharacterCorporationRole>
        method = Method::GET;
        path = "/characters/{}/roles";
        required_scopes = ScopeBuilder::new().characters(CharactersScopes::new().read_corporation_roles()).build();
    }

    define_esi_endpoint! {
        /// Retrieves a paginated list of NPC standing entries for the provided character ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdStandings>
        ///
        /// # Required Scopes
        /// - [`CharactersScopes::read_standings`](crate::scope::CharactersScopes::read_standings):
        ///   `esi-characters.read_standings.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve standings for
        ///
        /// # Returns
        /// An ESI request builder that returns a list of the character's NPC standings when sent.
        auth fn get_standings(
            access_token: &str,
            character_id: i64
        ) -> EsiRequest<Vec<Standing>>
        method = Method::GET;
        path = "/characters/{}/standings";
        required_scopes = ScopeBuilder::new().characters(CharactersScopes::new().read_standings()).build();
    }

    define_esi_endpoint! {
        /// Retrieves a list of the provided character ID's corporation titles
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdTitles>
        ///
        /// # Required Scopes
        /// - [`CharactersScopes::read_titles`](crate::scope::CharactersScopes::read_titles):
        ///   `esi-characters.read_titles.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve corporation titles for
        ///
        /// # Returns
        /// An ESI request builder that returns a list of the character's corporation titles when sent.
        auth fn get_character_corporation_titles(
            access_token: &str,
            character_id: i64
        ) -> EsiRequest<Vec<CharacterCorporationTitle>>
        method = Method::GET;
        path = "/characters/{}/titles";
        required_scopes = ScopeBuilder::new().characters(CharactersScopes::new().read_titles()).build();
    }
}
