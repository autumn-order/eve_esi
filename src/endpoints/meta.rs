//! # EVE ESI Meta Endpoints
//!
//! This module provides the [`MetaEndpoints`] struct and associated methods for accessing
//! meta-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing meta-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct MetaEndpoints<'a> {
    client: &'a Client,
}

impl<'a> MetaEndpoints<'a> {
    /// Creates a new instance of [`MetaEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
