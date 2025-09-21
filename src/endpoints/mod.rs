//! # EVE ESI Endpoints
//!
//! This module provides access to the different categories of endpoints available for EVE Online's ESI API.
//!
//! ## Modules
//! - [`alliance`]: Alliance endpoints (4/4 endpoints)
//! - [`character`]: Character endpoints (11/11 endpoints)
//! - [`corporation`]: Corporation endpoints (22/22 endpoints)
//! - [`market`]: Market endpoints (11/11 endpoints)
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
use character::CharacterEndpoints;
use corporation::CorporationEndpoints;
use market::MarketEndpoints;

impl Client {
    /// Access to Alliance ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    ///
    /// Returns an API client for interacting with alliance-related endpoints.
    pub fn alliance(&self) -> AllianceEndpoints<'_> {
        AllianceEndpoints::new(self)
    }

    /// Access to Character ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    ///
    /// Returns an API client for interacting with character-related endpoints.
    pub fn character(&self) -> CharacterEndpoints<'_> {
        CharacterEndpoints::new(self)
    }

    /// Access to Corporation ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    ///
    /// Returns an API client for interacting with corporation-related endpoints.
    pub fn corporation(&self) -> CorporationEndpoints<'_> {
        CorporationEndpoints::new(self)
    }

    /// Access to market ESI endpoints
    ///
    /// For an overview & usage example, see the [endpoints module documentation](super)
    pub fn market(&self) -> MarketEndpoints<'_> {
        MarketEndpoints::new(self)
    }
}
