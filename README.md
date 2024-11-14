# EVE ESI

Rust API wrapper for interaction with [EVE Online's ESI](https://esi.evetech.net/ui/).

## Usage

```rust
let user_agent = format!("APPLICATION_NAME/1.0 (example@example.com)");
let esi_client = eve_esi::Client::new(&user_agent);

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
- You can override the esi_url for the ESI Client by simply doing `esi_client.esi_url = "http://your_url.com" for use cases such as unit tests with crates like [mockito](https://docs.rs/mockito/latest/mockito/) to emulate endpoints, see this repository's unit tests for an example.
