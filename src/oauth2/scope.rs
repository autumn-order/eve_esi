//! EVE Online ESI OAuth2 scope definitions and builder
//!
//! This module provides a type-safe way to define and manage EVE Online ESI OAuth2 scopes.
//! It uses a builder pattern to create a list of scopes.
//!
/// # Notes
/// - For custom scopes, use the `custom` method.
/// - For no scopes, simply use ScopeBuilder::new().build().
///
/// # Example
/// ```
/// let scopes = eve_esi::oauth2::ScopeBuilder::new()
///     .public_data()
///     .build();
/// ```

/// ESI OAuth2 scope string constants organized in a nested module structure.
pub mod scope {
    /// publicData scope.
    pub mod public {
        /// publicData scope.
        pub const PUBLIC_DATA: &str = "publicData";
    }
}

/// Builder for creating a list of EVE Online ESI OAuth2 scopes.
///
/// For a full overview & examples, see the [module-level documentation](self).
pub struct ScopeBuilder {
    scopes: Vec<String>,
}

impl ScopeBuilder {
    /// Creates a new ScopeBuilder instance.
    ///
    /// # Example
    /// ```
    /// let builder = eve_esi::oauth2::ScopeBuilder::new();
    /// ```
    pub fn new() -> Self {
        ScopeBuilder { scopes: Vec::new() }
    }

    /// Builds the list of scopes into Vec<String>.
    ///
    /// # Example
    /// ```
    /// let scopes = eve_esi::oauth2::ScopeBuilder::new()
    ///     .public_data()
    ///     .build();
    /// ```
    pub fn build(self) -> Vec<String> {
        self.scopes
    }

    /// Adds a custom scope to the list.
    ///
    /// # Example
    /// ```
    /// let scopes = eve_esi::oauth2::ScopeBuilder::new()
    ///     .custom("custom_scope")
    ///     .build();
    /// ```
    pub fn custom(mut self, scope: &str) -> Self {
        self.scopes.push(scope.to_string());
        self
    }

    /// Adds the publicData scope.
    ///
    /// # Example
    /// ```
    /// let scopes = eve_esi::oauth2::ScopeBuilder::new()
    ///     .public_data()
    ///     .build();
    /// ```
    pub fn public_data(mut self) -> Self {
        self.scopes.push(scope::public::PUBLIC_DATA.to_string());
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
            .build();

        // Assert that we have the right number of scopes
        assert_eq!(scopes.len(), 2);

        // Assert that the scopes are correct and in the right order
        assert_eq!(scopes[0], scope::public::PUBLIC_DATA);
        assert_eq!(scopes[1], "custom_scope");
    }
}
