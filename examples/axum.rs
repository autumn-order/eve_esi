use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct GetByIdParams {
    id: i32,
}

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/character", get(get_esi_character))
        .route("/corporation", get(get_esi_corporation));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("Test character API at http://localhost:8000/character?id=2114794365");
    println!("Test corporation API at http://localhost:8000/corporation?id=98785281");
    axum::serve(listener, app).await.unwrap();
}

async fn get_esi_character(params: Query<GetByIdParams>) -> Response {
    let mut esi_client: eve_esi::EsiClient = eve_esi::EsiClient::new(USER_AGENT);

    esi_client.esi_url = "https://esi.evetech.net/latest".to_string();

    let character_id: i32 = params.0.id;

    match esi_client
        .characters()
        .get_character_public_information(character_id)
        .await
    {
        Ok(character) => (StatusCode::OK, Json(character)).into_response(),
        Err(error) => {
            let status_code: StatusCode = match &error {
                eve_esi::error::EsiError::ReqwestError(ref err) => {
                    StatusCode::from_u16(err.status().unwrap().into()).unwrap()
                }
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            (status_code, Json(error.to_string())).into_response()
        }
    }
}

async fn get_esi_corporation(params: Query<GetByIdParams>) -> Response {
    let esi_client: eve_esi::EsiClient = eve_esi::EsiClient::new(USER_AGENT);

    let corporation_id: i32 = params.0.id;

    match esi_client
        .corporations()
        .get_corporation_information(corporation_id)
        .await
    {
        Ok(corporation) => (StatusCode::OK, Json(corporation)).into_response(),
        Err(error) => {
            let status_code: StatusCode = match &error {
                eve_esi::error::EsiError::ReqwestError(ref err) => {
                    StatusCode::from_u16(err.status().unwrap().into()).unwrap()
                }
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            (status_code, Json(error.to_string())).into_response()
        }
    }
}
