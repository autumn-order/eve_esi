//! # EVE ESI Industry Scopes
//!
//! This module provides a type-safe way to add industry-related scopes for OAuth2 to the [`super::ScopeBuilder`]

/// Read access to character industry jobs
pub const READ_CHARACTER_JOBS: &str = "esi-industry.read_character_jobs.v1";
/// Read access to character's mining ledger
pub const READ_CHARACTER_MINING: &str = "esi-industry.read_character_mining.v1";
/// Read access to corporation industry jobs
pub const READ_CORPORATION_JOBS: &str = "esi-industry.read_corporation_jobs.v1";
/// Read access to corporation mining ledger (moon mining structures)
pub const READ_CORPORATION_MINING: &str = "esi-industry.read_corporation_mining.v1";

/// Struct with methods for listing industry scopes to request for OAuth2
pub struct IndustryScopes {
    pub(super) scopes: Vec<String>,
}

impl Default for IndustryScopes {
    /// Create a default instance of [`IndustryScopes`]
    fn default() -> Self {
        Self::new()
    }
}

impl IndustryScopes {
    /// Create a new instance of [`IndustryScopes`]
    pub fn new() -> Self {
        IndustryScopes { scopes: Vec::new() }
    }

    /// Creates a new instance of [`IndustryScopes`] with all scopes applied
    pub fn all() -> Self {
        IndustryScopes::new()
            .read_character_jobs()
            .read_character_mining()
            .read_corporation_jobs()
            .read_corporation_mining()
    }

    /// Read access to character industry jobs
    ///
    /// Adds the `esi-industry.read_character_jobs.v1` scope
    pub fn read_character_jobs(mut self) -> Self {
        self.scopes.push(READ_CHARACTER_JOBS.to_string());
        self
    }

    /// Read access to character's mining ledger
    ///
    /// Adds the `esi-industry.read_character_mining.v1` scope
    pub fn read_character_mining(mut self) -> Self {
        self.scopes.push(READ_CHARACTER_MINING.to_string());
        self
    }

    /// Read access to corporation industry jobs
    ///
    /// Adds the `esi-industry.read_corporation_jobs.v1` scope
    pub fn read_corporation_jobs(mut self) -> Self {
        self.scopes.push(READ_CORPORATION_JOBS.to_string());
        self
    }

    /// Read access to corporation mining ledger (moon mining structures)
    ///
    /// Adds the `esi-industry.read_corporation_mining.v1` scope
    pub fn read_corporation_mining(mut self) -> Self {
        self.scopes.push(READ_CORPORATION_MINING.to_string());
        self
    }
}

#[cfg(test)]
mod industry_scopes_tests {
    use crate::scope::IndustryScopes;

    /// Tests initializing a default instance of [`IndustryScopes`]
    #[test]
    fn test_industry_scopes_default() {
        let industry_scopes = IndustryScopes::default();

        assert_eq!(industry_scopes.scopes.len(), 0)
    }
}
