use eve_esi::{
    model::{character::CharacterResearchAgent, oauth2::EveJwtClaims},
    oauth2::scope::CharacterScopes,
    ScopeBuilder,
};
use oauth2::TokenResponse;

use crate::{
    oauth2::util::{jwk_response::get_jwk_success_response, jwt::create_mock_token_with_claims},
    util::setup,
};

/// Successful retrieval of character research agents via authenticated ESI route
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Create mock character research agents
/// - Create a mock token for authenticated route
/// - Add mock JWT key endpoint as authenticated ESI endpoints validate with keys before request
/// - Configure mock server with authenticated ESI endpoint returning the research agents
///
/// # Assertions
/// - Assert JWT keys were fetched for token validation prior to request
/// - Assert 1 request & expected access token was sent to the mock server
/// - Assert result is Ok
/// - Assert received expected character research agents
#[tokio::test]
async fn test_get_agents_research_success() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Create mock character research agents
    let mock_research_agents = vec![CharacterResearchAgent {
        agent_id: 100,
        points_per_day: 1.07832178,
        remainder_points: 1.07832178,
        skill_type_id: 100,
        started_at: "2018-12-20T16:11:54Z".parse().unwrap(),
    }];

    // Create a mock token for authenticated route
    let mut mock_claims = EveJwtClaims::mock();
    mock_claims.scp = ScopeBuilder::new()
        .character(CharacterScopes::new().read_agents_research())
        .build();

    let token = create_mock_token_with_claims(false, mock_claims);
    let access_token = token.access_token().secret().to_string();

    // Add mock JWT key endpoint as authenticated ESI endpoints validate with keys before request
    let mock_jwk = get_jwk_success_response(&mut mock_server, 1);

    // Configure mock server with authenticated ESI endpoint returning the research agents
    let mock = mock_server
        .mock("GET", "/characters/2114794365/agents_research")
        .with_status(200)
        .with_header("content-type", "application/json")
        // Expect proper access token for authenticated route
        .with_header("Authorization", &format!("Bearer {}", access_token))
        .with_body(serde_json::to_string(&mock_research_agents).unwrap())
        .create();

    // Retrieve the character research agents using access token
    let result = esi_client
        .character()
        .get_agents_research(2114794365, &access_token)
        .await;

    // Assert JWT keys were fetched for token validation prior to request
    mock_jwk.assert();

    // Assert 1 request & expected access token was sent to the mock server
    mock.assert();

    // Assert result is Ok
    assert!(result.is_ok());

    // Assert received expected character research agents
    let research_agents = result.unwrap();
    assert_eq!(research_agents, mock_research_agents);
}

/// Error handling when server returns 500 internal error
///
/// # Test Setup
/// - Setup a basic EsiClient & mock HTTP server
/// - Create a mock token for authenticated route
/// - Add mock JWT key endpoint as authenticated ESI endpoints validate with keys before request
/// - Configure mock server with an authenticated ESI endpoint returning 500 internal server error
///
/// # Assertions
/// - Assert JWT keys were fetched for token validation prior to request
/// - Assert 1 request was made to the mock server
/// - Assert result is error
/// - Assert reqwest error is due to status INTERNAL_SERVER_ERROR
#[tokio::test]
async fn test_get_agents_research_500_internal_error() {
    // Setup a basic EsiClient & mock HTTP server
    let (esi_client, mut mock_server) = setup().await;

    // Add mock JWT key endpoint as authenticated ESI endpoints validate with keys before request
    let mock_jwk = get_jwk_success_response(&mut mock_server, 1);

    // Configure mock server with an authenticated ESI endpoint returning 500 internal server error
    let mock = mock_server
        .mock("GET", "/characters/2114794365/agents_research")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal server error"}"#)
        .create();

    // Create a mock token for authenticated route
    let mut mock_claims = EveJwtClaims::mock();
    mock_claims.scp = ScopeBuilder::new()
        .character(CharacterScopes::new().read_agents_research())
        .build();

    let token = create_mock_token_with_claims(false, mock_claims);
    let access_token = token.access_token().secret().to_string();

    // Attempt to retrieve research agents
    let result = esi_client
        .character()
        .get_agents_research(2114794365, &access_token)
        .await;

    // Assert JWT keys were fetched for token validation prior to request
    mock_jwk.assert();

    // Assert 1 request was made to the mock server
    mock.assert();

    // Assert result is error
    assert!(result.is_err());
    match result {
        Err(eve_esi::Error::ReqwestError(err)) => {
            // Assert reqwest error is due to status INTERNAL_SERVER_ERROR
            assert!(err.status().is_some());
            assert_eq!(
                err.status().unwrap(),
                reqwest::StatusCode::INTERNAL_SERVER_ERROR
            );
        }
        err => {
            panic!(
                "Expected ReqwestError, got different error type: {:#?}",
                err
            )
        }
    }
}
