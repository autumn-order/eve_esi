//! # EVE ESI Skills Scopes
//!
//! This module provides a type-safe way to add skill-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                   | Description                                                       |
//! | ---------------------------------------- | ----------------------------------------------------------------- |
//! | [`SkillScopes::new`]                     | Creates a new instance of [`SkillScopes`]                         |
//! | [`SkillScopes::all`]                     | Creates a new instance of [`SkillScopes`] with all scopes applied |

/// Struct with methods for listing skill scopes to request for OAuth2
pub struct SkillScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for SkillScopes {
    /// Create a default instance of [`SkillScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl SkillScopes {
    /// Create a new instance of [`SkillScopes`]
    pub fn new() -> Self {
        SkillScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`SkillScopes`] with all scopes applied
    pub fn all() -> Self {
        SkillScopes::new()
    }
}
