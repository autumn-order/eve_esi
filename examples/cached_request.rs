//! EVE ESI Cached Request Example
//!
//! This example demonstrates how to properly update data from ESI, utilizing a last updated
//! timestamp to conditionally get information from ESI depending on if it has been updated
//! since it was last fetched.
//!
//! The benefit is decreased ESI token usage to avoid the risk of getting rate limited. Every
//! 2xx response uses 2 tokens while a 304 not modified response only consumes 1 token as it incurs
//! less of a resource strain on ESI to not have to return the entire model for every request.

use chrono::Utc;
use eve_esi::{CacheStrategy, CachedResponse};

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

    // Fetch character for the first time - 2 tokens used on OK response
    //
    // In this instance, we have no information on the character at all in
    // our application so we would NOT use cached headers here because we need
    // the info regardless of when it was last modified.
    let initial_character = match esi_client
        .character()
        .get_character_public_information(character_id)
        .send()
        .await
    {
        Ok(character) => character,
        // Early return an error if fetching character information fails
        Err(error) => return Err(error.into()),
    };

    // Now, we would store our character in a database with a timestamp of when we last
    // updated them.
    let last_updated = Utc::now();

    // Retrieve character to update information, this time including our last modified timestamp
    // so ESI can use a less resource intensive not modified response if no change has occurred since
    // last request.
    let cache_result = match esi_client
        .character()
        .get_character_public_information(character_id)
        // Send the timestamp of when we last updated the character, returns not modified
        // if character info hasn't changed since we last updated them.
        .send_cached(CacheStrategy::IfModifiedSince(last_updated))
        .await
    {
        Ok(result) => result,
        // Early return an error if fetching character information fails
        Err(error) => return Err(error.into()),
    };

    // Determine if we have updated information since last cache request
    let character = match cache_result {
        // We return the updated character from ESI (updating our database entry in a real application)
        //
        // Note: this case wouldn't occur in this example as we just fetched information that has a 30
        // day cache time, in a real application we'd wait until after the 30 day cache window expires to
        // fetch again.
        CachedResponse::Fresh(updated_character) => updated_character,
        // We return the initially fetched character as no information has since changed
        CachedResponse::NotModified => initial_character,
    };

    println!("{:#?}", character);

    Ok(())
}
