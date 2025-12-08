//! EVE ESI Basic Example
//!
//! This example demonstrates initiating an ESI client for public requests
//! and using the ESI client to fetch a character's information from ESI.

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
    let response = match esi_client
        .character()
        .get_character_public_information(character_id)
        .send()
        .await
    {
        Ok(character) => character,
        // Early return an error if fetching character information fails
        Err(error) => return Err(error),
    };

    // Full response
    println!("Response Data: {:#?}", response);

    // Use `data` method to access only character information
    println!("Character Info: {:#?}", response);

    Ok(())
}
