//! Character Endpoints for EVE Online's ESI API.
//!
//! This module provides the [`CharacterApi`] struct and associated methods for accessing
//! character-related endpoints of the EVE Online ESI (EVE Swagger Interface) API.
//!
//! The [`CharacterApi`] acts as a high-level interface for retrieving public information
//! and affiliations for EVE Online characters. It requires an [`Client`] instance
//! to perform HTTP requests to the ESI endpoints.
//!
//! # Features
//! - Fetch public information about a character by character ID
//! - Retrieve affiliations (corporation, alliance, faction) for a list of characters
//!
//! # References
//! - [ESI API Documentation](https://developers.eveonline.com/api-explorer)
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

    /// Retrieves information about a specific character from EVE Online's ESI API.
    ///
    /// This endpoint fetches character information based on the provided character ID.
    /// The endpoint returns data such as the character's name, corporation, alliance_id,
    /// and other relevant information.
    ///
    /// # Arguments
    /// - `character_id` (`Vec<`[`i32`]`>`): The ID of the character to retrieve information for.
    ///
    /// # Returns
    /// Returns a `Result` containing either:
    /// - [`Character`] - The character data if successfully retrieved
    /// - [`Error`] - An error if the request failed (e.g., character not found, network issues)
    ///
    /// # EVE ESI Reference
    /// This endpoint is documented at [EVE ESI Reference](https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterId).
    ///
    /// # Example
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     let esi_client = eve_esi::Client::builder()
    ///         .user_agent("MyApp/1.0 (contact@example.com)")
    ///         .build()
    ///         .expect("Failed to build Client");
    ///
    ///     // Get information about the character Hyziri (id: 2114794365)
    ///     let character = esi_client.character().get_character_public_information(2114794365).await.unwrap();
    ///     println!("Character name: {}", character.name);
    /// }
    /// ```
    pub async fn get_character_public_information(
        &self,
        character_id: i32,
    ) -> Result<Character, Error> {
        let url = format!("{}/characters/{}/", self.client.inner.esi_url, character_id);

        debug!(
            "Fetching character information for character ID {} from {}",
            character_id, url
        );

        let start_time = Instant::now();

        // Fetch character information from ESI
        let result = self.client.get_from_public_esi::<Character>(&url).await;

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
                error!("Failed to fetch character information for character ID {} after {}ms due to error: {:#?}",
                    character_id,
                    elapsed.as_millis(),
                    err
                );

                Err(err.into())
            }
        }
    }

    /// Retrieve affiliations for a list of characters.
    ///
    /// This endpoint returns a list of affiliations for the requested characters.
    /// Each affiliation includes the character's corporation, alliance, and faction IDs.
    ///
    /// # Arguments
    /// - `character_ids` (`Vec<`[`i32`]`): A list of character IDs to retrieve affiliations for.
    ///
    /// # Returns
    /// Returns a `Result` containing either:
    /// - `Vec<`[`CharacterAffiliation`]`>` - The affiliations for the characters if successfully retrieved
    /// - [`Error`] - An error if the request failed (network issues)
    ///
    /// # EVE ESI Reference
    /// This endpoint is documented at [EVE ESI Reference](https://developers.eveonline.com/api-explorer#/operations/PostCharactersAffiliation).
    ///
    /// # Example
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     let esi_client = eve_esi::Client::builder()
    ///         .user_agent("MyApp/1.0 (contact@example.com)")
    ///         .build()
    ///         .expect("Failed to build Client");
    ///
    ///     // Get affiliations for characters with IDs 2114794365 and 2117053828
    ///     let affiliations = esi_client.character().character_affiliation(vec![2114794365, 2117053828]).await.unwrap();
    ///     for affiliation in affiliations {
    ///         let alliance_id = if let Some(alliance_id) = affiliation.alliance_id {
    ///             alliance_id.to_string()
    ///         } else {
    ///             "None".to_string()
    ///         };
    ///
    ///         println!("Character ID: {}, Alliance ID: {}, Corporation ID: {}", affiliation.character_id, alliance_id, affiliation.corporation_id);
    ///     }
    /// }
    pub async fn character_affiliation(
        &self,
        character_ids: Vec<i32>,
    ) -> Result<Vec<CharacterAffiliation>, Error> {
        let url = format!("{}/characters/affiliation/", self.client.inner.esi_url);

        debug!(
            "Fetching character affiliations for {} characters from {}",
            character_ids.len(),
            url
        );

        let start_time = Instant::now();

        // Fetch character affiliations from ESI
        let result = self
            .client
            .post_to_public_esi::<Vec<CharacterAffiliation>, Vec<i32>>(&url, &character_ids)
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
                error!(
                    "Failed to fetch character affiliations for {} character(s) after {}ms due to error: {:#?}",
                    character_ids.len(),
                    elapsed.as_millis(),
                    err
                );

                Err(err.into())
            }
        }
    }
}
