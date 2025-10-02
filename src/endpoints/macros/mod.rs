//! # EVE ESI Endpoint Request Macros
//!
//! Provides the `define_endpoint!` macro which is used to concisely define ESI endpoints in a way that only includes
//! the info that varies between each endpoints.
//!
//! During compilation this macro is then expanded into an actual function which includes all the necessary logic to
//! build an ESI request URL with path & query parameters, making the request with an HTTP client, and handling
//! logging of any debug, info, and error messages related to the request.
//!
//! ## Endpoint Variants
//!
//! | Variant        | Required Arguments                      |
//! | -------------- | --------------------------------------- |
//! | `pub_get`      |                                         |
//! | `pub_post`     | `body_name: body_type`                  |
//! | `auth_get`     | `&access_token`                         |
//! | `auth_post`    | `&access_token`, `body_name: body_type` |
//! | `auth_put`     | `&access_token`, `body_name: body_type` |
//! | `auth_delete`  | `&access_token`                         |
//!
//! ## Usage
//!
//! Use the already defined endpoints with `define_endpoint!` as a guideline as to how it varies between the different
//! request types, for example public requests do not utilize an `access_token` or `required_scopes`.
//!
//! The `define_endpoint!` macro must be defined as an impl of a struct which contains a `client` field with the type
//! `&'a Client` (The [`crate::Client`]).
//!
//! ### URL Path & Query Parameters
//! The `define_endpoint!` macro is flexible when it comes to URL parameters in that it permits not having any path
//! or query parameters at all, only path parameters, only query parameters, or both.
//! - Following the `access_token` for authenticated endpoints & any POST/PUT body (e.g. `contact_ids` in the example below),
//! the path parameters are then defined.
//! - A `;` is used as a delimiter to denote where the required endpoint method arguments & path parameters end and the query paramters begin
//!
//! ### Example
//!
//! ```ignore
//! define_endpoint! {
//!     // Any function documentation will go here
//!     /// Example function documentation starting with `///`
//!     // Set the endpoint variant to `auth_post` for an authenticated POST request
//!     // Set function to be named as `add_contacts`
//!     auth_post add_contacts(
//!         // Set access_token argument required for all authenticated routes
//!         access_token: &str,
//!         // Set post request body argument
//!         contact_ids: Vec<i64>,
//!         // Set URL path parameter argument
//!         // The `;` marks the end of the URL path parameters and the start of the query parameters
//!         character_id: i64;
//!         // Set URL query parameter arguments
//!         standing: f64,
//!         label_ids: Vec<i64>,
//!         watched: bool,
//!     // Set return type for the function
//!     ) -> Result<Vec<i64>, Error>
//! // Set the endpoint URL with `{}` representing the path parameters
//! url = "{}/characters/{}/contacts";
//! // Set the label text used in logging messages
//! label = "add contacts";
//! // Set the required ESI scopes to be checked for when access token is validated prior to request
//! // This prevents incurring ESI rate limits by preventing potential errors on the client side
//! required_scopes =  ScopeBuilder::new()
//!     .characters(CharactersScopes::new().write_contacts())
//!     .build();
//! }
//! ```

#[macro_use]
mod logging;
#[macro_use]
mod endpoint;
