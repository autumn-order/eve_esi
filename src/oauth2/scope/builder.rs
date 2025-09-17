//! # EVE Online OAuth2 Scope Builder
//!
//! This module provides a type-safe way to define and manage EVE Online ESI OAuth2 scopes
//! using the [`ScopeBuilder`].
//!
//! # Methods
//! - [`ScopeBuilder::new`]: Creates a new [`ScopeBuilder`] instance
//! - [`ScopeBuilder::build`]: Builds the list of scopes into a `Vec<`[`String`]`>`
//! - [`ScopeBuilder::public_data`]: Adds the `publicData` scope
//! - [`ScopeBuilder::character`]: Adds scopes from [`CharacterScopes`]
//! - [`ScopeBuilder::custom`]: Adds a custom scope
//!
//! For an overview & usage, see the [module-level documentation](super).

use crate::oauth2::scope::CorporationScopes;

use super::character::CharacterScopes;

/// `publicData` scope
pub const PUBLIC_DATA: &str = "publicData";

/// Builder for creating a list of EVE Online OAuth2 scopes.
///
/// For a full overview & examples, see the [module-level documentation](self).
pub struct ScopeBuilder {
    scopes: Vec<String>,
}

impl ScopeBuilder {
    /// Creates a new [`ScopeBuilder`] instance.
    pub fn new() -> Self {
        ScopeBuilder { scopes: Vec::new() }
    }

    /// Builds the list of scopes into a `Vec<`[`String`]`>`.
    pub fn build(self) -> Vec<String> {
        self.scopes
    }

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

    /// Adds a custom scope
    pub fn custom(mut self, scope: &str) -> Self {
        self.scopes.push(scope.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that all existing scopes can be built.
    ///
    /// # Setup
    /// - Creates a new ScopeBuilder instance with every scope.
    ///
    /// # Assertions
    /// - Asserts that we have the right number of scopes.
    /// - Asserts that the scopes are correct and in the right order.
    #[test]
    fn test_all_scopes() {
        // Create a new ScopeBuilder instance with every scope
        let scopes = ScopeBuilder::new()
            .public_data()
            .custom("custom_scope")
            .character(CharacterScopes::new().read_agents_research())
            .build();

        // Assert that we have the right number of scopes
        assert_eq!(scopes.len(), 3);

        // Assert that the scopes are correct and in the right order
        assert_eq!(scopes[0], super::PUBLIC_DATA);
        assert_eq!(scopes[1], "custom_scope");
        assert_eq!(scopes[2], super::super::character::READ_AGENTS_RESEARCH)
    }
}
