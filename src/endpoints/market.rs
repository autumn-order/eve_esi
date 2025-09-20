//! # EVE ESI Market Endpoints
//!
//! This module provides the [`MarketEndpoints`] struct and associated methods for accessing
//! market-related ESI endpoints.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
//!
//! # ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>

use crate::Client;

/// Provides methods for accessing market-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct MarketEndpoints<'a> {
    client: &'a Client,
}

impl<'a> MarketEndpoints<'a> {
    /// Creates a new instance of [`MarketEndpoints`].
    ///
    /// For an overview & usage examples, see the [endpoints module documentation](super)
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
