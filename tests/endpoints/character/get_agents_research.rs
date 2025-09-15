use eve_esi::{model::oauth2::EveJwtClaims, oauth2::scope::CharacterScopes, ScopeBuilder};
use oauth2::TokenResponse;

use crate::{
    oauth2::util::{jwk_response::get_jwk_success_response, jwt::create_mock_token_with_claims},
    util::setup,
};

/// Successful retrieval of character research agents
#[tokio::test]
async fn test_get_agents_research_success() {
    let (esi_client, mut mock_server) = setup().await;

    let mock_research_agents = serde_json::json!([{
        "agent_id": 100,
        "points_per_day": 1.07832178,
        "remainder_points": 1.07832178,
        "skill_type_id": 100,
        "started_at": "2018-12-20T16:11:54Z",
    }]);

    let mut mock_access_token_claims = EveJwtClaims::mock();
    mock_access_token_claims.scp = ScopeBuilder::new()
        .character(CharacterScopes::new().read_agents_research())
        .build();
    let token = create_mock_token_with_claims(false, mock_access_token_claims);

    let access_token = token.access_token().secret().to_string();

    // Create JWT key endpoint for token validation before request
    let mock_jwt_key_endpoint = get_jwk_success_response(&mut mock_server, 1);

    let mock_research_agents_endpoint = mock_server
        .mock("GET", "/characters/2114794365/agents_research")
        .with_status(200)
        .with_header("content-type", "application/json")
        // Expect access token for authenticated route
        .with_header("Authorization", &format!("Bearer {}", access_token))
        .with_body(mock_research_agents.to_string())
        .create();

    let character_id = 2114794365;
    let result = esi_client
        .character()
        .get_agents_research(character_id, &access_token)
        .await;

    // Assert JWT keys were fetched for token validation prior to request
    mock_jwt_key_endpoint.assert();

    // Assert 1 request & expected access token was received for mock endpoint
    mock_research_agents_endpoint.assert();

    assert!(result.is_ok());
}

/// Failed retrieval of character affiliations due to an internal server error
#[tokio::test]
async fn test_get_agents_research_500_internal_error() {
    let (esi_client, mut mock_server) = setup().await;

    let mut mock_access_token_claims = EveJwtClaims::mock();
    mock_access_token_claims.scp = ScopeBuilder::new()
        .character(CharacterScopes::new().read_agents_research())
        .build();
    let token = create_mock_token_with_claims(false, mock_access_token_claims);

    let access_token = token.access_token().secret().to_string();

    // Create JWT key endpoint for token validation before request
    let mock_jwt_key_endpoint = get_jwk_success_response(&mut mock_server, 1);

    let mock_research_agents_endpoint = mock_server
        .mock("GET", "/characters/2114794365/agents_research")
        .with_status(500)
        .with_header("content-type", "application/json")
        // Expect access token for authenticated route
        .with_header("Authorization", &format!("Bearer {}", access_token))
        .with_body(r#"{"error": "Internal server error"}"#)
        .create();

    let character_id = 2114794365;
    let result = esi_client
        .character()
        .get_agents_research(character_id, &access_token)
        .await;

    // Assert JWT keys were fetched for token validation prior to request
    mock_jwt_key_endpoint.assert();

    // Assert 1 request & expected access token was received for mock endpoint
    mock_research_agents_endpoint.assert();

    assert!(result.is_err());

    assert!(
        matches!(result, Err(eve_esi::Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR))
    );
}
