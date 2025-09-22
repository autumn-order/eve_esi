//! # EVE ESI Scope Builder
//!
//! This module provides a type-safe way to define and manage EVE Online ESI OAuth2 scopes
//! using the [`ScopeBuilder`].
//!
//! For an overview & usage, see the [module-level documentation](super).
//!
//! ## Methods
//! | Method                         | Description                                          |
//! | ------------------------------ | ---------------------------------------------------- |
//! | [`ScopeBuilder::new`]          | Creates a new [`ScopeBuilder`] instance              |
//! | [`ScopeBuilder::build`]        | Builds the list of scopes into a `Vec<`[`String`]`>` |
//! | [`ScopeBuilder::custom`]       | Adds a custom scope                                  |
//! | [`ScopeBuilder::public_data`]  | Access to retrieve public information on a character |
//! | [`ScopeBuilder::alliances`]    | Adds scopes from [`AlliancesScopes`]                 |
//! | [`ScopeBuilder::assets`]       | Adds scopes from [`AssetsScopes`]                    |
//! | [`ScopeBuilder::calendar` ]    | Adds scopes from [`CalendarScopes`]                  |
//! | [`ScopeBuilder::characters`]   | Adds scopes from [`CharactersScopes`]                |
//! | [`ScopeBuilder::clones`]       | Adds scopes from [`ClonesScopes`]                    |
//! | [`ScopeBuilder::contracts`]    | Adds scopes from [`ContractsScopes`]                 |
//! | [`ScopeBuilder::corporations`] | Adds scopes from [`CorporationsScopes`]              |
//! | [`ScopeBuilder::fittings`]     | Adds scopes from [`FittingsScopes`]                  |
//! | [`ScopeBuilder::fleets`]       | Adds scopes from [`FleetsScopes`]                    |
//! | [`ScopeBuilder::industry`]     | Adds scopes from [`IndustryScopes`]                  |
//! | [`ScopeBuilder::killmails`]    | Adds scopes from [`KillmailsScopes`]                 |
//! | [`ScopeBuilder::location`]     | Adds scopes from [`LocationScopes`]                  |
//! | [`ScopeBuilder::mail`]         | Adds scopes from [`MailScopes`]                      |
//! | [`ScopeBuilder::markets`]      | Adds scopes from [`MarketsScopes`]                   |
//! | [`ScopeBuilder::planets`]      | Adds scopes from [`PlanetsScopes`]                   |
//! | [`ScopeBuilder::search`]       | Adds scopes from [`SearchScopes`]                    |
//! | [`ScopeBuilder::skills`]       | Adds scopes from [`SkillsScopes`]                    |
//! | [`ScopeBuilder::ui`]           | Adds scopes from [`UiScopes`]                        |
//! | [`ScopeBuilder::universe`]     | Adds scopes from [`UniverseScopes`]                  |
//! | [`ScopeBuilder::wallet`]       | Adds scopes from [`WalletScopes`]                    |

use crate::scope::{
    AlliancesScopes, AssetsScopes, CalendarScopes, CharactersScopes, ClonesScopes, ContractsScopes,
    CorporationsScopes, FittingsScopes, FleetsScopes, IndustryScopes, KillmailsScopes,
    LocationScopes, MailScopes, MarketsScopes, PlanetsScopes, SearchScopes, SkillsScopes, UiScopes,
    UniverseScopes, WalletScopes,
};

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
            .characters(CharactersScopes::all())
            .corporations(CorporationsScopes::all())
            .wallet(WalletScopes::all())
            .markets(MarketsScopes::all())
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

    /// Adds scopes from [`AlliancesScopes`]
    pub fn alliances(mut self, alliances_scopes: AlliancesScopes) -> Self {
        self.scopes.extend(alliances_scopes.scopes);
        self
    }

    /// Adds scopes from [`AssetsScopes`]
    pub fn assets(mut self, assets_scopes: AssetsScopes) -> Self {
        self.scopes.extend(assets_scopes.scopes);
        self
    }

    /// Adds scopes from [`CalendarScopes`]
    pub fn calendar(mut self, calendar_scopes: CalendarScopes) -> Self {
        self.scopes.extend(calendar_scopes.scopes);
        self
    }

    /// Adds scopes from [`CharactersScopes`]
    pub fn characters(mut self, characters_scopes: CharactersScopes) -> Self {
        self.scopes.extend(characters_scopes.scopes);
        self
    }

    /// Adds scopes from [`ClonesScopes`]
    pub fn clones(mut self, clones_scopes: ClonesScopes) -> Self {
        self.scopes.extend(clones_scopes.scopes);
        self
    }

    /// Adds scopes from [`ContractsScopes`]
    pub fn contracts(mut self, contracts_scopes: ContractsScopes) -> Self {
        self.scopes.extend(contracts_scopes.scopes);
        self
    }

    /// Adds scopes from [`CorporationsScopes`]
    pub fn corporations(mut self, corporations_scopes: CorporationsScopes) -> Self {
        self.scopes.extend(corporations_scopes.scopes);
        self
    }

    /// Adds scopes from [`FittingsScopes`]
    pub fn fittings(mut self, fittings_scopes: FittingsScopes) -> Self {
        self.scopes.extend(fittings_scopes.scopes);
        self
    }

    /// Adds scopes from [`FleetsScopes`]
    pub fn fleets(mut self, fleets_scopes: FleetsScopes) -> Self {
        self.scopes.extend(fleets_scopes.scopes);
        self
    }

    /// Adds scopes from [`IndustryScopes`]
    pub fn industry(mut self, industry_scopes: IndustryScopes) -> Self {
        self.scopes.extend(industry_scopes.scopes);
        self
    }

    /// Adds scopes from [`KillmailsScopes`]
    pub fn killmails(mut self, killmails_scopes: KillmailsScopes) -> Self {
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

    /// Adds scopes from [`MarketsScopes`]
    pub fn markets(mut self, markets_scopes: MarketsScopes) -> Self {
        self.scopes.extend(markets_scopes.scopes);
        self
    }

    /// Adds scopes from [`PlanetsScopes`]
    pub fn planets(mut self, planets_scopes: PlanetsScopes) -> Self {
        self.scopes.extend(planets_scopes.scopes);
        self
    }

    /// Adds scopes from [`SearchScopes`]
    pub fn search(mut self, search_scopes: SearchScopes) -> Self {
        self.scopes.extend(search_scopes.scopes);
        self
    }

    /// Adds scopes from [`SkillsScopes`]
    pub fn skills(mut self, skills_scopes: SkillsScopes) -> Self {
        self.scopes.extend(skills_scopes.scopes);
        self
    }

    /// Adds scopes from [`UiScopes`]
    pub fn ui(mut self, ui_scopes: UiScopes) -> Self {
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
