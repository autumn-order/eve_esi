//! # EVE ESI Insurance Endpoints
//!
//! This module provides the [`InsuranceEndpoints`] struct and associated methods for accessing
//! insurance-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing insurance-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct InsuranceEndpoints<'a> {
    client: &'a Client,
}

impl<'a> InsuranceEndpoints<'a> {
    /// Creates a new instance of [`InsuranceEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
