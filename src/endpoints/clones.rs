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
//!
//! ### Authenticated (2)
//!
//! | Endpoint                                 | Description                                                                   |
//! | ---------------------------------------- | ----------------------------------------------------------------------------- |
//! | [`ClonesEndpoints::get_clones`]          | Get list of clones for the provided character ID                              |
//! | [`ClonesEndpoints::get_active_implants`] | Get list of type IDs of implants for the provided character ID's active clone |

use crate::{
    esi::EsiRequest, model::clones::CharacterClones, scope::ClonesScopes, Client, ScopeBuilder,
};
use reqwest::Method;

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

    define_esi_endpoint! {
        /// Get list of clones for the provided character ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdClones>
        ///
        /// # Required Scopes
        /// - [`ClonesScopes::read_clones`](crate::scope::ClonesScopes::read_clones):
        ///   `esi-clones.read_clones.v1`
        ///
        /// # Arguments
        /// - `access_token`  (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id`  (`i64`): The ID of the character to retrieve clones for
        ///
        /// # Returns
        /// An ESI request builder that returns clone information including home location and jump clones when sent.
        auth fn get_clones(
            access_token: &str,
            character_id: i64
        ) -> EsiRequest<CharacterClones>
        method = Method::GET;
        path = "/characters/{}/clones";
        required_scopes = ScopeBuilder::new()
            .clones(ClonesScopes::new().read_clones())
            .build();
    }

    define_esi_endpoint! {
        /// Get list of type IDs of implants for the provided character ID's active clone
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdImplants>
        ///
        /// # Required Scopes
        /// - [`ClonesScopes::read_implants`](crate::scope::ClonesScopes::read_implants):
        ///   `esi-clones.read_implants.v1`
        ///
        /// # Arguments
        /// - `access_token`  (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id`  (`i64`): The ID of the character to retrieve implants for
        ///
        /// # Returns
        /// An ESI request builder that returns a list of implant type IDs installed in the character's active clone when sent.
        auth fn get_active_implants(
            access_token: &str,
            character_id: i64
        ) -> EsiRequest<Vec<i64>>
        method = Method::GET;
        path = "/characters/{}/implants";
        required_scopes = ScopeBuilder::new()
            .clones(ClonesScopes::new().read_implants())
            .build();
    }
}
