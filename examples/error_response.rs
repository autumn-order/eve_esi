//! EVE ESI Error Response Example
//!
//! This example demonstrates handling an error response such as HTTP status code
//! 4xx indicating a user error or a 5xx internal server error indicating ESI is unavailable.

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

    // Invalid character ID to get information for
    let non_existant_character_id: i64 = 1;

    // Use let else syntax to early return if we don't get the error we are expecting
    let Err(eve_esi::Error::EsiError(error)) = esi_client
        .character()
        .get_character_public_information(non_existant_character_id)
        .send()
        .await
    else {
        panic!("Expected an EsiResponseError, got different error")
    };

    // Check for 4xx client errors (e.g., 400, 404, 429)
    if (400..500).contains(&error.status) {
        println!("Client error (4xx): Status {}", error.status);
        println!("Error message: {}", error.message);
    }

    // Check for 5xx server errors (e.g., 500, 502, 503)
    if (500..600).contains(&error.status) {
        println!("Server error (5xx): Status {}", error.status);
        println!("Error message: {}", error.message);
        println!("ESI may be experiencing issues");
    }

    // Alternative: Using match with range patterns
    match error.status {
        // Handle rate limited error differently than other client errors, such as pushing
        // an update job back into queue until the retry after time has elapsed
        429 => {
            // Retry after header will only be present on status code 429 rate limited
            if let Some(retry_after) = error.retry_after {
                // Seconds to wait until tokens have replenished for another request
                println!("Retry after {:?} seconds", retry_after)
            };
        }
        400..=499 => println!("Client error range using match: {}", error.status),
        500..=599 => println!("Server error range using match: {}", error.status),
        _ => println!("Other status code: {}", error.status),
    }

    Ok(())
}
