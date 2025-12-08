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
    esi::EsiRequest,
    model::contacts::{AllianceContact, CharacterContact, ContactLabel, CorporationContact},
    scope::{AlliancesScopes, CharactersScopes, CorporationsScopes},
    Client, ScopeBuilder,
};
use reqwest::Method;

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

    define_esi_endpoint! {
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
        /// An ESI request builder that returns a list of alliance contacts when sent.
        auth fn get_alliance_contacts(
            access_token: &str,
            alliance_id: i64
        ) -> EsiRequest<Vec<AllianceContact>>
        method = Method::GET;
        path = "/alliances/{}/contacts";
        required_scopes = ScopeBuilder::new()
            .alliances(AlliancesScopes::new().read_contacts())
            .build();
    }

    define_esi_endpoint! {
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
        /// An ESI request builder that returns a list of contact labels for the alliance when sent.
        auth fn get_alliance_contact_labels(
            access_token: &str,
            alliance_id: i64
        ) -> EsiRequest<Vec<ContactLabel>>
        method = Method::GET;
        path = "/alliances/{}/contacts/labels";
        required_scopes = ScopeBuilder::new()
            .alliances(AlliancesScopes::new().read_contacts())
            .build();
    }

    define_esi_endpoint! {
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
        /// - `character_id`  (`i64`): The ID of the character to delete contacts for
        /// - `contact_ids`   (`Vec<i64>`): List of contact IDs to delete (up to 20 per request)
        ///
        /// # Returns
        /// An ESI request builder that deletes the specified contacts when sent.
        auth fn delete_contacts(
            access_token: &str,
            character_id: i64;
            contact_ids: Vec<i64>
        ) -> EsiRequest<()>
        method = Method::DELETE;
        path = "/characters/{}/contacts";
        required_scopes = ScopeBuilder::new()
            .characters(CharactersScopes::new().write_contacts())
            .build();
    }

    define_esi_endpoint! {
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
        /// An ESI request builder that returns a list of character contacts when sent.
        auth fn get_contacts(
            access_token: &str,
            character_id: i64
        ) -> EsiRequest<Vec<CharacterContact>>
        method = Method::GET;
        path = "/characters/{}/contacts";
        required_scopes = ScopeBuilder::new()
            .characters(CharactersScopes::new().read_contacts())
            .build();
    }

    define_esi_endpoint! {
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
        /// An ESI request builder that adds contacts and returns a list of the created contact IDs when sent.
        auth fn add_contacts(
            access_token: &str,
            character_id: i64;
            standing: f64,
            label_ids: Vec<i64>,
            watched: bool
        ) -> EsiRequest<Vec<i64>>
        method = Method::POST;
        path = "/characters/{}/contacts";
        required_scopes =  ScopeBuilder::new()
            .characters(CharactersScopes::new().write_contacts())
            .build();
        body = contact_ids: Vec<i64>;
    }

    define_esi_endpoint! {
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
        /// An ESI request builder that updates contacts and returns a list of the edited contact IDs when sent.
        auth fn edit_contacts(
            access_token: &str,
            character_id: i64;
            standing: f64,
            label_ids: Vec<i64>,
            watched: bool
        ) -> EsiRequest<Vec<i64>>
        method = Method::PUT;
        path = "/characters/{}/contacts";
        required_scopes =  ScopeBuilder::new()
            .characters(CharactersScopes::new().write_contacts())
            .build();
        body = contact_ids: Vec<i64>;
    }

    define_esi_endpoint! {
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
        /// An ESI request builder that returns a list of contact labels for the character when sent.
        auth fn get_contact_labels(
            access_token: &str,
            character_id: i64
        ) -> EsiRequest<Vec<ContactLabel>>
        method = Method::GET;
        path = "/characters/{}/contacts/labels";
        required_scopes = ScopeBuilder::new()
            .characters(CharactersScopes::new().read_contacts())
            .build();
    }

    define_esi_endpoint! {
        /// Get list of contacts for the provided corporation ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdContacts>
        ///
        /// # Required Scopes
        /// - [`CorporationsScopes::read_contacts`](crate::scope::CorporationsScopes::read_contacts):
        ///   `esi-corporations.read_contacts.v1`
        ///
        /// # Arguments
        /// - `access_token`  (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve contacts for
        ///
        /// # Returns
        /// An ESI request builder that returns a list of corporation contacts when sent.
        auth fn get_corporation_contacts(
            access_token: &str,
            corporation_id: i64
        ) -> EsiRequest<Vec<CorporationContact>>
        method = Method::GET;
        path = "/corporations/{}/contacts";
        required_scopes = ScopeBuilder::new()
            .corporations(CorporationsScopes::new().read_contacts())
            .build();
    }

    define_esi_endpoint! {
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
        /// An ESI request builder that returns a list of contact labels for the corporation when sent.
        auth fn get_corporation_contact_labels(
            access_token: &str,
            corporation_id: i64
        ) -> EsiRequest<Vec<ContactLabel>>
        method = Method::GET;
        path = "/corporations/{}/contacts/labels";
        required_scopes = ScopeBuilder::new()
            .corporations(CorporationsScopes::new().read_contacts())
            .build();
    }
}
