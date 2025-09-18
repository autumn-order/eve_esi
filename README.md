# EVE ESI

> [!WARNING]
>
> **This crate is still under development, APIs may change between versions**
>
> Currently the crate features OAuth2 login with EVE Online and we're now currently working on implementing all ESI endpoints. The goal is to implement an average of around 10 endpoints per day, it takes about 15 minutes to implement each endpoint due to writing related structs, enums, documentation & integration tests. All endpoints are expected to be added around early October at this rate.
>
> Documentation for this crate is still work-in-progress, if you see any issues or areas for improvement in documentation contributions are always welcome to help make this crate more accessible to other developers.

[![Crates.io Version](https://img.shields.io/crates/v/eve_esi?logo=rust)](https://crates.io/crates/eve_esi/)
[![codecov](https://codecov.io/gh/hyziri/eve_esi/graph/badge.svg?token=OXD57P1UY6)](https://codecov.io/gh/hyziri/eve_esi)
[![Maintainability](https://qlty.sh/gh/hyziri/projects/eve_esi/maintainability.svg)](https://qlty.sh/gh/hyziri/projects/eve_esi)
[![wakatime](https://wakatime.com/badge/github/hyziri/eve_esi.svg)](https://wakatime.com/badge/github/hyziri/eve_esi)
[![Discord](https://img.shields.io/discord/1414000815017824288?logo=Discord&color=%235865F2)](https://discord.gg/HjaGsBBtFg)

A thread-safe, asynchronous client which provides methods & types for interaction with [EVE Online's ESI](https://developers.eveonline.com/api-explorer) & [EVE Online's single sign-on (SSO)](https://developers.eveonline.com/docs/services/sso/).

**Documentation:** https://docs.rs/eve_esi/latest/eve_esi/

**Contributing:** https://github.com/hyziri/eve_esi/blob/main/CONTRIBUTING.md

**Discord:** https://discord.gg/HjaGsBBtFg

For usage examples, ESI client configuration, and logging configuration, please see the [documentation](https://docs.rs/eve_esi/latest/eve_esi/)

Have questions about this crate or EVE Online's ESI in general? Ask us in [Discord](https://discord.gg/HjaGsBBtFg)!

## Features

**EVE Online Single Sign-On**

EVE Online's single sign-on (OAuth2) login flow has been fully implemented in the crate featuring:

- Configuring of an EVE ESI client with an EVE Online developer application client id, client secret, and callback URL
- Creation of login URLs to begin the OAuth2 login flow
- The fetching, caching, & proactive refreshing of JWT token keys used to validate access tokens
- Access token fetching, validation, & refreshing

**EVE Online ESI Endpoints**

The implementation of ESI endpoints within this crate is still ongoing, we are aiming for all to be added by early October. So far the following 3 [ESI API Explorer](https://developers.eveonline.com/api-explorer) categories have been implemented:

- Character endpoints
- Corporation endpoints
- Alliance endpoints

Going forward a new version of this crate will be released with the implementation of a new category up until version 0.5.0 which indicates all endpoints have been added.

**Thread-safe**

This crate implements the usage of read/write locks, compare exchanges, atomic bools, & tokio notifiers to provide performance in applications at scale.

- **Refresh Locks:** The usage of atomic bools allows for the acquisition of a refresh lock on the JWT token key cache used to validate access tokens so that only 1 thread actually performs the cache refresh
- **High Concurrency:** The refresh lock is acquired via a compare exchange which is performance efficient in high concurrency applications where dozens of worker threads may attempt to acquire this refresh lock at once
- **Refresh Notifications:** While the refresh lock is in progress and the cache is currently expired or empty meaning no keys are available for validation, threads will wait for a tokio notifier notification that the refresh has completed. This makes it so that threads wait no longer than necessary for a completed refresh.
- **Read/Write Locks:** The JWT key cache utilizes read/write locks for minimal performance impact when accessing the JWT keys in the cache across multiple threads

The ESI client provided by this crate by default is wrapped within an Arc (atomic reference counter) which allows for the ESI client to be safely shared across threads.

- **Asynchronous:** This crate utilizes Rust ecosystem tools such as [Reqwest](https://crates.io/crates/reqwest) & [Tokio](https://crates.io/crates/tokio) to make ESI requests asynchronously and with minimal latency.

- **Caching:** Caching is utilized where possible to do to mimize latency with the OAuth2 login flow for EVE Online, the JWT token keys used to validate access tokens are cached for up to one hour and refreshed proactively before expiration to avoid any delay when validating tokens.

- **Type-Safe:** In addition to Rust's strong type safety, this crate additionally defines enums for ESI models in every area where it is applicable & possible to do so to make clear what exactly one can expect from ESI responses.

- **Documentation:** The endpoints, models, & enums within this crate are all documented to help clarify what certain fields or enum variants are for, making it more accessible to developers unfamiliar with some areas of the game. Even the 250~ variants of the `NotificationType` enum have documentation.

- **Testing:** All functions are unit tested and all endpoints implemented within this crate are integration tested to ensure proper error responses, handling of parameters & URLs, and deserialization of ESI JSON responses to the models defined within this crate.

- **Logging:** All functions & endpoints integrate the usage of the [log crate](https://crates.io/crates/log) to provide insight to the inner workings of this crate at runtime and help narrow down the source of any issues that may occur. Logging is configurable and opt-in, from detailed trace logging to only essential info logging.

- **Configuration:** The ESI client provided by this crate is configurable, from a basic client with only a user agent for public ESI to a client for OAuth2 to fine-tuning the inner workings of the client such as the parameters & timings of how JWT tokens are cached and refreshed

## Usage Example

Create a new ESI Client instance and request public information about a corporation from ESI.

```rust
// esi_client is asynchronous, #[tokio::main] allows for making the main function async
#[tokio::main]
async fn main() {
    // Set a user_agent to identify your application when making requests
    let user_agent = "MyApp/1.0 (contact@example.com; +https://github.com/your/repository)";

    // Create a basic ESI client with user_agent
    let esi_client = eve_esi::Client::new(user_agent).expect("Failed to build ESI Client");

    // Get information about the corporation The Order of Autumn (id: 98785281)
    let corporation = esi_client.corporation().get_corporation_information(98785281).await.unwrap();

    println!("Corporation name: {}", corporation.name);
}
```

For more usage examples, ESI client configuration, and logging configuration, please see the [documentation](https://docs.rs/eve_esi/latest/eve_esi/)

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
7. The callback route will fetch a JWT token using the authorization code from login and then return a response with your character ID & name after validating the token
