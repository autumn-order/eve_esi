//! # EVE ESI Clones Endpoints
//!
//! This module provides the [`ClonesEndpoints`] struct and associated methods for accessing
//! clone-related ESI endpoints.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! ## Endpoints (2)

//! ### Authenticated (2)
//!
//! | Endpoint                        | Description                                      |
//! | ------------------------------- | ------------------------------------------------ |
//! | [`ClonesEndpoints::get_clones`] | Get list of clones for the provided character ID |

use crate::{model::clone::CharacterClones, scope::ClonesScopes, Client, Error, ScopeBuilder};

/// Provides methods for accessing clone-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct ClonesEndpoints<'a> {
    client: &'a Client,
}

impl<'a> ClonesEndpoints<'a> {
    /// Creates a new instance of [`ClonesEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    define_endpoint! {
        /// Get list of clones for the provided character ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdClones>
        ///
        /// # Required Scopes
        /// - [`ClonesEndpoints::get_clones`](crate::scope::ClonesEndpoints::get_clones):
        ///   `esi-assets.get_clones.v1`
        ///
        /// # Arguments
        /// - `access_token`  (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id`  (`i64`): The ID of the character to retrieve calendar event attendees for.
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - [`CharacterClones`]: list of clones for the provided character ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_clones(
            access_token: &str,
            character_id: i64
        ) -> Result<CharacterClones, Error>
        url = "{}/characters/{}/clones";
        label = "clones";
        required_scopes = ScopeBuilder::new()
            .clones(ClonesScopes::new().read_clones())
            .build();
    }
}
