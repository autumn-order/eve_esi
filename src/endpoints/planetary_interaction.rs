//! # EVE ESI Planetary Interaction Endpoints
//!
//! This module provides the [`PlanetaryInteractionEndpoints`] struct and associated methods for accessing
//! planetary interaction-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing planetary interaction-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct PlanetaryInteractionEndpoints<'a> {
    client: &'a Client,
}

impl<'a> PlanetaryInteractionEndpoints<'a> {
    /// Creates a new instance of [`PlanetaryInteractionEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
