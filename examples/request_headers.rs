//! EVE ESI Request Headers Example
//!
//! This example demonstrates how to modify request headers when making an ESI request
//! for the purposes of changing the language of the response, setting the tenant server the
//! data comes from, or providing a custom header with the `with_header` method.

use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
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

    match request.send().await {
        // Return the character information
        Ok(character) => (StatusCode::OK, Json(character.data)).into_response(),
        // Return an error if fetching character information fails
        Err(error) => Error::from(error).into_response(),
    }
}
