//! EVE ESI Request Headers Example
//!
//! This example demonstrates how to modify request headers when making an ESI request
//! for the purposes of changing the language of the response, setting the tenant server the
//! data comes from, or providing a custom header with the `with_header` method.

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

    // Character ID to get information for
    let character_id: i64 = 2114794365;

    // Set the endpoint to make the request to
    let mut request = esi_client
        .character()
        .get_character_public_information(character_id)
        // Set headers in the original method chain
        .with_language(eve_esi::Language::English);

    // Or modify headers conditionally
    if true {
        request = request.with_language(eve_esi::Language::English);
    }

    let response = match request.send().await {
        Ok(response) => response,
        // Return an error if fetching character information fails
        Err(error) => return Err(error),
    };

    println!("{:#?}", response);

    Ok(())
}
