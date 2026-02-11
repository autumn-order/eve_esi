//! # EVE ESI Mail Endpoints
//!
//! This module provides the [`MailEndpoints`] struct and associated methods for accessing
//! mail-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing mail-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct MailEndpoints<'a> {
    client: &'a Client,
}

impl<'a> MailEndpoints<'a> {
    /// Creates a new instance of [`MailEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
