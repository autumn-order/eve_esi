//! # EVE ESI Contacts Endpoints
//!
//! This module provides the [`ContactsEndpoints`] struct and associated methods for accessing
//! contact-related ESI endpoints.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! ## Endpoints (0)
//! ### Public (0)
//!
//! | Endpoint | Description |
//! | -------- | ----------- |
//! |          |             |
//!
//! ### Authenticated (0)
//!
//! | Endpoint | Description |
//! | -------- | ----------- |
//! |          |             |

use crate::{
    model::contacts::{AllianceContact, CharacterContact, ContactLabel},
    scope::{AlliancesScopes, CharactersScopes},
    Client, Error, ScopeBuilder,
};

/// Provides methods for accessing contact-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct ContactsEndpoints<'a> {
    client: &'a Client,
}

impl<'a> ContactsEndpoints<'a> {
    /// Creates a new instance of [`ContactsEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    define_endpoint! {
        /// Get list of contacts for the provided alliance ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetAlliancesAllianceIdContacts>
        ///
        /// # Required Scopes
        /// - [`AlliancesScopes::read_contacts`](crate::scope::AlliancesScopes::read_contacts):
        ///   `esi-alliances.read_contacts.v1`
        ///
        /// # Arguments
        /// - `access_token`  (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `alliance_id`  (`i64`): The ID of the alliance to retrieve contacts for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`AllianceContact`]`>`: list of contacts for the provided alliance ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_alliance_contacts(
            access_token: &str,
            alliance_id: i64
        ) -> Result<Vec<AllianceContact>, Error>
        url = "{}/alliances/{}/contacts";
        label = "contacts";
        required_scopes = ScopeBuilder::new()
            .alliances(AlliancesScopes::new().read_contacts())
            .build();
    }

    define_endpoint! {
        /// Get list of contact labels for the provided alliance ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/schemas/AlliancesAllianceIdContactsLabelsGet>
        ///
        /// # Required Scopes
        /// - [`AlliancesScopes::read_contacts`](crate::scope::AlliancesScopes::read_contacts):
        ///   `esi-alliances.read_contacts.v1`
        ///
        /// # Arguments
        /// - `access_token`  (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `alliance_id`  (`i64`): The ID of the alliance to retrieve contacts labels for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`ContactLabel`]`>`: list of contact labels for the provided alliance ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_alliance_contact_labels(
            access_token: &str,
            alliance_id: i64
        ) -> Result<Vec<ContactLabel>, Error>
        url = "{}/alliances/{}/contacts/labels";
        label = "contact labels";
        required_scopes = ScopeBuilder::new()
            .alliances(AlliancesScopes::new().read_contacts())
            .build();
    }

    /// Delete list of contacts by ID for provided character ID
    ///
    /// For an overview & usage examples, see the [endpoints module documentation](super)
    ///
    /// # ESI Documentation
    /// - <https://developers.eveonline.com/api-explorer#/operations/DeleteCharactersCharacterIdContacts>
    ///
    /// # Required Scopes
    /// - [`CharactersScopes::write_contacts`](crate::scope::CharactersScopes::write_contacts):
    ///   `esi-characters.write_contacts.v1`
    ///
    /// # Arguments
    /// - `access_token`  (`&str`): Access token used for authenticated ESI routes in string format.
    /// - `contact_ids`   (`Vec<i64>`): List of contact IDs to delete (up to 20 per request)
    /// - `character_id`  (`i64`): The ID of the alliance to retrieve contacts labels for
    ///
    /// # Returns
    /// Returns a [`Result`] containing either:
    /// - `()`: No error if request was successful
    /// - [`Error`]: An error if the fetch request fails
    pub async fn delete_contacts(
        &self,
        access_token: &str,
        contact_ids: Vec<i64>,
        character_id: i64,
    ) -> Result<(), Error> {
        // Can't use auth_delete endpoint macro due to having to convert contact_ids into an array for the URL parameter
        let array_string = format!(
            "[{}]",
            contact_ids
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        let url = format!(
            "{}/characters/{}/contacts?contact_ids={}",
            self.client.inner.esi_url, character_id, array_string
        );
        let required_scopes = ScopeBuilder::new()
            .characters(CharactersScopes::new().write_contacts())
            .build();

        let esi = self.client.esi();
        let api_call =
            esi.delete_from_authenticated_esi::<()>(&url, &access_token, required_scopes);

        esi_common_impl!("delete contacts", url, api_call, (character_id))
    }

    define_endpoint! {
        /// Get list of contacts for the provided character ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdContacts>
        ///
        /// # Required Scopes
        /// - [`CharactersScopes::read_contacts`](crate::scope::CharactersScopes::read_contacts):
        ///   `esi-characters.read_contacts.v1`
        ///
        /// # Arguments
        /// - `access_token`  (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id`  (`i64`): The ID of the character to retrieve contacts for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CharacterContact`]`>`: list of contacts for the provided character ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_contacts(
            access_token: &str,
            character_id: i64
        ) -> Result<Vec<CharacterContact>, Error>
        url = "{}/characters/{}/contacts";
        label = "contacts";
        required_scopes = ScopeBuilder::new()
            .characters(CharactersScopes::new().read_contacts())
            .build();
    }
}
