//! # EVE ESI Scope Builder
//!
//! This module provides a type-safe way to define and manage EVE Online ESI OAuth2 scopes
//! using the [`ScopeBuilder`].
//!
//! For an overview & usage, see the [module-level documentation](super).
//!
//! # Methods
//! - [`ScopeBuilder::new`]: Creates a new [`ScopeBuilder`] instance
//! - [`ScopeBuilder::build`]: Builds the list of scopes into a `Vec<`[`String`]`>`
//! - [`ScopeBuilder::public_data`]: Access to retrieve public information on a character (this scope is mostly just for show)
//! - [`ScopeBuilder::character`]: Adds scopes from [`CharacterScopes`]
//! - [`ScopeBuilder::corporation`]: Adds scopes from [`CorporationScopes`]
//! - [`ScopeBuilder::wallet`]: Adds scopes from [`WalletScopes`]
//! - [`ScopeBuilder::market`]: Adds scopes from [`MarketScopes`]
//! - [`ScopeBuilder::custom`]: Adds a custom scope

use crate::scope::{CorporationScopes, MarketScopes, WalletScopes};

use super::characters::CharacterScopes;

/// `publicData` scope
pub const PUBLIC_DATA: &str = "publicData";

/// Builder for creating a list of EVE Online OAuth2 scopes.
///
/// For a full overview & examples, see the [module-level documentation](self).
pub struct ScopeBuilder {
    scopes: Vec<String>,
}

impl Default for ScopeBuilder {
    /// Create a default instance of [`ScopeBuilder`]
    fn default() -> Self {
        Self::new()
    }
}

impl ScopeBuilder {
    /// Creates a new [`ScopeBuilder`] instance
    pub fn new() -> Self {
        ScopeBuilder { scopes: Vec::new() }
    }

    /// Builds a [`ScopeBuilder`] into a `Vec<`[`String`]`>` containing all scopes
    pub fn all() -> Vec<String> {
        ScopeBuilder::new()
            .public_data()
            .character(CharacterScopes::all())
            .corporation(CorporationScopes::all())
            .wallet(WalletScopes::all())
            .market(MarketScopes::all())
            .build()
    }

    /// Builds a [`ScopeBuilder`] into a `Vec<`[`String`]`>` containing the configured scopes
    pub fn build(self) -> Vec<String> {
        self.scopes
    }

    /// Access to retrieve public information on a character (this scope is mostly just for show)
    ///
    /// Adds the `publicData` scope
    pub fn public_data(mut self) -> Self {
        self.scopes.push(PUBLIC_DATA.to_string());
        self
    }

    /// Adds scopes from [`CharacterScopes`]
    pub fn character(mut self, character_scopes: CharacterScopes) -> Self {
        self.scopes.extend(character_scopes.scopes);
        self
    }

    /// Adds scopes from [`CorporationScopes`]
    pub fn corporation(mut self, corporation_scopes: CorporationScopes) -> Self {
        self.scopes.extend(corporation_scopes.scopes);
        self
    }

    /// Adds scopes from [`WalletScopes`]
    pub fn wallet(mut self, wallet_scopes: WalletScopes) -> Self {
        self.scopes.extend(wallet_scopes.scopes);
        self
    }

    /// Adds scopes from [`MarketScopes`]
    pub fn market(mut self, market_scopes: MarketScopes) -> Self {
        self.scopes.extend(market_scopes.scopes);
        self
    }

    /// Adds a custom scope
    pub fn custom(mut self, scope: &str) -> Self {
        self.scopes.push(scope.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // These scope builder tests are basic, the majority of its actual functionality is
    // tested in the endpoint integration tests. Here we're just ensuring the core functions
    // work.

    /// Tests initialization & successful building of a new instance of scope builder
    #[test]
    fn test_scope_builder_default() {
        ScopeBuilder::default().build();
    }

    /// Tests that all existing scopes can be built
    #[test]
    fn test_scope_builder_all() {
        ScopeBuilder::all();
    }

    /// Tests successful setting & building with a custom scope
    #[test]
    fn test_scope_builder_custom() {
        let scopes = ScopeBuilder::new().custom("custom_scope").build();

        assert_eq!(scopes[0], "custom_scope");
    }
}
