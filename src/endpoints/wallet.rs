//! # EVE ESI Wallet Endpoints
//!
//! This module provides the [`WalletEndpoints`] struct and associated methods for accessing
//! wallet-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing wallet-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct WalletEndpoints<'a> {
    client: &'a Client,
}

impl<'a> WalletEndpoints<'a> {
    /// Creates a new instance of [`WalletEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
