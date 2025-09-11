//! # EVE ESI Endpoints
//!
//! This module provides access to the different categories of endpoints available for EVE Online's ESI API.
//!
//! # ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! # Modules
//! - [`alliance`]: Alliance endpoints
//! - [`character`]: Character endpoints
//! - [`corporation`]: Corporation endpoints
//!
//! # Usage
//! ## Public ESI Endpoints
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
//! ## Authenticated ESI Endpoints
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
//!         .get_agents_research(character_id, access_token)
//!         .await
//!         .unwrap();
//! }
//! ```

pub mod alliance;
pub mod character;
pub mod corporation;
pub mod endpoints;
