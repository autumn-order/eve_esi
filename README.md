# EVE ESI

Rust API wrapper for interaction with [EVE Online's ESI](https://esi.evetech.net/ui/).

## Usage

```rust
let reqwest_client: reqwest::Client = reqwest::Client::new().header(reqwest::header::USER_AGENT, "APPLICATION_NAME/1.0 (APPLICATION_EMAIL)");
let esi_client: eve_esi::EsiClient<'_> = eve_esi::EsiClient::new(&reqwest_client);

let character_id: i32 = 2114794365;

let character: eve_esi::model::Character = esi_client.get_character(character_id).await.unwrap();

println!(character);
```

Make certain you set the user agent as demonstrated above, ensure it includes contact email in case there are any issues with your ESI requests.

## Examples

See the [axum](https://github.com/blackrose-eve/eve_esi/tree/main/examples/axum.rs) example for a more complete usage demonstration.

To test out the example:

1. Run `cargo run --example axum`
2. Head to one of the URLs posted in your terminal, change the IDs to test out different characters/corporations.

## Notes

- More ESI routes will be added as needed, feel free to submit pull requests to add any you may need.
- Only public ESI routes are available, private routes will be added at a later date when required by Rust based applications built by [Autumn](https://github.com/autumn-order).
