//! EVE Online ESI OAuth2 scope definitions and builder
//!
//! This module provides a type-safe way to define and manage EVE Online ESI OAuth2 scopes.
//! It uses a builder pattern to create a list of scopes.

/// ESI OAuth2 scope string constants organized in a nested module structure.
pub mod scope {
    pub mod public {
        pub const PUBLIC_DATA: &str = "publicData";
    }
}

/// Builder for creating a list of EVE Online ESI OAuth2 scopes.
///
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
pub struct ScopeBuilder {
    scopes: Vec<String>,
}

impl ScopeBuilder {
    pub fn new() -> Self {
        ScopeBuilder { scopes: Vec::new() }
    }

    pub fn build(self) -> Vec<String> {
        self.scopes
    }

    pub fn custom(mut self, scope: &str) -> Self {
        self.scopes.push(scope.to_string());
        self
    }

    pub fn public_data(mut self) -> Self {
        self.scopes.push(scope::public::PUBLIC_DATA.to_string());
        self
    }
}
