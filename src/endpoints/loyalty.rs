//! # EVE ESI Loyalty Endpoints
//!
//! This module provides the [`LoyaltyEndpoints`] struct and associated methods for accessing
//! loyalty-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing loyalty-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct LoyaltyEndpoints<'a> {
    client: &'a Client,
}

impl<'a> LoyaltyEndpoints<'a> {
    /// Creates a new instance of [`LoyaltyEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
