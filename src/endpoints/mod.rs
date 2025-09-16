//! # EVE ESI Endpoints
//!
//! This module provides access to the different categories of endpoints available for EVE Online's ESI API.
//!
//! ## Modules
//! - [`alliance`]: Alliance endpoints
//! - [`character`]: Character endpoints
//! - [`corporation`]: Corporation endpoints
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
//! - **ESI Client:** Setup a basic ESI client as demonstrated in [`crate::builder`] module docs
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
//! - **User Login:** You will need the character to login first in order to get an access token
//!   using an authorization code. You will need a login route as demonstrated in the [`crate::oauth2::login`]
//!   module docs.
//! - **Access Token:** You will get this by getting a character's token in the callback route
//!   using the authorization code provided after login as demonstrated in the [`crate::oauth2::token`]
//!   module docs
//!
//! ```no_run
//! use eve_esi::Client;
//!
//! // Fetch character research agents from an authenticated ESI endpoint
//! async fn get_character_research_agents(
//!     esi_client: Client,
//!     character_id: i64,
//!     access_token: &str
//! ) {
//!     // Get character research agents for character_id using the access_token
//!     let research_agents = esi_client
//!         .character()
//!         .get_agents_research(&access_token, character_id)
//!         .await
//!         .unwrap();
//! }
//! ```

#[macro_use]
mod macros;

pub mod alliance;
pub mod character;
pub mod corporation;
pub mod endpoints;
