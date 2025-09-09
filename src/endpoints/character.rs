//! Character Endpoints for EVE Online's ESI API.
//!
//! This module provides the [`CharacterApi`] struct and associated methods for accessing
//! character-related endpoints of the EVE Online ESI (EVE Stable Infrastructure) API.
//!
//! # Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! # Methods
//! - [`CharacterApi::get_character_public_information`]: Retrieves the public information of a specific character
//! - [`CharacterApi::character_affiliation`]: Retrieve affiliations for a list of characters
//!
//! # Usage Example
//! ```no_run
//! #[tokio::main]
//! async fn main() {
//!     let esi_client = eve_esi::Client::builder()
//!         .user_agent("MyApp/1.0 (contact@example.com)")
//!         .build()
//!         .expect("Failed to build Client");
//!
//!     // Get public information for a character
//!     let character = esi_client.character().get_character_public_information(2114794365).await.unwrap();
//!     println!("Character name: {}", character.name);
//! }
//! ```

use std::time::Instant;

use log::{debug, error, info};

use crate::error::Error;
use crate::Client;

use crate::model::character::{Character, CharacterAffiliation};

/// Provides methods for accessing character-related endpoints of the EVE Online ESI API.
///
/// The `CharacterApi` struct acts as an interface for retrieving information about EVE Online characters
/// using the ESI API. It requires an [`Client`] for making HTTP requests to the ESI endpoints.
///
/// See the [module-level documentation](self) for an overview and usage example.
pub struct CharacterApi<'a> {
    client: &'a Client,
}

impl<'a> CharacterApi<'a> {
    /// Creates a new instance of `CharacterApi`.
    ///
    /// # Arguments
    /// - `client` - The [`Client`] used for making HTTP requests to the ESI endpoints.
    ///
    /// # Returns
    /// Returns a new instance of `CharacterApi`.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves the public information of a specific character
    ///
    /// # Documentation
    /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterId>
    ///
    /// # Arguments
    /// - `character_id` (`Vec<`[`i32`]`>`): The ID of the character to retrieve information for.
    ///
    /// # Returns
    /// Returns a `Result` containing either:
    /// - [`Character`]: The character's information if successfully retrieved
    /// - [`Error`]: An error if the fetch request fails
    pub async fn get_character_public_information(
        &self,
        character_id: i32,
    ) -> Result<Character, Error> {
        let url = format!("{}/characters/{}/", self.client.inner.esi_url, character_id);

        let message = format!(
            "Fetching character information for character ID {} from {}",
            character_id, url
        );

        debug!("{}", message);

        let start_time = Instant::now();

        // Fetch character information from ESI
        let result = self.client.get_from_public_esi::<Character>(&url).await;

        let elapsed = start_time.elapsed();
        match result {
            Ok(character) => {
                let message = format!(
                    "Successfully fetched character information for character ID: {} (took {}ms)",
                    character_id,
                    elapsed.as_millis()
                );

                info!("{}", message);

                Ok(character)
            }
            Err(err) => {
                let message = format!(
                    "Failed to fetch character information for character ID {} after {}ms due to error: {:#?}",
                        character_id,
                        elapsed.as_millis(),
                        err
                );

                error!("{}", message);

                Err(err.into())
            }
        }
    }

    /// Retrieve affiliations for a list of characters
    ///
    /// # Documentation
    /// - <https://developers.eveonline.com/api-explorer#/operations/PostCharactersAffiliation>
    ///
    /// # Arguments
    /// - `character_ids` (`Vec<`[`i32`]`): A list of character IDs to retrieve affiliations for.
    ///
    /// # Returns
    /// Returns a `Result` containing either:
    /// - `Vec<`[`CharacterAffiliation`]`>`: The affiliations of the characters if successfully retrieved
    /// - [`Error`]: An error if the fetch request fails
    pub async fn character_affiliation(
        &self,
        character_ids: Vec<i32>,
    ) -> Result<Vec<CharacterAffiliation>, Error> {
        let url = format!("{}/characters/affiliation/", self.client.inner.esi_url);

        let message = format!(
            "Fetching character affiliations for {} characters from {}",
            character_ids.len(),
            url
        );

        debug!("{}", message);

        let start_time = Instant::now();

        // Fetch character affiliations from ESI
        let result = self
            .client
            .post_to_public_esi::<Vec<CharacterAffiliation>, Vec<i32>>(&url, &character_ids)
            .await;

        let elapsed = start_time.elapsed();
        match result {
            Ok(affiliations) => {
                let message = format!(
                    "Successfully fetched character affiliations for {} character(s) (took {}ms)",
                    elapsed.as_millis(),
                    character_ids.len()
                );

                info!("{}", message);

                Ok(affiliations)
            }
            Err(err) => {
                let message = format!(
                    "Failed to fetch character affiliations for {} character(s) after {}ms due to error: {:#?}",
                    character_ids.len(),
                    elapsed.as_millis(),
                    err
                );

                error!("{}", message);

                Err(err.into())
            }
        }
    }
}
