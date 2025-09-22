//! # EVE ESI Skills Scopes
//!
//! This module provides a type-safe way to add skill-related scopes for OAuth2 to the [`super::ScopeBuilder`]
//!
//! See [module-level documentation](super) for an overview & usage of scopes for the esi_crate
//!
//! ## Methods
//! | Method                                    | Description                                                        |
//! | ----------------------------------------- | ------------------------------------------------------------------ |
//! | [`SkillsScopes::new`]                     | Creates a new instance of [`SkillsScopes`]                         |
//! | [`SkillsScopes::all`]                     | Creates a new instance of [`SkillsScopes`] with all scopes applied |

/// Struct with methods for listing skill scopes to request for OAuth2
pub struct SkillsScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for SkillsScopes {
    /// Create a default instance of [`SkillsScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl SkillsScopes {
    /// Create a new instance of [`SkillsScopes`]
    pub fn new() -> Self {
        SkillsScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`SkillsScopes`] with all scopes applied
    pub fn all() -> Self {
        SkillsScopes::new()
    }
}

#[cfg(test)]
mod skills_scopes_tests {
    use crate::scope::SkillsScopes;

    /// Tests initializing a default instance of [`SkillsScopes`]
    #[test]
    fn test_skills_scopes_default() {
        let skills_scopes = SkillsScopes::default();

        assert_eq!(skills_scopes.scopes.len(), 0)
    }
}
