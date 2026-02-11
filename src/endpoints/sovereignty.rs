//! # EVE ESI Sovereignty Endpoints
//!
//! This module provides the [`SovereigntyEndpoints`] struct and associated methods for accessing
//! sovereignty-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing sovereignty-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct SovereigntyEndpoints<'a> {
    client: &'a Client,
}

impl<'a> SovereigntyEndpoints<'a> {
    /// Creates a new instance of [`SovereigntyEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
