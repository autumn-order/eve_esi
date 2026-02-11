//! # EVE ESI User Interface Endpoints
//!
//! This module provides the [`UserInterfaceEndpoints`] struct and associated methods for accessing
//! user interface-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing user interface-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct UserInterfaceEndpoints<'a> {
    client: &'a Client,
}

impl<'a> UserInterfaceEndpoints<'a> {
    /// Creates a new instance of [`UserInterfaceEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
