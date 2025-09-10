//! EVE ESI Axum Example
//!
//! This is an example demonstrating how to build an ESI client and make requests to public ESI routes with
//! the Axum web framework.
//!
//! This demonstrates best practices such as sharing the ESI client across threads, setting a user agent
//! to identify the application, and sharing an HTTP pool with a reqwest client if the application is
//! using reqwest beyond just ESI calls.

use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct GetByIdParams {
    id: i64,
}

#[tokio::main]
async fn main() {
    // Enable logging
    // Run with `RUST_LOG=eve_esi=debug cargo run --example axum` to see logs
    env_logger::init();

    // Always set a user agent for your ESI client
    // For production apps, ensure it contains a contact email in case anything goes wrong with your ESI requests
    // E.G. "MyApp/1.0 (contact@example.com)"
    let user_agent: String = format!(
        "{}/{} ({})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_REPOSITORY")
    );

    // Optional: Build a reqwest client, share it with ESI client to share an HTTP request pool for performance
    // Only do this if your app uses reqwest client elsewhere beyond ESI requests
    let reqwest_client = reqwest::Client::builder()
        .user_agent(&user_agent)
        .build()
        .expect("Failed to build reqwest client");

    // Build an ESI client with a user agent & optional reqwest client
    let esi_client: eve_esi::Client = eve_esi::Client::builder()
        // Always set a user agent to identify your application
        .user_agent(&user_agent)
        .reqwest_client(reqwest_client.clone())
        .build()
        .expect("Failed to build ESI client");

    // Share the ESI client across threads with .layer(Extension)
    // Not doing this will result in caching not working properly & requests taking longer
    let app = Router::new()
        .route("/character", get(get_esi_character))
        .route("/corporation", get(get_esi_corporation))
        .layer(Extension(esi_client))
        // Optional: Share reqwest client across threads if your application uses it
        // You'll access it the same way you do for the esi_client
        .layer(Extension(reqwest_client));

    // Start the API server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Test character API at http://localhost:8080/character?id=2114794365");
    println!("Test corporation API at http://localhost:8080/corporation?id=98785281");
    axum::serve(listener, app).await.unwrap();
}

async fn get_esi_character(
    Extension(esi_client): Extension<eve_esi::Client>,
    params: Query<GetByIdParams>,
) -> Response {
    // Get character id from request URL
    let character_id: i64 = params.0.id;

    // Request character public information from ESI
    match esi_client
        .character()
        .get_character_public_information(character_id)
        .await
    {
        // Return the character information
        Ok(character) => (StatusCode::OK, Json(character)).into_response(),
        // Return an error if fetching character information fails
        Err(error) => {
            let status_code: StatusCode = match &error {
                eve_esi::Error::ReqwestError(ref err) => {
                    StatusCode::from_u16(err.status().unwrap().into()).unwrap()
                }
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            (status_code, Json(error.to_string())).into_response()
        }
    }
}

async fn get_esi_corporation(
    Extension(esi_client): Extension<eve_esi::Client>,
    params: Query<GetByIdParams>,
) -> Response {
    // Get corporation id from request URL
    let corporation_id: i64 = params.0.id;

    // Request corporation information from ESI
    match esi_client
        .corporation()
        .get_corporation_information(corporation_id)
        .await
    {
        // Return the corporation information
        Ok(corporation) => (StatusCode::OK, Json(corporation)).into_response(),
        // Return an error if fetching corporation information fails
        Err(error) => {
            let status_code: StatusCode = match &error {
                eve_esi::Error::ReqwestError(ref err) => {
                    StatusCode::from_u16(err.status().unwrap().into()).unwrap()
                }
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            (status_code, Json(error.to_string())).into_response()
        }
    }
}
