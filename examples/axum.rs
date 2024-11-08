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
    let reqwest_client: reqwest::Client = reqwest::Client::new();
    let esi_client: eve_esi::Client = eve_esi::Client::new(reqwest_client);

    let character_id: i32 = params.0.id;

    match esi_client.get_character(character_id).await {
        Ok(character) => (StatusCode::OK, Json(character)).into_response(),
        Err(error) => {
            let status_code: StatusCode =
                StatusCode::from_u16(error.status().unwrap().into()).unwrap();

            (status_code, Json(error.to_string())).into_response()
        }
    }
}

async fn get_esi_corporation(params: Query<GetByIdParams>) -> Response {
    let reqwest_client: reqwest::Client = reqwest::Client::new();
    let esi_client: eve_esi::Client = eve_esi::Client::new(reqwest_client);

    let corporation_id: i32 = params.0.id;

    match esi_client.get_corporation(corporation_id).await {
        Ok(corporation) => (StatusCode::OK, Json(corporation)).into_response(),
        Err(error) => {
            let status_code = StatusCode::from_u16(error.status().unwrap().into()).unwrap();

            (status_code, Json(error.to_string())).into_response()
        }
    }
}
