use eve_esi::{model::oauth2::EveJwtClaims, oauth2::scope::CharacterScopes, ScopeBuilder};
use oauth2::TokenResponse;

use crate::{
    endpoints::util::{authenticated_endpoint_test_setup, mock_access_token_with_scopes},
    oauth2::util::{jwk_response::get_jwk_success_response, jwt::create_mock_token_with_claims},
    util::integration_test_setup,
};

/// Successful retrieval of character's blueprints
#[tokio::test]
async fn test_get_blueprints_success() {
    let (esi_client, mut mock_server, mock_jwt_key_endpoint) =
        authenticated_endpoint_test_setup().await;
    let access_token = mock_access_token_with_scopes(
        ScopeBuilder::new()
            .character(CharacterScopes::new().read_blueprints())
            .build(),
    );

    let mock_blueprints = serde_json::json!([{
        "item_id": 0,
        "location_flag": "Hangar",
        "location_id": 0,
        "material_efficiency": 0,
        "quantity": -1,
        "runs": -1,
        "time_efficiency": 0,
        "type_id": 0
    }]);

    let mock_blueprints_endpoint = mock_server
        .mock("GET", "/characters/2114794365/blueprints?page=0")
        .with_status(200)
        .with_header("content-type", "application/json")
        // Expect access token for authenticated route
        .with_header("Authorization", &format!("Bearer {}", access_token))
        .with_body(mock_blueprints.to_string())
        .create();

    let character_id = 2114794365;
    let page = 0;
    let result = esi_client
        .character()
        .get_blueprints(&access_token, character_id, page)
        .await;

    // Assert JWT keys were fetched for token validation prior to request
    mock_jwt_key_endpoint.assert();

    // Assert 1 request & expected access token was received for mock endpoint
    mock_blueprints_endpoint.assert();

    assert!(result.is_ok());
}

/// Failed retrieval of character's blueprints due to an internal server error
#[tokio::test]
async fn test_get_blueprints_internal_error() {
    let (esi_client, mut mock_server) = integration_test_setup().await;

    let mut mock_access_token_claims = EveJwtClaims::mock();
    mock_access_token_claims.scp = ScopeBuilder::new()
        .character(CharacterScopes::new().read_blueprints())
        .build();
    let token = create_mock_token_with_claims(false, mock_access_token_claims);

    let access_token = token.access_token().secret().to_string();

    // Create JWT key endpoint for token validation before request
    let mock_jwt_key_endpoint = get_jwk_success_response(&mut mock_server, 1);

    let mock_blueprints_endpoint = mock_server
        .mock("GET", "/characters/2114794365/blueprints?page=0")
        .with_status(500)
        .with_header("content-type", "application/json")
        // Expect access token for authenticated route
        .with_header("Authorization", &format!("Bearer {}", access_token))
        .with_body(r#"{"error": "Internal server error"}"#)
        .create();

    let character_id = 2114794365;
    let page = 0;
    let result = esi_client
        .character()
        .get_blueprints(&access_token, character_id, page)
        .await;

    // Assert JWT keys were fetched for token validation prior to request
    mock_jwt_key_endpoint.assert();

    // Assert 1 request & expected access token was received for mock endpoint
    mock_blueprints_endpoint.assert();

    assert!(result.is_err());

    assert!(
        matches!(result, Err(eve_esi::Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR))
    );
}
