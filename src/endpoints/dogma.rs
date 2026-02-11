//! # EVE ESI Dogma Endpoints
//!
//! This module provides the [`DogmaEndpoints`] struct and associated methods for accessing
//! dogma-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing dogma-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct DogmaEndpoints<'a> {
    client: &'a Client,
}

impl<'a> DogmaEndpoints<'a> {
    /// Creates a new instance of [`DogmaEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
