# EVE ESI

[![Crates.io Version](https://img.shields.io/crates/v/eve_esi?logo=rust)](https://crates.io/crates/eve_esi/)
[![codecov](https://codecov.io/gh/hyziri/eve_esi/graph/badge.svg?token=OXD57P1UY6)](https://codecov.io/gh/hyziri/eve_esi)
[![Maintainability](https://qlty.sh/gh/hyziri/projects/eve_esi/maintainability.svg)](https://qlty.sh/gh/hyziri/projects/eve_esi)

A thread-safe, asynchronous client which provides methods & types for interaction with [EVE Online's ESI](https://developers.eveonline.com/api-explorer) & [EVE Online's single sign-on (SSO)](https://developers.eveonline.com/docs/services/sso/).

This crate implements concurrency & caching to provide performance in applications at scale. For example JSON web token keys (JWT keys) are used to validate tokens after a successful EVE Online single sign-on login, this crate automatically caches the keys and refreshes them proactively before expiry in a background task for mimimal latency.

This crate is still heavily under development and has yet to implement the majority of ESI routes as well as the remainder of the OAuth2 flow such as token validation.

## Usage

Create a new ESI Client instance and request public information about a corporation from ESI.

```rust
// esi_client is asynchronous, #[tokio::main] allows for making the main function async
// You would ideally use esi_client with an async web framework like Axum as shown in examples
#[tokio::main]
async fn main() {
    // Build a new ESI Client with the builder method
    let esi_client = eve_esi::Client::builder()
        // Always set a user agent to identify your application
        .user_agent("MyApp/1.0 (contact@example.com)")
        .build()
        .expect("Failed to build Client");

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
  .user_agent("MyApp/1.0 (contact@example.com)")
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

Anything single sign-on/oauth2 related is accessed with:

```rust
esi_client.oauth2()
```

Creating a URL to redirect your users to the login process would be:

```rust
// Set scopes to request
let scopes = eve_esi::ScopeBuilder::new()
  .public_data()
  .build();

// Create a login URL
let login = esi_client
  .oauth2()
  .login_url(scopes)
  .expect("Failed to create a login url"); // Errors if OAuth2 is not configured on client
```

TODO: Validate token

### Making Authenticated ESI Requests

TODO: Work in progress

## Examples

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

TODO: First half of the login flow is done, second half with the callback route & token validation is work in progress.

## Logging

This library uses the [`log`](https://crates.io/crates/log) crate for logging. To capture log output,
applications using this library should initialize a logger implementation like `env_logger`,
`simple_logger`, or any other implementation of the `log` crate's facade.

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
    .user_agent("MyApp/1.0 (contact@example.com)")
    .build()
    .expect("Failed to build Client");
```
