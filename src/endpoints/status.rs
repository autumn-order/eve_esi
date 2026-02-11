//! # EVE ESI Status Endpoints
//!
//! This module provides the [`StatusEndpoints`] struct and associated methods for accessing
//! status-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing status-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct StatusEndpoints<'a> {
    client: &'a Client,
}

impl<'a> StatusEndpoints<'a> {
    /// Creates a new instance of [`StatusEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
