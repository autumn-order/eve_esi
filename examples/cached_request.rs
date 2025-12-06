//! EVE ESI Cached Request Example
//!
//! This example demonstrates how to properly update data from ESI, utilizing a last updated
//! timestamp to conditionally get information from ESI depending on if it has been updated
//! since it was last fetched.
//!
//! The benefit is decreased ESI token usage to avoid the risk of getting rate limited. Every
//! 2xx response uses 2 tokens while a 304 not modified response only consumes 1 token as it incurs
//! less of a resource strain on ESI to not have to return the entire model for every request.

use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use chrono::Utc;
use eve_esi::{CacheStrategy, CachedResponse};
use serde::Deserialize;

/// Shared error enum that implements an internal server error response that can be returned
#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    EsiError(#[from] eve_esi::Error),
    #[error(transparent)]
    AxumError(#[from] axum::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
    }
}

#[derive(Deserialize)]
struct GetByIdParams {
    id: i64,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Enable logging
    // Run with `RUST_LOG=eve_esi=debug cargo run --example axum` to see logs
    env_logger::init();

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

    // Share the ESI client across threads with .layer(Extension)
    // Not doing this will result in JWT key caching for token validation not working
    // & requests taking longer.
    let app = Router::new()
        .route("/character", get(get_esi_character))
        .layer(Extension(esi_client));

    // Start the API server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("Test character API at http://localhost:8080/character?id=2114794365");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_esi_character(
    Extension(esi_client): Extension<eve_esi::Client>,
    params: Query<GetByIdParams>,
) -> Response {
    // Get character id from request URL
    let character_id: i64 = params.0.id;

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
        Err(error) => return Error::from(error).into_response(),
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
        Err(error) => return Error::from(error).into_response(),
    };

    // Determine if we have updated information since last cache request
    let character = match cache_result {
        // We return the updated character from ESI (updating our database entry in a real application)
        //
        // Note: this case would rarely ever occur in this example, usually we wouldn't fetch immediately again,
        // instead waiting until after the 30 day character info cache has expired.
        CachedResponse::Fresh(updated_character) => updated_character,
        // We return the initially fetched character as no information has since changed
        CachedResponse::NotModified => initial_character,
    };

    (StatusCode::OK, Json(character.data)).into_response()
}
