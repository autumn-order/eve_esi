//! # EVE ESI Search Endpoints
//!
//! This module provides the [`SearchEndpoints`] struct and associated methods for accessing
//! search-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing search-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct SearchEndpoints<'a> {
    client: &'a Client,
}

impl<'a> SearchEndpoints<'a> {
    /// Creates a new instance of [`SearchEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
