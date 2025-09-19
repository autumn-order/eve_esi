//! # EVE Online OAuth2 Wallet Scopes
//!
//! This module provides a type-safe way to add wallet-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! # Methods
//! - [`WalletScopes::new`]: Creates a new instance of [`WalletScopes`]

/// Access to retrieve information for character's corporation wallets
pub const READ_CORPORATION_WALLETS: &str = "esi-wallet.read_corporation_wallets.v1";

/// Struct with methods for listing wallet scopes to request for OAuth2
pub struct WalletScopes {
    pub(super) scopes: Vec<String>,
}

impl WalletScopes {
    /// Create a new instance of [`WalletScopes`]
    pub fn new() -> Self {
        WalletScopes { scopes: Vec::new() }
    }

    /// Create a new instance of [`WalletScopes`] with all scopes applied
    pub fn all() -> Self {
        WalletScopes::new().read_corporation_wallets()
    }

    /// Adds the `esi-wallet.read_corporation_wallets.v1` scope
    ///
    /// Access to retrieve information for character's corporation wallets
    pub fn read_corporation_wallets(mut self) -> Self {
        self.scopes.push(READ_CORPORATION_WALLETS.to_string());
        self
    }
}
