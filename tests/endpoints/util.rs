use eve_esi::model::oauth2::EveJwtClaims;
use mockito::{Mock, ServerGuard};
use oauth2::TokenResponse;

use crate::{
    oauth2::util::{jwk_response::get_jwk_success_response, jwt::create_mock_token_with_claims},
    util::setup,
};

/// Utility to setup JWT key endpoint for validation to test authenticated ESI routes
pub(super) async fn authenticated_endpoint_test_setup() -> (eve_esi::Client, ServerGuard, Mock) {
    let (esi_client, mut mock_server) = setup().await;

    // Create JWT key endpoint for token validation before request
    let mock_jwt_key_endpoint = get_jwk_success_response(&mut mock_server, 1);

    (esi_client, mock_server, mock_jwt_key_endpoint)
}

/// Utility to create an access token for authenticated ESI routes
pub(super) fn mock_access_token_with_scopes(scopes: Vec<String>) -> String {
    let mut mock_access_token_claims = EveJwtClaims::mock();
    mock_access_token_claims.scp = scopes;

    let token = create_mock_token_with_claims(false, mock_access_token_claims);

    token.access_token().secret().to_string()
}
