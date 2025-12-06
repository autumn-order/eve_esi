//! EVE ESI Custom Request Example
//!
//! This example demonstrates how to make a custom ESI request using the ESI client by
//! defining the struct of the ESI response, setting the endpoint to make the request
//! to, and defining the HTTP request method (GET, PUT, POST, DELETE).

use eve_esi::EsiResponse;
use reqwest::Method;
use serde::Deserialize;

// Derive Deserialize for the struct the response will be deserialized to
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Status {
    players: i64,
    server_version: String,
    start_time: String,
    // ESI does not return this field in the response if it is false
    //
    // use #[serde(default)] to default to false if field isn't present
    #[serde(default)]
    vip: bool,
}

#[tokio::main]
async fn main() -> Result<(), eve_esi::Error> {
    // Always set a user agent for your ESI client
    // For production apps, ensure it contains a contact email in case anything goes wrong with your ESI requests
    // E.G. "MyApp/1.0 (contact@example.com; +https://github.com/your/repository)"
    let user_agent: String = format!(
        "{}/{} (+{})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_REPOSITORY")
    );

    // Create a basic ESI client with a user agent to identify your application
    let esi_client = eve_esi::Client::new(&user_agent)?;

    // Set the expected response type to EsiResponse<Status> so the client knows the struct
    // to deserialize the response to.
    let response: EsiResponse<Status> = esi_client
        .esi()
        // Create a new request, set the endpoint to `https://esi.evetech.net/status`
        .new_request("https://esi.evetech.net/status")
        // Modify the request method using `with_method`, by default the method is GET
        .with_method(Method::GET)
        // Use `with_access_token` method if it is an authenticated endpoint
        // Make the request using the `send` method
        .send()
        .await?;

    println!("{:#?}", response);

    Ok(())
}
