//! # EVE Online OAuth2 Market Scopes
//!
//! This module provides a type-safe way to add market-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! # Methods
//! - [`MarketScopes::new`]: Creates a new instance of [`MarketScopes`]
//! - [`MarketScopes::all`]: Create a new instance of [`MarketScopes`] with all scopes applied
//! - [`MarketScopes::read_character_orders`]: Access to retrieve information on character's market orders

/// Access to retrieve information on character's market orders
pub const READ_CHARACTERS_ORDERS: &str = "esi-markets.read_character_orders.v1";

/// Struct with methods for listing corporation scopes to request for OAuth2
pub struct MarketScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for MarketScopes {
    /// Create a default instance of [`MarketScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl MarketScopes {
    /// Create a new instance of [`MarketScopes`]
    pub fn new() -> Self {
        MarketScopes { scopes: Vec::new() }
    }

    /// Create a new instance of [`MarketScopes`] with all scopes applied
    pub fn all() -> Self {
        MarketScopes::new().read_character_orders()
    }

    /// Adds the `esi-markets.read_character_orders.v1` scope
    ///
    /// Access to retrieve information on character's market orders
    pub fn read_character_orders(mut self) -> Self {
        self.scopes.push(READ_CHARACTERS_ORDERS.to_string());
        self
    }
}

#[cfg(test)]
mod market_scopes_tests {
    use crate::oauth2::scope::MarketScopes;

    /// Tests initializing a default instance of [`MarketScopes`]
    #[test]
    fn test_market_scopes_default() {
        let market_scopes = MarketScopes::default();

        assert_eq!(market_scopes.scopes.len(), 0)
    }
}
