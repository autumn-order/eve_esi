# EVE ESI

[![Crates.io Version](https://img.shields.io/crates/v/eve_esi?logo=rust)](https://crates.io/crates/eve_esi/)
[![codecov](https://codecov.io/gh/hyziri/eve_esi/graph/badge.svg?token=OXD57P1UY6)](https://codecov.io/gh/hyziri/eve_esi)
[![Maintainability](https://qlty.sh/gh/hyziri/projects/eve_esi/maintainability.svg)](https://qlty.sh/gh/hyziri/projects/eve_esi)

A thread-safe, asynchronous client which provides methods & types for interaction with [EVE Online's ESI](https://developers.eveonline.com/api-explorer) & [EVE Online's single sign-on (SSO)](https://developers.eveonline.com/docs/services/sso/).

This crate implements concurrency & caching to provide performance in applications at scale. For example JSON web token keys (JWT keys) are used to validate tokens after a successful EVE Online single sign-on login, this crate automatically caches the keys and refreshes them proactively before expiry in a background task for mimimal latency.

This crate is still heavily under development and has yet to implement the majority of ESI routes.

## Usage

Create a new ESI Client instance and request public information about a corporation from ESI.

```rust
// esi_client is asynchronous, #[tokio::main] allows for making the main function async
// You would ideally use esi_client with an async web framework like Axum as shown in examples
#[tokio::main]
async fn main() {
    // Set a user_agent to identify your application when making requests
    let user_agent = "MyApp/1.0 (contact@example.com; +https://github.com/your/repository)";

    // Create a basic ESI client with user_agent
    let esi_client = eve_esi::Client::new(user_agent).expect("Failed to build Client");

    // Get information about the corporation The Order of Autumn (id: 98785281)
    let corporation = esi_client.corporation().get_corporation_information(98785281).await.unwrap();

    println!("Corporation name: {}", corporation.name);
}
```

## Quickstart

### Building a Client

You can build an ESI client with the builder method:

```rust
let esi_client = eve_esi::Client::builder()
  // Always set a user agent to identify your application
  .user_agent("MyApp/1.0 (contact@example.com; +https://github.com/your/repository)")
  // Optional: Set these 3 to configure for single sign-on login & authenticated ESI routes
  // Get them from https://developers.eveonline.com/applications
  .client_id("client_id")
  .client_secret("client_secret")
  .callback_url("http://localhost:8080/callback") // This would be an API endpoint on your app, see SSO example
  .build()
  .expect("Failed to build ESI Client");
```

For ideal performance, if you are already using a `reqwest::Client` in your application you should `.clone()` it when building an `eve_esi::Client` to share the same HTTP pool rather than `eve_esi::Client` using its own `reqwest::Client` by default:

```rust
let esi_client = eve_esi::Client::builder()
  .reqwest_client(reqwest_client.clone())
```

### Making Public ESI Requests

This library mirrors the [EVE ESI API explorer](https://developers.eveonline.com/api-explorer) in that you can access endpoints by the format of:

```rust
esi_client.category_name().method_name()

// This would translate to:
let alliances = esi_client.alliance().list_all_alliances()
```

### OAuth2 (SSO) Login

To access any authenticated ESI routes, your users will first need to sign-in using EVE Online's single-sign on (SSO), also known as OAuth2.
A complete SSO example can be found at <https://github.com/hyziri/eve_esi/blob/main/examples/sso.rs>.

Anything single sign-on/oauth2 related is accessed with:

```rust
esi_client.oauth2()
```

Login route: create a URL to redirect your users for the login process:

```rust
// Set scopes to request
let scopes = eve_esi::ScopeBuilder::new()
  .public_data() // publicData scope
  .build();

// Create a login URL
let login = esi_client
  .oauth2()
  .login_url(scopes)
  .expect("Failed to create a login url"); // Errors if OAuth2 is not configured on ESI Client

let login_url = login.login_url; // Redirect users to this URL to begin the login process
let state = login.state; // State code for preventing CSRF which you should validate in your callback route
```

Callback route: retrieving a token, getting the access & refresh token:

```rust
// Use the authorization code present as the {?code=...} query parameter in your callback route URL
// See https://github.com/hyziri/eve_esi/blob/main/examples/sso.rs for callback route example
let token = esi_client
    .oauth2()
    .get_token(authorization_code)
    .await
    .expect("Failed to fetch token");

let access_token = token.access_token();
let refresh_token = token.refresh_token();

// Refresh token can be converted to a String this way
let refresh_token_string = refresh_token.unwrap().secret().to_string();
```

Callback route: validating a token, accessing character ID & name:

```rust
// Validate the token to access the claims
let claims = esi_client
    .oauth2()
    .validate_token(token.access_token().secret().to_string())
    .await
    .expect("Failed to validate token");

// Access character ID & name

// claims.sub looks like a String: "CHARACTER:EVE:2114794365"
// We'll extract the ID part and convert it to an i32
let id_str = claims.sub.split(':').collect::<Vec<&str>>()[2];

let character_id: i32 = id_str.parse().expect("Failed to parse id to i32");
let character_name: String = claims.name;
```

Refreshing a token:

```rust
// In this scenario, refresh token was stored in database & retrieved as a string
let refresh_token = "refresh_token_string...".to_string();

// Call `get_token_refresh` and pass the refresh token string to get a new token
let token = esi_client
    .oauth2()
    .get_token_refresh(refresh_token)
    .await
    .expect("Failed to fetch token");

// Validate the token here and replace the old access/refresh token stored in the database...
```

### Making Authenticated ESI Requests

TODO: Work in progress

## Examples

If you wish to see logs for the steps of how the crate works in the examples, prefix the `cargo run --example sso` command like so `RUST_LOG=eve_esi=debug cargo run --example sso`.

### Axum

A basic example demonstrating how to use the `eve_esi` crate with the `axum` web framework to create an API that serves ESI data.

1. Run `cargo run --example axum`
2. Head to one of the URLs posted in your terminal, change the URL IDs in your browser to test out different characters/corporations.

### SSO

An example demonstrating how to use the `eve_esi` crate with the `axum` web framework to utilize EVE SSO authentication to login with EVE Online. This is a prerequisite for accessing private ESI routes.

1. Create a developer application on [EVE Online's Developer Portal](https://developers.eveonline.com/applications)
2. Set the callback URL to `http://localhost:8080/callback`
3. Copy .env.example to .env and set the CALLBACK_URL, EVE_ESI_CLIENT_ID, EVE_ESI_CLIENT_SECRET, & CONTACT_EMAIL variables
4. Run `cargo run --example sso`
5. Go to `http://localhost:8080/login` in your browser
6. Login with EVE Online, you'll then be redirected to `http://localhost:8080/callback`
7. Internally, the callback route will fetch a JWT token using the authorization code from login. The callback response will show your character ID & name.

## Logging

This library uses the [`log`](https://crates.io/crates/log) crate for logging. To capture log output,
applications using this library should initialize a logger implementation like `env_logger`,
`simple_logger`, or any other implementation of the `log` crate's facade.

For production, you'll generally want to use log level either `info` or `warn` depending on how much you wish to rely on this crate's logging for your application.

### Log Levels

- **Error**: Used for failures that prevent successful API calls
- **Warn**: Used for potential issues that don't prevent operation but could be problematic
- **Info**: Used for successful API calls and important client state changes
- **Debug**: Used for detailed information about API call parameters and responses
- **Trace**: Used for very detailed debugging information

### Example with env_logger

```rust
// Set RUST_LOG environment variable to control log levels
// e.g., RUST_LOG=eve_esi=debug,info

// Initialize env_logger
env_logger::init();

// Now logs from eve_esi will be captured
let esi_client = eve_esi::Client::builder()
    .user_agent("MyApp/1.0 (contact@example.com; +https://github.com/your/repository)")
    .build()
    .expect("Failed to build Client");
```
