//! # EVE ESI Universe Endpoints
//!
//! This module provides the [`UniverseEndpoints`] struct and associated methods for accessing
//! universe-related ESI endpoints.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! ## Endpoints (1)
//!
//! ### Public (1)
//!
//! | Endpoint                                 | Description                                                                   |
//! | ---------------------------------------- | ----------------------------------------------------------------------------- |
//! | [`UniverseEndpoints::get_factions`]      | Retrieves a list of information for all NPC factions in EVE Online            |

use crate::{model::universe::Faction, Client, Error};

/// Provides methods for accessing universe-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct UniverseEndpoints<'a> {
    client: &'a Client,
}

impl<'a> UniverseEndpoints<'a> {
    /// Creates a new instance of [`UniverseEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    define_endpoint! {
        /// Retrieves a list of information for all NPC factions in EVE Online
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetUniverseFactions>
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - Vec<[`Faction`]>: List of information for all NPC factions in EVE Online
        /// - [`Error`]: An error if the fetch request failed
        pub_get get_factions() -> Result<Vec<Faction>, Error>
        url = "{}/universe/factions";
        label = "list of all NPC faction information";
    }
}
