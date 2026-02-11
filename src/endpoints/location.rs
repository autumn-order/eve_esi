//! # EVE ESI Location Endpoints
//!
//! This module provides the [`LocationEndpoints`] struct and associated methods for accessing
//! location-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing location-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct LocationEndpoints<'a> {
    client: &'a Client,
}

impl<'a> LocationEndpoints<'a> {
    /// Creates a new instance of [`LocationEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
