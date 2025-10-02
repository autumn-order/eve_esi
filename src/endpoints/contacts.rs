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

    define_endpoint! {
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
        /// - `character_id`  (`i64`): The ID of the alliance to retrieve contacts labels for
        /// - `contact_ids`   (`Vec<i64>`): List of contact IDs to delete (up to 20 per request)
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `()`: No error if request was successful
        /// - [`Error`]: An error if the fetch request fails
        auth_delete delete_contacts(
            access_token: &str,
            character_id: i64;
            contact_ids: Vec<i64>
        ) -> Result<(), Error>
        url = "{}/characters/{}/contacts";
        label = "delete contacts";
        required_scopes = ScopeBuilder::new()
            .characters(CharactersScopes::new().write_contacts())
            .build();
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

    define_endpoint! {
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
        /// - `character_id`     (`i64`): The ID of the character to add contacts for
        /// - `standing`         (`f64`): The standing to set for the provided contact IDs
        /// - `label_ids`   (`Vec<i64>`): List of label IDs to set for the contacts (Use an empty Vec if none)
        /// - `watched`         (`bool`): Bool indicating whether or not to add contacts to buddy list (will only
        ///   be applied to characters)
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<i64>`: List of IDs of the created contacts
        /// - [`Error`]: An error if the fetch request fails
        auth_post add_contacts(
            access_token: &str,
            contact_ids: Vec<i64>,
            character_id: i64;
            standing: f64,
            label_ids: Vec<i64>,
            watched: bool,
        ) -> Result<Vec<i64>, Error>
        url = "{}/characters/{}/contacts";
        label = "add contacts";
        required_scopes =  ScopeBuilder::new()
            .characters(CharactersScopes::new().write_contacts())
            .build();
    }

    define_endpoint! {

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
        /// - `character_id`     (`i64`): The ID of the character to edit contacts for
        /// - `standing`         (`f64`): The standing to set for the provided contact IDs
        /// - `label_ids`   (`Vec<i64>`): List of label IDs to set for the contacts (Use an empty Vec if none)
        /// - `watched`         (`bool`): Bool indicating whether or not to add contacts to buddy list (will only
        ///   be applied to characters)
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<i64>`: List of IDs of the edited contacts
        /// - [`Error`]: An error if the fetch request fails
        auth_put edit_contacts(
            access_token: &str,
            contact_ids: Vec<i64>,
            character_id: i64;
            standing: f64,
            label_ids: Vec<i64>,
            watched: bool,
        ) -> Result<Vec<i64>, Error>
        url = "{}/characters/{}/contacts";
        label = "edit contacts";
        required_scopes =  ScopeBuilder::new()
            .characters(CharactersScopes::new().write_contacts())
            .build();
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
