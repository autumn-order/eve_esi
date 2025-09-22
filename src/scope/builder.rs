//! # EVE ESI Scope Builder
//!
//! This module provides a type-safe way to define and manage EVE Online ESI OAuth2 scopes
//! using the [`ScopeBuilder`].
//!
//! For an overview & usage, see the [module-level documentation](super).
//!
//! ## Methods
//! |              Method           |                     Description                      |
//! | ----------------------------- | ---------------------------------------------------- |
//! | [`ScopeBuilder::new`]         | Creates a new [`ScopeBuilder`] instance              |
//! | [`ScopeBuilder::build`]       | Builds the list of scopes into a `Vec<`[`String`]`>` |
//! | [`ScopeBuilder::custom`]      | Adds a custom scope                                  |
//! | [`ScopeBuilder::public_data`] | Access to retrieve public information on a character |
//! | [`ScopeBuilder::alliance`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::assets`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::calendar`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::character`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::clones`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::contracts`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::corporation`] | Adds scopes from [`CorporationScopes`]               |
//! | [`ScopeBuilder::fittings`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::fleets`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::industry`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::killmails`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::location`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::mail`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::markets`]      | Adds scopes from [`MarketScopes`]                    |
//! | [`ScopeBuilder::planets`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::search`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::skills`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::ui`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::universe`]   | Adds scopes from [`CharacterScopes`]                 |
//! | [`ScopeBuilder::wallet`]      | Adds scopes from [`WalletScopes`]                    |

use crate::scope::{
    AllianceScopes, AssetScopes, CalendarScopes, CloneScopes, ContractScopes, CorporationScopes,
    FittingScopes, FleetScopes, IndustryScopes, KillmailScopes, LocationScopes, MailScopes,
    MarketScopes, PlanetScopes, SearchScopes, SkillScopes, UniverseScopes, UserInterfaceScopes,
    WalletScopes,
};

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
            .markets(MarketScopes::all())
            .build()
    }

    /// Builds a [`ScopeBuilder`] into a `Vec<`[`String`]`>` containing the configured scopes
    pub fn build(self) -> Vec<String> {
        self.scopes
    }

    /// Adds a custom scope
    pub fn custom(mut self, scope: &str) -> Self {
        self.scopes.push(scope.to_string());
        self
    }

    /// Access to retrieve public information on a character (this scope is mostly just for show)
    ///
    /// Adds the `publicData` scope
    pub fn public_data(mut self) -> Self {
        self.scopes.push(PUBLIC_DATA.to_string());
        self
    }

    /// Adds scopes from [`AllianceScopes`]
    pub fn alliance(mut self, alliance_scopes: AllianceScopes) -> Self {
        self.scopes.extend(alliance_scopes.scopes);
        self
    }

    /// Adds scopes from [`AssetScopes`]
    pub fn assets(mut self, assets_scopes: AssetScopes) -> Self {
        self.scopes.extend(assets_scopes.scopes);
        self
    }

    /// Adds scopes from [`CalendarScopes`]
    pub fn calendar(mut self, calendar_scopes: CalendarScopes) -> Self {
        self.scopes.extend(calendar_scopes.scopes);
        self
    }

    /// Adds scopes from [`CharacterScopes`]
    pub fn character(mut self, character_scopes: CharacterScopes) -> Self {
        self.scopes.extend(character_scopes.scopes);
        self
    }

    /// Adds scopes from [`CloneScopes`]
    pub fn clones(mut self, clones_scopes: CloneScopes) -> Self {
        self.scopes.extend(clones_scopes.scopes);
        self
    }

    /// Adds scopes from [`ContractScopes`]
    pub fn contracts(mut self, contracts_scopes: ContractScopes) -> Self {
        self.scopes.extend(contracts_scopes.scopes);
        self
    }

    /// Adds scopes from [`CorporationScopes`]
    pub fn corporation(mut self, corporation_scopes: CorporationScopes) -> Self {
        self.scopes.extend(corporation_scopes.scopes);
        self
    }

    /// Adds scopes from [`FittingScopes`]
    pub fn fittings(mut self, fittings_scopes: FittingScopes) -> Self {
        self.scopes.extend(fittings_scopes.scopes);
        self
    }

    /// Adds scopes from [`FleetScopes`]
    pub fn fleets(mut self, fleets_scopes: FleetScopes) -> Self {
        self.scopes.extend(fleets_scopes.scopes);
        self
    }

    /// Adds scopes from [`IndustryScopes`]
    pub fn industry(mut self, industry_scopes: IndustryScopes) -> Self {
        self.scopes.extend(industry_scopes.scopes);
        self
    }

    /// Adds scopes from [`KillmailScopes`]
    pub fn killmails(mut self, killmails_scopes: KillmailScopes) -> Self {
        self.scopes.extend(killmails_scopes.scopes);
        self
    }

    /// Adds scopes from [`LocationScopes`]
    pub fn location(mut self, location_scopes: LocationScopes) -> Self {
        self.scopes.extend(location_scopes.scopes);
        self
    }

    /// Adds scopes from [`MailScopes`]
    pub fn mail(mut self, mail_scopes: MailScopes) -> Self {
        self.scopes.extend(mail_scopes.scopes);
        self
    }

    /// Adds scopes from [`MarketScopes`]
    pub fn markets(mut self, markets_scopes: MarketScopes) -> Self {
        self.scopes.extend(markets_scopes.scopes);
        self
    }

    /// Adds scopes from [`PlanetScopes`]
    pub fn planets(mut self, planets_scopes: PlanetScopes) -> Self {
        self.scopes.extend(planets_scopes.scopes);
        self
    }

    /// Adds scopes from [`SearchScopes`]
    pub fn search(mut self, search_scopes: SearchScopes) -> Self {
        self.scopes.extend(search_scopes.scopes);
        self
    }

    /// Adds scopes from [`SkillScopes`]
    pub fn skills(mut self, skills_scopes: SkillScopes) -> Self {
        self.scopes.extend(skills_scopes.scopes);
        self
    }

    /// Adds scopes from [`UserInterfaceScopes`]
    pub fn ui(mut self, ui_scopes: UserInterfaceScopes) -> Self {
        self.scopes.extend(ui_scopes.scopes);
        self
    }

    /// Adds scopes from [`UniverseScopes`]
    pub fn universe(mut self, universe_scopes: UniverseScopes) -> Self {
        self.scopes.extend(universe_scopes.scopes);
        self
    }

    /// Adds scopes from [`WalletScopes`]
    pub fn wallet(mut self, wallet_scopes: WalletScopes) -> Self {
        self.scopes.extend(wallet_scopes.scopes);
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
