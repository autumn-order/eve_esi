//! EVE ESI Axum SSO Example
//!
//! This is an example demonstrating single sign-on with EVE Online's OAuth2 API.
//!
//! This example is incomplete as it demonstrates the first half of the login but still has yet to implement
//! the second half of handling the callback and validating the token.

use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Json, Redirect, Response},
    routing::get,
    Router,
};
use std::env;

#[tokio::main]
async fn main() {
    // Retrieve environment from the .env
    dotenv::dotenv().ok();

    let contact_email = env::var("CONTACT_EMAIL").expect("Please set CONTACT_EMAIL in your .env");
    let esi_client_id =
        env::var("EVE_ESI_CLIENT_ID").expect("Please set EVE_ESI_CLIENT_ID in your .env");
    let esi_secret_secret =
        env::var("EVE_ESI_CLIENT_SECRET").expect("Please set EVE_ESI_CLIENT_SECRET in your .env");
    let callback_url =
        env::var("EVE_ESI_CALLBACK_URL").expect("Please set EVE_ESI_CALLBACK_URL in your .env");

    // Always set a user agent for your ESI client
    // For production apps, ensure it contains a contact email in case anything goes wrong with your ESI requests
    // E.G. "MyApp/1.0 (contact@example.com)"
    let user_agent: String = format!(
        "{}/{} ({}) ({})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        contact_email,
        env!("CARGO_PKG_REPOSITORY")
    );

    // Build an ESI client with a user agent & optional reqwest client
    let esi_client: eve_esi::Client = eve_esi::Client::builder()
        // Always set a user agent to identify your application
        .user_agent(&user_agent)
        // client_id, client_secret, and callback_url must be set to enable OAuth2 for ESI client
        .client_id(&esi_client_id)
        .client_secret(&esi_secret_secret)
        .callback_url(&callback_url)
        .build()
        .expect("Failed to build Client");

    // Access the esi_client from an Axum extension to share it across threads
    let app = Router::new()
        .route("/login", get(login))
        .layer(Extension(esi_client));

    // Start the API server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Login at http://localhost:8080/login");
    axum::serve(listener, app).await.unwrap();
}

async fn login(Extension(esi_client): Extension<eve_esi::Client>) -> Response {
    // Build the scopes we wish to request from the user
    let scopes = eve_esi::oauth2::ScopeBuilder::new().public_data().build();

    // Generate the login url or return an error if one occurs
    let auth_data = match esi_client.oauth2().login_url(scopes) {
        Ok(auth_data) => auth_data,
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

    // Redirect the user to the login url to begin the single sign-on flow
    Redirect::temporary(&auth_data.login_url).into_response()
}
