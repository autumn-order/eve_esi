//! # EVE Online OAuth2 Corporation Scopes
//!
//! This module provides a type-safe way to add corporation-releated scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! # Methods
//! - [`CorporationScopes::new`]: Creates a new instance of [`CorporationScopes`]

/// Access to retrieve information on corporation's blueprints
pub const READ_BLUEPRINTS: &str = "esi-corporations.read_blueprints.v1";
/// Access to read information on corporation container logs
pub const READ_CONTAINER_LOGS: &str = "esi-corporations.read_container_logs.v1";

/// Struct with methods for listing corporation scopes to request for OAuth2
pub struct CorporationScopes {
    pub(super) scopes: Vec<String>,
}

impl CorporationScopes {
    /// Create a new instance of [`CorporationScopes`]
    pub fn new() -> Self {
        CorporationScopes { scopes: Vec::new() }
    }

    /// Adds the `esi-corporations.read_blueprints.v1` scope
    pub fn read_blueprints(mut self) -> Self {
        self.scopes.push(READ_BLUEPRINTS.to_string());
        self
    }

    /// Adds the `esi-corporations.read_container_logs.v1` scope
    pub fn read_container_logs(mut self) -> Self {
        self.scopes.push(READ_CONTAINER_LOGS.to_string());
        self
    }
}
