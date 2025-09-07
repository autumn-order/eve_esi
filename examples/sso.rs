use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Json, Redirect, Response},
    routing::get,
    Router,
};
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
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
    let esi_client: eve_esi::EsiClient = eve_esi::EsiClient::builder()
        .user_agent(&user_agent)
        .client_id(&esi_client_id)
        .client_secret(&esi_secret_secret)
        .callback_url(&callback_url)
        .build()
        .expect("Failed to build EsiClient");

    // Arc is used to share the client between threads safely
    // Sharing the esi_client as an Extension avoids having initialize it in every API route
    // This allows you to configure it once here in main as opposed to configuring again in every API route
    let shared_client = Arc::new(esi_client);
    let app = Router::new()
        .route("/login", get(login))
        .layer(Extension(shared_client));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Login at http://localhost:8080/login");
    axum::serve(listener, app).await.unwrap();
}

async fn login(Extension(esi_client): Extension<Arc<eve_esi::EsiClient>>) -> Response {
    let scopes = eve_esi::oauth2::ScopeBuilder::new().public_data().build();

    let auth_data = match esi_client.oauth2().login_url(scopes) {
        Ok(auth_data) => auth_data,
        Err(err) => {
            println!("Error initiating OAuth login: {}", err);

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal Server Error" })),
            )
                .into_response();
        }
    };

    Redirect::temporary(&auth_data.login_url).into_response()
}
