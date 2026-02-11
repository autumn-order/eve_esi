//! # EVE ESI Killmails Endpoints
//!
//! This module provides the [`KillmailsEndpoints`] struct and associated methods for accessing
//! killmail-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing killmail-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct KillmailsEndpoints<'a> {
    client: &'a Client,
}

impl<'a> KillmailsEndpoints<'a> {
    /// Creates a new instance of [`KillmailsEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
