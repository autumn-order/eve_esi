![Autumn Banner](https://raw.githubusercontent.com/autumn-order/branding/refs/heads/main/autumn-github-banner-dark.png#gh-light-mode-only)
![Autumn Banner](https://raw.githubusercontent.com/autumn-order/branding/refs/heads/main/autumn-github-banner-light.png#gh-dark-mode-only)

# EVE ESI

Rust API wrapper for interaction with [EVE Online's ESI](https://esi.evetech.net/ui/).

## Usage

```rust
let user_agent = format!("APP_NAME/1.0 (contact@example.com)");
let esi_client = eve_esi::Client::new(&user_agent);

let character_id: i32 = 2114794365;

let character: eve_esi::model::Character = esi_client.characters().get_character(character_id).await.unwrap();

println!(character);
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

## Notes

- More ESI routes will be added as needed, feel free to submit pull requests to add any you may need.
- Only public ESI routes are available, private routes will be added at a later date when required by Rust based applications built by [Autumn](https://github.com/autumn-order).
- You can override the esi_url for the ESI Client by simply doing `esi_client.esi_url = "http://your_url.com" for use cases such as unit tests with crates such as [mockito](https://docs.rs/mockito/latest/mockito/) to emulate endpoints, see this repository's tests folder for examples.
