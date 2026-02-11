//! # EVE ESI Incursions Endpoints
//!
//! This module provides the [`IncursionsEndpoints`] struct and associated methods for accessing
//! incursion-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing incursion-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct IncursionsEndpoints<'a> {
    client: &'a Client,
}

impl<'a> IncursionsEndpoints<'a> {
    /// Creates a new instance of [`IncursionsEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
