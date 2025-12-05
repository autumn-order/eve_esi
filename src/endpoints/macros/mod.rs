//! # EVE ESI Endpoint Request Macros
//!
//! Provides the `define_esi_endpoint!` macro which is used to concisely define ESI endpoints that return
//! `EsiRequest<T>` builders. This allows endpoints to be defined with only the information that varies between them.
//!
//! During compilation, this macro expands into a function that builds an `EsiRequest<T>` with the appropriate
//! URL (including path and query parameters), HTTP method, access token (for authenticated endpoints), and
//! required scopes. Users can then customize the request with additional headers before calling `.send()` or
//! `.send_with_cache()` to execute it.
//!
//! ## Endpoint Variants
//!
//! | Variant  | Required Arguments                      |
//! | -------- | --------------------------------------- |
//! | `pub fn` | None (public endpoints)                 |
//! | `auth fn`| `access_token: &str` (authenticated)    |
//!
//! ## HTTP Methods
//!
//! The macro supports all HTTP methods via `reqwest::Method`:
//! - `Method::GET` - Retrieve resources
//! - `Method::POST` - Create resources or submit data
//! - `Method::PUT` - Update resources
//! - `Method::DELETE` - Delete resources
//!
//! ## Usage
//!
//! Use existing endpoints defined with `define_esi_endpoint!` as a guideline. The macro must be used
//! within an `impl` block of a struct that contains a `client` field of type `&'a Client` (the [`crate::Client`]).
//!
//! ### URL Path & Query Parameters
//!
//! The macro is flexible with URL parameters:
//! - **Path parameters**: Listed first after `access_token` (for authenticated endpoints)
//! - **Query parameters**: Separated by a `;` semicolon after path parameters
//! - **Body parameters**: Specified with `body = name: Type;` syntax
//!
//! ### Required Components
//!
//! All endpoints must specify:
//! - `method = Method::XXX;` - The HTTP method to use
//! - `url = "...";` - The URL template with `{}` for path parameters
//! - `required_scopes = ...;` - For authenticated endpoints only
//!
//! ### Example: Public Endpoint
//!
//! ```ignore
//! define_esi_endpoint! {
//!     /// Retrieves information about a specific market group
//!     ///
//!     /// # Arguments
//!     /// - `market_group_id` (`i64`): The ID of the market group
//!     ///
//!     /// # Returns
//!     /// An ESI request builder that returns market group information when sent.
//!     pub fn get_market_group_info(
//!         market_group_id: i64
//!     ) -> EsiRequest<MarketGroupInfo>
//!     method = Method::GET;
//!     url = "{}/markets/groups/{}";
//! }
//! ```
//!
//! ### Example: Authenticated Endpoint with Query Parameters
//!
//! ```ignore
//! define_esi_endpoint! {
//!     /// Fetches a character's contacts
//!     ///
//!     /// # Arguments
//!     /// - `access_token` (`&str`): Access token for authentication
//!     /// - `character_id` (`i64`): The character's ID
//!     /// - `page` (`i32`): Page number for pagination
//!     ///
//!     /// # Returns
//!     /// An ESI request builder that returns a list of contacts when sent.
//!     auth fn get_contacts(
//!         access_token: &str,
//!         character_id: i64;
//!         page: i32
//!     ) -> EsiRequest<Vec<Contact>>
//!     method = Method::GET;
//!     url = "{}/characters/{}/contacts";
//!     required_scopes = ScopeBuilder::new()
//!         .characters(CharactersScopes::new().read_contacts())
//!         .build();
//! }
//! ```
//!
//! ### Example: Authenticated POST with Body
//!
//! ```ignore
//! define_esi_endpoint! {
//!     /// Adds contacts for a character
//!     ///
//!     /// # Arguments
//!     /// - `access_token` (`&str`): Access token for authentication
//!     /// - `character_id` (`i64`): The character's ID
//!     /// - `contact_ids` (`Vec<i64>`): List of contact IDs to add
//!     /// - `standing` (`f64`): Standing value for the contacts
//!     ///
//!     /// # Returns
//!     /// An ESI request builder that returns a list of added contact IDs when sent.
//!     auth fn add_contacts(
//!         access_token: &str,
//!         character_id: i64;
//!         standing: f64
//!     ) -> EsiRequest<Vec<i64>>
//!     method = Method::POST;
//!     url = "{}/characters/{}/contacts";
//!     required_scopes = ScopeBuilder::new()
//!         .characters(CharactersScopes::new().write_contacts())
//!         .build();
//!     body = contact_ids: Vec<i64>;
//! }
//! ```
//!
//! ## Return Type
//!
//! All endpoints return `EsiRequest<T>` where `T` is the expected response type when deserialized.
//! Users must call `.send()` or `.send_with_cache()` on the returned builder to execute the request.

#[macro_use]
mod endpoint;
