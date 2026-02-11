//! # EVE ESI Faction Warfare Endpoints
//!
//! This module provides the [`FactionWarfareEndpoints`] struct and associated methods for accessing
//! faction warfare-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing faction warfare-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct FactionWarfareEndpoints<'a> {
    client: &'a Client,
}

impl<'a> FactionWarfareEndpoints<'a> {
    /// Creates a new instance of [`FactionWarfareEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
