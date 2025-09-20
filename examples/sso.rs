//! EVE ESI Axum SSO Example
//!
//! This is an example demonstrating single sign-on with EVE Online's OAuth2 API.
//! This example demonstrates:
//!
//! 1. Configuring an ESI Client for OAuth2 & using it with the Axum web framework
//! 2. A login API route to redirect users to begin EVE Online's single sign-on
//! 3. Creating a callback API route which validates the state string to prevent CSRF and then
//!    fetches an access token before validating it and returning a response with the user's
//!    name & character ID.
//!
//! Additionally, this example demonstrates the usage of a session to store the state string
//! for the user between API routes.

use std::env;

use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::{IntoResponse, Json, Redirect, Response},
    routing::get,
    Router,
};
use oauth2::TokenResponse;
use serde::{Deserialize, Serialize};
use time::Duration;
use tower_sessions::{cookie::SameSite, Expiry, MemoryStore, Session, SessionManagerLayer};

const STATE_KEY: &str = "state";

#[derive(Deserialize)]
struct CallbackParams {
    state: String,
    code: String,
}

#[derive(Serialize)]
struct Character {
    character_id: i64,
    character_name: String,
}

#[derive(Default, Deserialize, Serialize, Debug)]
struct State(String);

#[tokio::main]
async fn main() {
    // Enable logging
    // Run with `RUST_LOG=eve_esi=debug cargo run --example sso` to see logs
    env_logger::init();

    // Retrieve environment from the .env
    dotenvy::dotenv().ok();

    let contact_email = env::var("CONTACT_EMAIL").expect("Please set CONTACT_EMAIL in your .env");
    let esi_client_id =
        env::var("EVE_ESI_CLIENT_ID").expect("Please set EVE_ESI_CLIENT_ID in your .env");
    let esi_secret_secret =
        env::var("EVE_ESI_CLIENT_SECRET").expect("Please set EVE_ESI_CLIENT_SECRET in your .env");
    let callback_url =
        env::var("EVE_ESI_CALLBACK_URL").expect("Please set EVE_ESI_CALLBACK_URL in your .env");

    // Always set a user agent for your ESI client
    // For production apps, ensure it contains a contact email in case anything goes wrong with your ESI requests
    // E.G. "MyApp/1.0 (contact@example.com; +https://github.com/your/repository)"
    let user_agent: String = format!(
        "{}/{} ({}; +{})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        contact_email,
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
        // client_id, client_secret, and callback_url must be set to enable OAuth2 for ESI client
        .client_id(&esi_client_id)
        .client_secret(&esi_secret_secret)
        .callback_url(&callback_url)
        .build()
        .expect("Failed to build Client");

    // Create a session layer, we use this to store the state code between the login & callback URLs
    // to validate in the callback to prevent CSRF.
    // In production, you'd typically use a Valkey/Redis instance instead of a MemoryStore.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        // You would set this to true for a production application
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(120)));

    // Share the ESI client across threads with .layer(Extension)
    // Not doing this will result in JWT key caching for token validation not working
    // & requests taking longer.
    let app = Router::new()
        .route("/login", get(login))
        .route("/callback", get(callback))
        .layer(Extension(esi_client))
        // Share reqwest_client across threads as well if your app needs it to share HTTP pool
        .layer(Extension(reqwest_client))
        .layer(session_layer);

    // Start the API server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Login at http://localhost:8080/login");
    axum::serve(listener, app).await.unwrap();
}

async fn login(session: Session, Extension(esi_client): Extension<eve_esi::Client>) -> Response {
    // Build the scopes we wish to request from the user
    let scopes = eve_esi::ScopeBuilder::new().public_data().build();

    // Generate the login url or return an error if one occurs
    let login_url = match esi_client.oauth2().login_url(scopes) {
        Ok(login_url) => login_url,
        // If OAuth2 is not properly configured such as .env not being set then an error will be returned
        Err(err) => {
            println!("Error initiating OAuth login: {}", err);

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal Server Error" })),
            )
                .into_response();
        }
    };

    // Store the state we'll validate in callback to prevent CSRF
    session
        .insert(STATE_KEY, State(login_url.state))
        .await
        .unwrap();

    // Redirect the user to the login url to begin the single sign-on flow
    Redirect::temporary(&login_url.login_url).into_response()
}

async fn callback(
    session: Session,
    Extension(esi_client): Extension<eve_esi::Client>,
    params: Query<CallbackParams>,
) -> Response {
    // Get the state from the session store
    let state: State = match session.get(STATE_KEY).await {
        // Ensure the state key in session has an actual value
        Ok(state) => match state {
            Some(state) => state,
            None => {
                println!("Found state in session store but has no value");

                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": "Internal Server Error" })),
                )
                    .into_response();
            }
        },
        // State was not found in session store, return an error
        Err(err) => {
            println!("Error initiating OAuth login: {}", err);

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal Server Error" })),
            )
                .into_response();
        }
    };

    // Validate the state to ensure it matches as expected
    if state.0 != params.0.state {
        return (
            StatusCode::BAD_REQUEST,
            "There was an issue logging you in, please try again.",
        )
            .into_response();
    }

    // Retrieve the access token
    let token = match esi_client.oauth2().get_token(&params.0.code).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error retrieving token: {}", err);

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal Server Error" })),
            )
                .into_response();
        }
    };

    // Validate the token
    let claims = match esi_client
        .oauth2()
        .validate_token(token.access_token().secret().to_string())
        .await
    {
        Ok(claims) => claims,
        Err(err) => {
            println!("Error validating token: {}", err);

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal Server Error" })),
            )
                .into_response();
        }
    };

    // Use utility function to parse `sub` field of claims to a character ID
    // The `sub` field is a string: "CHARACTER:EVE:123456789"
    // The `character_id()` function turns it into an i64: 123456789
    match claims.character_id() {
        Ok(character_id) => {
            let character_name = claims.name;
            let character = Character {
                character_id,
                character_name,
            };

            (StatusCode::OK, Json(character)).into_response()
        }
        Err(err) => {
            // Error if the sub field can't be parsed to a character ID
            // This shouldn't occur unless EVE changes their sub field format
            println!(
                "Error parsing JWT claims `sub` field to character id: {}",
                err
            );

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal Server Error" })),
            )
                .into_response();
        }
    }
}
