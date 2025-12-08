//! # EVE ESI Endpoints
//!
//! This module provides access to the different categories of endpoints available for EVE Online's ESI API.
//!
//! ## Modules
//! | Module          | Description           | Public Endpoints | Authenticated Endpoints |
//! | --------------- | --------------------- | ---------------- | ----------------------- |
//! | [`alliance`]    | Alliance endpoints    | 4                |                         |
//! | [`assets`]      | Clone endpoints       |                  | 6                       |
//! | [`calendar`]    | Calendar endpoints    |                  | 4                       |
//! | [`character`]   | Character endpoints   | 3                | 9                       |
//! | [`clones`]      | Clone endpoints       |                  | 2                       |
//! | [`contacts`]    | Contact endpoints     |                  | 9                       |
//! | [`corporation`] | Corporation endpoints | 4                | 18                      |
//! | [`market`]      | Market endpoints      | 6                | 5                       |
//! | [`universe`]    | Universe endpoints    | 1                |                         |
//!
//! The rest of the declared endpoints submodules have yet to have any endpoints implemented.
//!
//! ## ESI Documentation
//! - ESI API Explorer: <https://developers.eveonline.com/api-explorer>
//! - Error Rate Limits: <https://developers.eveonline.com/docs/services/esi/best-practices/#error-limit>
//!
//! ## ESI Error Rate Limits
//! ESI imposes a rate limit if your application's requests return too many errors as documented
//! [here](https://developers.eveonline.com/docs/services/esi/best-practices/#error-limit). This crate implements
//! measures to reduce potential errors.
//!
//! Authenticated ESI routes will return a relevant error should one of the following cases occur:
//!
//! - [`crate::OAuthError::ValidateTokenError`]: Access token fails validation, either improperly formatted or wasn't created by EVE Online
//! - [`crate::OAuthError::AccessTokenExpired`]: Access token is expired
//! - [`crate::OAuthError::AccessTokenMissingScopes`]: Access token is missing the scopes required by the ESI endpoint
//!
//! ## Usage
//! ### Public ESI Endpoints
//! **Prerequisites:**
//! - **ESI Client:** Setup a basic ESI client as demonstrated in [`crate::client`] module docs
//!
//! ```no_run
//! use eve_esi::Client;
//!
//! // Fetch corporation information from a public ESI endpoint
//! async fn get_corporation_information(esi_client: Client, corporation_id: i64) {
//!     // Fetch corporation information with provided corporation_id
//!     let corporation = esi_client
//!         .corporation()
//!         .get_corporation_information(corporation_id)
//!         .send()
//!         .await
//!         .unwrap();
//! }
//! ```
//!
//! ### Authenticated ESI Endpoints
//! **Prerequisites:**
//! - **ESI Client:** Setup an ESI client for OAuth2 as demonstrated in [`crate::builder`] module docs
//! - **User Login:** You will need the character to login first in order to retrieve an access token
//!   using an authorization code. You will need a login route as demonstrated in the [`crate::oauth2::login`]
//!   module docs. Make sure you request the scopes required for the endpoint!
//! - **Access Token:** You will get this by getting a character's token in the callback route
//!   using the authorization code provided after login as demonstrated in the [`crate::oauth2::token`]
//!   module docs
//!
//! ```no_run
//! use eve_esi::Client;
//!
//! // Fetch character notifications from an authenticated ESI endpoint
//! async fn get_character_notifications(
//!     esi_client: Client,
//!     character_id: i64,
//!     access_token: &str
//! ) {
//!     // Get character notifications for character_id using the access_token
//!     let notifications = esi_client
//!         .character()
//!         .get_character_notifications(&access_token, character_id)
//!         .send()
//!         .await
//!         .unwrap();
//! }
//! ```

#[macro_use]
mod macros;

pub mod alliance;
pub mod assets;
pub mod calendar;
pub mod character;
pub mod clones;
pub mod contacts;
pub mod contracts;
pub mod corporation;
pub mod corporation_projects;
pub mod dogma;
pub mod faction_warfare;
pub mod fittings;
pub mod fleets;
pub mod incursions;
pub mod industry;
pub mod insurance;
pub mod killmails;
pub mod location;
pub mod loyalty;
pub mod mail;
pub mod market;
pub mod meta;
pub mod planetary_interaction;
pub mod routes;
pub mod search;
pub mod skills;
pub mod sovereignty;
pub mod status;
pub mod universe;
pub mod user_interface;
pub mod wallet;

use crate::Client;

use alliance::AllianceEndpoints;
use assets::AssetsEndpoints;
use calendar::CalendarEndpoints;
use character::CharacterEndpoints;
use clones::ClonesEndpoints;
use contacts::ContactsEndpoints;
use corporation::CorporationEndpoints;
use corporation_projects::CorporationProjectsEndpoints;
use dogma::DogmaEndpoints;
use faction_warfare::FactionWarfareEndpoints;
use fittings::FittingsEndpoints;
use fleets::FleetsEndpoints;
use incursions::IncursionsEndpoints;
use industry::IndustryEndpoints;
use insurance::InsuranceEndpoints;
use killmails::KillmailsEndpoints;
use location::LocationEndpoints;
use loyalty::LoyaltyEndpoints;
use mail::MailEndpoints;
use market::MarketEndpoints;
use meta::MetaEndpoints;
use planetary_interaction::PlanetaryInteractionEndpoints;
use routes::RoutesEndpoints;
use search::SearchEndpoints;
use skills::SkillsEndpoints;
use sovereignty::SovereigntyEndpoints;
use status::StatusEndpoints;
use universe::UniverseEndpoints;
use user_interface::UserInterfaceEndpoints;
use wallet::WalletEndpoints;

impl Client {
    /// Access to Alliance ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    ///
    /// Returns an API client for interacting with alliance-related endpoints.
    pub fn alliance(&self) -> AllianceEndpoints<'_> {
        AllianceEndpoints::new(self)
    }

    /// Access to assets ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    pub fn assets(&self) -> AssetsEndpoints<'_> {
        AssetsEndpoints::new(self)
    }

    /// Access to calendar ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    pub fn calendar(&self) -> CalendarEndpoints<'_> {
        CalendarEndpoints::new(self)
    }

    /// Access to Character ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    ///
    /// Returns an API client for interacting with character-related endpoints.
    pub fn character(&self) -> CharacterEndpoints<'_> {
        CharacterEndpoints::new(self)
    }

    /// Access to clones ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    pub fn clones(&self) -> ClonesEndpoints<'_> {
        ClonesEndpoints::new(self)
    }

    /// Access to contacts ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    pub fn contacts(&self) -> ContactsEndpoints<'_> {
        ContactsEndpoints::new(self)
    }

    /// Access to contracts ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn contracts(&self) -> ClonesEndpoints<'_> {
        ClonesEndpoints::new(self)
    }

    /// Access to Corporation ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    ///
    /// Returns an API client for interacting with corporation-related endpoints.
    pub fn corporation(&self) -> CorporationEndpoints<'_> {
        CorporationEndpoints::new(self)
    }

    /// Access to corporation projects ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn corporation_projects(&self) -> CorporationProjectsEndpoints<'_> {
        CorporationProjectsEndpoints::new(self)
    }

    /// Access to dogma ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn dogma(&self) -> DogmaEndpoints<'_> {
        DogmaEndpoints::new(self)
    }

    /// Access to faction warfare ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn faction_warfare(&self) -> FactionWarfareEndpoints<'_> {
        FactionWarfareEndpoints::new(self)
    }

    /// Access to fittings ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn fittings(&self) -> FittingsEndpoints<'_> {
        FittingsEndpoints::new(self)
    }

    /// Access to fleets ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn fleets(&self) -> FleetsEndpoints<'_> {
        FleetsEndpoints::new(self)
    }

    /// Access to incursions ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn incursions(&self) -> IncursionsEndpoints<'_> {
        IncursionsEndpoints::new(self)
    }

    /// Access to industry ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn industry(&self) -> IndustryEndpoints<'_> {
        IndustryEndpoints::new(self)
    }

    /// Access to insurance ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn insurance(&self) -> InsuranceEndpoints<'_> {
        InsuranceEndpoints::new(self)
    }

    /// Access to killmails ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn killmails(&self) -> KillmailsEndpoints<'_> {
        KillmailsEndpoints::new(self)
    }

    /// Access to location ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn location(&self) -> LocationEndpoints<'_> {
        LocationEndpoints::new(self)
    }

    /// Access to loyalty ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn loyalty(&self) -> LoyaltyEndpoints<'_> {
        LoyaltyEndpoints::new(self)
    }

    /// Access to mail ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn mail(&self) -> MailEndpoints<'_> {
        MailEndpoints::new(self)
    }

    /// Access to market ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    pub fn market(&self) -> MarketEndpoints<'_> {
        MarketEndpoints::new(self)
    }

    /// Access to meta ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn meta(&self) -> MetaEndpoints<'_> {
        MetaEndpoints::new(self)
    }

    /// Access to planetary interaction ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn planetary_interaction(&self) -> PlanetaryInteractionEndpoints<'_> {
        PlanetaryInteractionEndpoints::new(self)
    }

    /// Access to routes ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn routes(&self) -> RoutesEndpoints<'_> {
        RoutesEndpoints::new(self)
    }

    /// Access to search ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn search(&self) -> SearchEndpoints<'_> {
        SearchEndpoints::new(self)
    }

    /// Access to skills ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn skills(&self) -> SkillsEndpoints<'_> {
        SkillsEndpoints::new(self)
    }

    /// Access to sovereignty ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn sovereignty(&self) -> SovereigntyEndpoints<'_> {
        SovereigntyEndpoints::new(self)
    }

    /// Access to status ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn status(&self) -> StatusEndpoints<'_> {
        StatusEndpoints::new(self)
    }

    /// Access to universe ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    pub fn universe(&self) -> UniverseEndpoints<'_> {
        UniverseEndpoints::new(self)
    }

    /// Access to user interface ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn user_interface(&self) -> UserInterfaceEndpoints<'_> {
        UserInterfaceEndpoints::new(self)
    }

    /// Access to wallet ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    fn wallet(&self) -> WalletEndpoints<'_> {
        WalletEndpoints::new(self)
    }
}
