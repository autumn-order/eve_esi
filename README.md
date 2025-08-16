# EVE ESI

Rust API wrapper for interaction with [EVE Online's ESI](https://developers.eveonline.com/api-explorer).

## Usage

Create a new EsiClient instance and request public information about a character from ESI.

```rust
#[tokio::main]
async fn main() {
    let esi_client = eve_esi::EsiClient::builder()
        .user_agent("MyApp/1.0 (contact@example.com)")
        .build()
        .expect("Failed to build EsiClient");

    // Get information about the corporation The Order of Autumn (id: 98785281)
    let corporation = esi_client.corporation().get_corporation_information(98785281).await.unwrap();
    println!("Corporation name: {}", corporation.name);
}
```

Make certain you set the user agent as demonstrated above, ensure it includes contact email in case there are any issues with your ESI requests.

## Examples

### Axum

A basic example demonstrating how to use the `eve_esi` crate with the `axum` web framework to create an API that serves ESI data.

1. Run `cargo run --example axum`
2. Head to one of the URLs posted in your terminal, change the IDs to test out different characters/corporations.

### SSO

An example demonstrating how to use the `eve_esi` crate with the `axum` web framework to utilize EVE SSO authentication to login with EVE Online. This is a prerequisite for accessing private ESI routes.

1. Create a developer application on [EVE Online's Developer Portal](https://developers.eveonline.com/applications)
2. Set the callback URL to `http://localhost:8080/callback`
3. Copy .env.example to .env and set the CALLBACK_URL, EVE_ESI_CLIENT_ID, EVE_ESI_CLIENT_SECRET, & CONTACT_EMAIL variables
4. Run `cargo run --example sso`
5. Go to `http://localhost:8080/login` in your browser
6. Follow the login process, once authorized you'll be shown your character's information.

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
let esi_client = eve_esi::EsiClient::builder()
    .user_agent("MyApp/1.0 (contact@example.com)")
    .build()
    .expect("Failed to build EsiClient");
```

## Notes

- More ESI routes will be added as needed, feel free to submit pull requests to add any you may need.
- You can override the esi_url for the ESI Client by simply using `esi_client.esi_url = "http://your_url.com"` for use cases such as unit tests with crates such as [mockito](https://docs.rs/mockito/latest/mockito/) to emulate endpoints. See this repository's tests folder for examples.
