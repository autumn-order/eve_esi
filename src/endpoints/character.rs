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
    pub async fn get_character_public_information(
        &self,
        character_id: i64,
    ) -> Result<Character, Error> {
        let url = format!("{}/characters/{}/", self.client.inner.esi_url, character_id);

        debug!(
            "Fetching character information for character ID {} from \"{}\"",
            character_id, url
        );

        let start_time = Instant::now();

        // Fetch character information from ESI
        let result = self
            .client
            .esi()
            .get_from_public_esi::<Character>(&url)
            .await;

        let elapsed = start_time.elapsed();
        match result {
            Ok(character) => {
                info!(
                    "Successfully fetched character information for character ID: {} (took {}ms)",
                    character_id,
                    elapsed.as_millis()
                );

                Ok(character)
            }
            Err(err) => {
                error!(
                    "Failed to fetch character information for character ID {} after {}ms due to error: {:#?}",
                    character_id,
                    elapsed.as_millis(),
                    err);

                Err(err.into())
            }
        }
    }

    /// Retrieve affiliations for a list of characters
    ///
    /// For an overview & usage examples, see the [endpoints module documentation](super)
    ///
    /// # ESI Documentation
    /// - <https://developers.eveonline.com/api-explorer#/operations/PostCharactersAffiliation>
    ///
    /// # Arguments
    /// - `character_ids` (Vec<[`i64`]>): A list of character IDs to retrieve affiliations for.
    ///
    /// # Returns
    /// Returns a [`Result`] containing either:
    /// - Vec<[`CharacterAffiliation`]>: The affiliations of the characters if successfully retrieved
    /// - [`Error`]: An error if the fetch request fails
    pub async fn character_affiliation(
        &self,
        character_ids: Vec<i64>,
    ) -> Result<Vec<CharacterAffiliation>, Error> {
        let url = format!("{}/characters/affiliation/", self.client.inner.esi_url);

        debug!(
            "Fetching character affiliations for {} characters from \"{}\"",
            character_ids.len(),
            url
        );

        let start_time = Instant::now();

        // Fetch character affiliations from ESI
        let result = self
            .client
            .esi()
            .post_to_public_esi::<Vec<CharacterAffiliation>, Vec<i64>>(&url, &character_ids)
            .await;

        let elapsed = start_time.elapsed();
        match result {
            Ok(affiliations) => {
                info!(
                    "Successfully fetched character affiliations for {} character(s) (took {}ms)",
                    elapsed.as_millis(),
                    character_ids.len()
                );

                Ok(affiliations)
            }
            Err(err) => {
                error!(                    "Failed to fetch character affiliations for {} character(s) after {}ms due to error: {:#?}",
                character_ids.len(),
                elapsed.as_millis(),
                err);

                Err(err.into())
            }
        }
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

    /// Retrieves the public corporation history of the provided character ID
    ///
    /// For an overview & usage examples, see the [endpoints module documentation](super)
    ///
    /// # ESI Documentation
    /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterId>
    ///
    /// # Arguments
    /// - `character_id` (`i64`): The ID of the character to retrieve corporation history for.
    ///
    /// # Returns
    /// Returns a [`Result`] containing either:
    /// - [`CharacterCorporationHistory`]: The character's corporation history if request is successful
    /// - [`Error`]: An error if the fetch request fails
    pub async fn get_corporation_history(
        &self,
        character_id: i64,
    ) -> Result<Vec<CharacterCorporationHistory>, Error> {
        let url = format!(
            "{}/characters/{}/corporationhistory",
            self.client.inner.esi_url, character_id
        );

        debug!(
            "Fetching character corporation history for character ID {} from \"{}\"",
            character_id, url
        );

        let start_time = Instant::now();

        // Fetch character information from ESI
        let result = self
            .client
            .esi()
            .get_from_public_esi::<Vec<CharacterCorporationHistory>>(&url)
            .await;

        let elapsed = start_time.elapsed();
        match result {
            Ok(corporation_history) => {
                info!(
                    "Successfully fetched character corporation history with {} entries for character ID: {} (took {}ms)",
                    corporation_history.len(),
                    character_id,
                    elapsed.as_millis()
                );

                Ok(corporation_history)
            }
            Err(err) => {
                error!(
                    "Failed to fetch character corporation history for character ID {} after {}ms due to error: {:#?}",
                    character_id,
                    elapsed.as_millis(),
                    err);

                Err(err.into())
            }
        }
    }
}
