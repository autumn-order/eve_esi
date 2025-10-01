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
//! ## Endpoints (9)
//!
//! ### Authenticated (9)
//!
//! | Endpoint                                              | Description                                                |
//! | ----------------------------------------------------- | ---------------------------------------------------------- |
//! | [`ContactsEndpoints::get_alliance_contacts`]          | Get list of contacts for the provided alliance ID          |
//! | [`ContactsEndpoints::get_alliance_contact_labels`]    | Get list of contact labels for the provided alliance ID    |
//! | [`ContactsEndpoints::delete_contacts`]                | Delete list of contacts by ID for provided character ID    |
//! | [`ContactsEndpoints::get_contacts`]                   | Get list of contacts for the provided character ID         |
//! | [`ContactsEndpoints::add_contacts`]                   | Add list of contact IDs for the provided character ID      |
//! | [`ContactsEndpoints::edit_contacts`]                  | Edit list of contact IDs for the provided character ID     |
//! | [`ContactsEndpoints::get_contact_labels`]             | Get list of contact labels for the provided character ID   |
//! | [`ContactsEndpoints::get_corporation_contacts`]       | Get list of contacts for the provided corporation ID       |
//! | [`ContactsEndpoints::get_corporation_contact_labels`] | Get list of contact labels for the provided corporation ID |

use crate::{
    model::contacts::{AllianceContact, CharacterContact, ContactLabel, CorporationContact},
    scope::{AlliancesScopes, CharactersScopes, CorporationsScopes},
    Client, Error, ScopeBuilder,
};

use url::form_urlencoded::Serializer;
use url::Url;

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
        let contact_array_string = format!(
            "[{}]",
            contact_ids
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        let mut url = Url::parse(&format!(
            "{}/characters/{}/contacts",
            self.client.inner.esi_url, character_id
        ))?;

        {
            let mut ser = Serializer::new(String::new());
            ser.append_pair("contact_ids", &contact_array_string);
            url.set_query(Some(&ser.finish()));
        }

        let required_scopes = ScopeBuilder::new()
            .characters(CharactersScopes::new().write_contacts())
            .build();

        let esi = self.client.esi();
        let api_call =
            esi.delete_from_authenticated_esi::<()>(url.as_str(), &access_token, required_scopes);

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

    /// Add list of contact IDs for the provided character ID
    ///
    /// For an overview & usage examples, see the [endpoints module documentation](super)
    ///
    /// # ESI Documentation
    /// - <https://developers.eveonline.com/api-explorer#/operations/PostCharactersCharacterIdContacts>
    ///
    /// # Required Scopes
    /// - [`CharactersScopes::write_contacts`](crate::scope::CharactersScopes::write_contacts):
    ///   `esi-characters.write_contacts.v1`
    ///
    /// # Arguments
    /// - `access_token`    (`&str`): Access token used for authenticated ESI routes in string format.
    /// - `contact_ids` (`Vec<i64>`): List of contact IDs to add for the provided character ID
    /// - `standing`         (`f64`): The standing to set for the provided contact IDs
    /// - `label_ids`   (`Vec<i64>`): List of label IDs to set for the contacts (Use an empty Vec if none)
    /// - `watched`         (`bool`): Bool indicating whether or not to add contacts to buddy list (will only
    ///                               be applied to characters)
    /// - `character_id`     (`i64`): The ID of the character to add contacts for
    ///
    /// # Returns
    /// Returns a [`Result`] containing either:
    /// - `Vec<i64>`: List of IDs of the created contacts
    /// - [`Error`]: An error if the fetch request fails
    pub async fn add_contacts(
        &self,
        access_token: &str,
        contact_ids: Vec<i64>,
        standing: f64,
        label_ids: Vec<i64>,
        watched: bool,
        character_id: i64,
    ) -> Result<Vec<i64>, Error> {
        let label_array_string = format!(
            "[{}]",
            label_ids
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        let mut url = Url::parse(&format!(
            "{}/characters/{}/contacts",
            self.client.inner.esi_url, character_id
        ))?;

        {
            let mut ser = Serializer::new(String::new());
            ser.append_pair("standing", &standing.to_string());
            ser.append_pair("label_ids", &label_array_string);
            ser.append_pair("watched", &watched.to_string());
            url.set_query(Some(&ser.finish()));
        }

        let required_scopes = ScopeBuilder::new()
            .characters(CharactersScopes::new().write_contacts())
            .build();

        let esi = self.client.esi();
        let api_call = esi.post_to_authenticated_esi::<Vec<i64>, Vec<i64>>(
            url.as_str(),
            &contact_ids,
            &access_token,
            required_scopes,
        );

        esi_common_impl!("add contacts", url, api_call, (character_id))
    }

    /// Edit list of contact IDs for the provided character ID
    ///
    /// For an overview & usage examples, see the [endpoints module documentation](super)
    ///
    /// # ESI Documentation
    /// - <https://developers.eveonline.com/api-explorer#/operations/PutCharactersCharacterIdContacts>
    ///
    /// # Required Scopes
    /// - [`CharactersScopes::write_contacts`](crate::scope::CharactersScopes::write_contacts):
    ///   `esi-characters.write_contacts.v1`
    ///
    /// # Arguments
    /// - `access_token`    (`&str`): Access token used for authenticated ESI routes in string format.
    /// - `contact_ids` (`Vec<i64>`): List of contact IDs to edit for the provided character ID
    /// - `standing`         (`f64`): The standing to set for the provided contact IDs
    /// - `label_ids`   (`Vec<i64>`): List of label IDs to set for the contacts (Use an empty Vec if none)
    /// - `watched`         (`bool`): Bool indicating whether or not to add contacts to buddy list (will only
    ///                               be applied to characters)
    /// - `character_id`     (`i64`): The ID of the character to edit contacts for
    ///
    /// # Returns
    /// Returns a [`Result`] containing either:
    /// - `Vec<i64>`: List of IDs of the edited contacts
    /// - [`Error`]: An error if the fetch request fails
    pub async fn edit_contacts(
        &self,
        access_token: &str,
        contact_ids: Vec<i64>,
        standing: f64,
        label_ids: Vec<i64>,
        watched: bool,
        character_id: i64,
    ) -> Result<Vec<i64>, Error> {
        let label_array_string = format!(
            "[{}]",
            label_ids
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        let mut url = Url::parse(&format!(
            "{}/characters/{}/contacts",
            self.client.inner.esi_url, character_id
        ))?;

        {
            let mut ser = Serializer::new(String::new());
            ser.append_pair("standing", &standing.to_string());
            ser.append_pair("label_ids", &label_array_string);
            ser.append_pair("watched", &watched.to_string());
            url.set_query(Some(&ser.finish()));
        }

        let required_scopes = ScopeBuilder::new()
            .characters(CharactersScopes::new().write_contacts())
            .build();

        let esi = self.client.esi();
        let api_call = esi.put_to_authenticated_esi::<Vec<i64>, Vec<i64>>(
            url.as_str(),
            &contact_ids,
            &access_token,
            required_scopes,
        );

        esi_common_impl!("add contacts", url, api_call, (character_id))
    }

    define_endpoint! {
        /// Get list of contact labels for the provided character ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdContactsLabels>
        ///
        /// # Required Scopes
        /// - [`CharactersScopes::read_contacts`](crate::scope::CharactersScopes::read_contacts):
        ///   `esi-characters.read_contacts.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id`  (`i64`): The ID of the character to retrieve contacts labels for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`ContactLabel`]`>`: list of contact labels for the provided character ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_contact_labels(
            access_token: &str,
            character_id: i64
        ) -> Result<Vec<ContactLabel>, Error>
        url = "{}/characters/{}/contacts/labels";
        label = "contact labels";
        required_scopes = ScopeBuilder::new()
            .characters(CharactersScopes::new().read_contacts())
            .build();
    }

    define_endpoint! {
        /// Get list of contacts for the provided corporation ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdContacts>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_contacts`](crate::scope::CorporationsScopes::read_contacts):
        ///   `esi-alliances.read_contacts.v1`
        ///
        /// # Arguments
        /// - `access_token`  (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve contacts for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`CorporationContact`]`>`: list of contacts for the provided corporation ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_contacts(
            access_token: &str,
            corporation_id: i64
        ) -> Result<Vec<CorporationContact>, Error>
        url = "{}/corporations/{}/contacts";
        label = "contacts";
        required_scopes = ScopeBuilder::new()
            .corporations(CorporationsScopes::new().read_contacts())
            .build();
    }

    define_endpoint! {
        /// Get list of contact labels for the provided corporation ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdContactsLabels>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_contacts`](crate::scope::CorporationsScopes::read_contacts):
        ///   `esi-corporations.read_contacts.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id` (`i64`): The ID of the corporation to retrieve contacts labels for
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`ContactLabel`]`>`: list of contact labels for the provided corporation ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_contact_labels(
            access_token: &str,
            corporation_id: i64
        ) -> Result<Vec<ContactLabel>, Error>
        url = "{}/corporations/{}/contacts/labels";
        label = "contact labels";
        required_scopes = ScopeBuilder::new()
            .corporations(CorporationsScopes::new().read_contacts())
            .build();
    }
}
