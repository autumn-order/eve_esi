use oauth2::TokenResponse;

use crate::oauth2::util::jwk_response::get_jwk_success_response;
use crate::oauth2::util::jwt::create_mock_token;
use crate::util::integration_test_setup;

/// No validation will be made due to ESI client config diabling it
#[tokio::test]
async fn test_validate_token_before_request_disabled() {
    let (_, mut mock_server) = integration_test_setup().await;

    let config = eve_esi::Config::builder()
        .esi_validate_token_before_request(false)
        .build()
        .expect("Failed to build ESI Config");

    let esi_client = eve_esi::Client::builder()
        .client_id("client_id")
        .client_secret("client_secret")
        .callback_url("http://localhost:8080/callback")
        .config(config)
        .build()
        .expect("Failed to build ESI Client ");

    let expected_requests = 0;
    let mock_jwt_key_endpoint = get_jwk_success_response(&mut mock_server, expected_requests);

    let mock_token = create_mock_token(false);
    let access_token = mock_token.access_token().secret().to_string();

    let character_id = 123456789;
    let _ = esi_client
        .character()
        .get_agents_research(character_id, &access_token)
        .await;

    // Assert no requests were made due to validation being disabled
    mock_jwt_key_endpoint.assert();
}
