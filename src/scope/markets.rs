//! # EVE ESI Markets Scopes
//!
//! This module provides a type-safe way to add market-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                     | Description                                                         |
//! | ------------------------------------------ | ------------------------------------------------------------------- |
//! | [`MarketsScopes::new`]                     | Creates a new instance of [`MarketsScopes`]                         |
//! | [`MarketsScopes::all`]                     | Creates a new instance of [`MarketsScopes`] with all scopes applied |
//! | [`MarketsScopes::read_character_orders`]   | Access to retrieve information on character's market orders         |
//! | [`MarketsScopes::read_corporation_orders`] | Access to retrieve information on corporation's market orders       |
//! | [`MarketsScopes::structure_markets`]       | Access to retrieve information on a structure's market orders       |

/// Access to retrieve information on character's market orders
pub const READ_CHARACTER_ORDERS: &str = "esi-markets.read_character_orders.v1";
/// Access to retrieve information on corporation's market orders
pub const READ_CORPORATION_ORDERS: &str = "esi-markets.read_corporation_orders.v1";
/// Access to retrieve information on a structure's market orders
pub const STRUCTURE_MARKETS: &str = "esi-markets.structure_markets.v1";

/// Struct with methods for listing corporation scopes to request for OAuth2
pub struct MarketsScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for MarketsScopes {
    /// Create a default instance of [`MarketsScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl MarketsScopes {
    /// Create a new instance of [`MarketsScopes`]
    pub fn new() -> Self {
        MarketsScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`MarketsScopes`] with all scopes applied
    pub fn all() -> Self {
        MarketsScopes::new()
            .read_character_orders()
            .read_corporation_orders()
    }

    /// Access to retrieve information on character's market orders
    ///
    /// Adds the `esi-markets.read_character_orders.v1` scope
    pub fn read_character_orders(mut self) -> Self {
        self.scopes.push(READ_CHARACTER_ORDERS.to_string());
        self
    }

    /// Access to retrieve information on corporation's market orders
    ///
    /// Adds the `esi-markets.read_corporation_orders.v1` scope
    pub fn read_corporation_orders(mut self) -> Self {
        self.scopes.push(READ_CORPORATION_ORDERS.to_string());
        self
    }

    /// Access to retrieve information on a structure's market orders
    ///
    /// Adds the `esi-markets.structure_markets.v1` scope
    pub fn structure_markets(mut self) -> Self {
        self.scopes.push(STRUCTURE_MARKETS.to_string());
        self
    }
}

#[cfg(test)]
mod market_scopes_tests {
    use crate::scope::MarketsScopes;

    /// Tests initializing a default instance of [`MarketsScopes`]
    #[test]
    fn test_market_scopes_default() {
        let market_scopes = MarketsScopes::default();

        assert_eq!(market_scopes.scopes.len(), 0)
    }
}
