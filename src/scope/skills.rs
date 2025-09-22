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
//! | [`SkillsScopes::read_skillqueue`]         | Read access to character's skill queue                             |
//! | [`SkillsScopes::read_skills`]             | Read access to character's skills                                  |

/// Read access to character's skill queue
pub const READ_SKILLQUEUE: &str = "esi-skills.read_skillqueue.v1";
/// Read access to character's skills
pub const READ_SKILLS: &str = "esi-skills.read_skills.v1";

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
        SkillsScopes::new().read_skillqueue().read_skills()
    }

    /// Read access to character's skill queue
    ///
    /// Adds the `esi-skills.read_skillqueue.v1` scope
    pub fn read_skillqueue(mut self) -> Self {
        self.scopes.push(READ_SKILLQUEUE.to_string());
        self
    }

    /// Read access to character's skills
    ///
    /// Adds the `esi-skills.read_skills.v1` scope
    pub fn read_skills(mut self) -> Self {
        self.scopes.push(READ_SKILLS.to_string());
        self
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
