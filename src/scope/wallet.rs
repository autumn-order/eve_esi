//! # EVE ESI Wallet Scopes
//!
//! This module provides a type-safe way to add wallet-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                     | Description                                                           |
//! | ------------------------------------------ | --------------------------------------------------------------------- |
//! | [`WalletScopes::new`]                      | Creates a new instance of [`WalletScopes`]                            |
//! | [`WalletScopes::new`]                      | Creates a new instance of [`WalletScopes`] with all scopes applied    |
//! | [`WalletScopes::read_corporation_wallets`] | Access to retrieve information for character's corporation wallets    |

/// Access to retrieve information for character's corporation wallets
pub const READ_CORPORATION_WALLETS: &str = "esi-wallet.read_corporation_wallets.v1";

/// Struct with methods for listing wallet scopes to request for OAuth2
pub struct WalletScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for WalletScopes {
    /// Create a default instance of [`WalletScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl WalletScopes {
    /// Create a new instance of [`WalletScopes`]
    pub fn new() -> Self {
        WalletScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`WalletScopes`] with all scopes applied
    pub fn all() -> Self {
        WalletScopes::new().read_corporation_wallets()
    }

    /// Access to retrieve information for character's corporation wallets
    ///
    /// Adds the `esi-wallet.read_corporation_wallets.v1` scope
    pub fn read_corporation_wallets(mut self) -> Self {
        self.scopes.push(READ_CORPORATION_WALLETS.to_string());
        self
    }
}

#[cfg(test)]
mod wallet_scopes_tests {
    use crate::scope::WalletScopes;

    /// Tests initializing a default instance of [`WalletScopes`]
    #[test]
    fn test_wallet_scopes_default() {
        let wallet_scopes = WalletScopes::default();

        assert_eq!(wallet_scopes.scopes.len(), 0)
    }
}
