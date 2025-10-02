# EVE ESI

> [!WARNING]
>
> **This crate is still under development, APIs may change between versions**
>
> Currently the crate features OAuth2 login with EVE Online and we're now currently working on implementing all ESI endpoints. The goal is to implement new endpoints at a pace of 2-3 new categories per week, it takes about 15 minutes to implement each endpoint due to writing related structs, enums, documentation & integration tests. All endpoints should be implemented end of October/early November.
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

For usage examples, ESI client configuration, and logging configuration, please see the [documentation](https://docs.rs/eve_esi/latest/eve_esi/)

Have questions about this crate or EVE Online's ESI in general? Ask us in [Discord](https://discord.gg/HjaGsBBtFg)!

## Features

- **EVE Online ESI:** Ongoing implementation of every public & authenticated ESI endpoint at a goal pace of 10 endpoints/day (Should be completed by early October)
- **EVE Online OAuth2:** Features full implementation of OAuth2 single sign-on with EVE Online including out of the box JWT token key caching and refreshing ahead of expiration to validate access tokens.
- **Thread-safe:** Implements the usage of read/write locks, compare exchanges, atomic bools, & tokio notifiers to provide high concurrency performance in applications at scale.
- **Configurable:** Allows for the creation of simple ESI clients only for public endpoints, to clients created with a builder method for OAuth2, to providing a custom config to fine-tune settings to your application's needs.
- **Documentation:** The endpoints, models, & enums within this crate are all documented to help clarify what certain fields or enum variants are for, making it more accessible to developers unfamiliar with some areas of the game. Even the 250~ variants of the `NotificationType` enum have documentation.

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

## Endpoint Implementation Status

The following categories from the [ESI API Explorer](https://developers.eveonline.com/api-explorer) have been implemented:

| Category      | Description           | Public Endpoints | Authenticated Endpoints |
| ------------- | --------------------- | ---------------- | ----------------------- |
| `alliance`    | Alliance endpoints    | 4                |                         |
| `assets`      | Clone endpoints       |                  | 6                       |
| `calendar`    | Calendar endpoints    |                  | 4                       |
| `character`   | Character endpoints   | 3                | 9                       |
| `clones`      | Clone endpoints       |                  | 2                       |
| `contacts`    | Contact endpoints     |                  | 9                       |
| `corporation` | Corporation endpoints | 4                | 18                      |
| `market`      | Market endpoints      | 6                | 5                       |

New endpoints are being implemented at a pace of 2-3 new categories per week, all categories should be implemented end of October/early November.

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
