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
    model::contacts::{AllianceContact, ContactLabel},
    scope::AlliancesScopes,
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
}
