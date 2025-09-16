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

use std::time::Instant;

use crate::error::Error;
use crate::oauth2::scope::CharacterScopes;
use crate::{Client, ScopeBuilder};

use crate::model::character::{
    Blueprint, Character, CharacterAffiliation, CharacterCorporationHistory, CharacterResearchAgent,
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
        url = "{}/characters/{}/";
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
        url = "{}/characters/affiliation/";
        label = "character affiliation";
    }

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
    /// - `character_id` (`i64`): The ID of the character to retrieve research agent information for.
    /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
    ///
    /// # Returns
    /// Returns a [`Result`] containing either:
    /// - Vec<[`CharacterResearchAgent`]>: A Vec of the character's research agents
    /// - [`Error`]: An error if the fetch request fails
    pub async fn get_agents_research(
        &self,
        character_id: i64,
        access_token: &str,
    ) -> Result<Vec<CharacterResearchAgent>, Error> {
        let url = format!(
            "{}/characters/{}/agents_research",
            self.client.inner.esi_url, character_id
        );
        let required_scopes = ScopeBuilder::new()
            .character(CharacterScopes::new().read_agents_research())
            .build();

        debug!(
            "Fetching research agents for character ID {} from \"{}\"",
            character_id, url
        );

        let start_time = Instant::now();

        // Fetch character research agents from ESI
        let result = self
            .client
            .esi()
            .get_from_authenticated_esi::<Vec<CharacterResearchAgent>>(
                &url,
                access_token,
                required_scopes,
            )
            .await;

        let elapsed = start_time.elapsed();
        match result {
            Ok(research_agents) => {
                info!(
                    "Successfully fetched {} research agents for character ID: {} (took {}ms)",
                    research_agents.len(),
                    character_id,
                    elapsed.as_millis()
                );

                Ok(research_agents)
            }
            Err(err) => {
                error!(                  "Failed to fetch research agents for character ID {} after {}ms due to error: {:#?}",
                    character_id,
                    elapsed.as_millis(),
                    err);

                Err(err.into())
            }
        }
    }

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
    /// - `character_id` (`i64`): The ID of the character to retrieve research agent information for.
    /// - `page`         (`i32`): The page of blueprints to retrieve
    /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
    ///
    /// # Returns
    /// Returns a [`Result`] containing either:
    /// - Vec<[`Blueprint`]>: A Vec of the character's blueprints
    /// - [`Error`]: An error if the fetch request fails
    pub async fn get_blueprints(
        &self,
        character_id: i64,
        page: i32,
        access_token: &str,
    ) -> Result<Vec<Blueprint>, Error> {
        let url = format!(
            "{}/characters/{}/blueprints?page={}",
            self.client.inner.esi_url, character_id, page
        );
        let required_scopes = ScopeBuilder::new()
            .character(CharacterScopes::new().read_blueprints())
            .build();

        debug!(
            "Fetching blueprints for character ID {} from \"{}\"",
            character_id, url
        );

        let start_time = Instant::now();

        let result = self
            .client
            .esi()
            .get_from_authenticated_esi::<Vec<Blueprint>>(&url, access_token, required_scopes)
            .await;

        let elapsed = start_time.elapsed();
        match result {
            Ok(blueprints) => {
                info!(
                    "Successfully fetched {} blueprints for character ID: {} (took {}ms)",
                    blueprints.len(),
                    character_id,
                    elapsed.as_millis()
                );

                Ok(blueprints)
            }
            Err(err) => {
                error!(
                    "Failed to fetch blueprints for character ID {} after {}ms due to error: {:#?}",
                    character_id,
                    elapsed.as_millis(),
                    err
                );

                Err(err.into())
            }
        }
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
        /// - `i64`: The CSPA charge cost for evemailing the character
        /// - [`Error`]: An error if the fetch request fails
        pub_get calculate_a_cspa_charge_cost(
            character_id: i64
        ) -> Result<Vec<CharacterCorporationHistory>, Error>
        url = "{}/characters/{}/cspa";
        label = "CSPA charge cost";
    }
}
