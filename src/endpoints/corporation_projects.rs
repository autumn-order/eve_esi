//! # EVE ESI Corporation Project Endpoints
//!
//! This module provides the [`CorporationProjectsEndpoints`] struct and associated methods for accessing
//! corporation project-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing corporation project-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct CorporationProjectsEndpoints<'a> {
    client: &'a Client,
}

impl<'a> CorporationProjectsEndpoints<'a> {
    /// Creates a new instance of [`CorporationProjectsEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
