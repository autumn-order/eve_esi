use oauth2::basic::BasicTokenType;
use oauth2::{AccessToken, EmptyExtraTokenFields, RefreshToken, StandardTokenResponse};
use std::time::Duration;

/// Creates a mock token for usage with token related integration tests
pub fn create_mock_token() -> StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType> {
    // Create the token components
    let access_token = AccessToken::new("mock_access_token_value".to_string());
    let token_type = BasicTokenType::Bearer;
    let expires_in = Some(&Duration::from_secs(3600)); // 1 hour
    let refresh_token = Some(RefreshToken::new("mock_refresh_token_value".to_string()));

    // Create empty extra fields
    let extra_fields = EmptyExtraTokenFields {};

    // Create the token response
    let mut token = StandardTokenResponse::new(access_token, token_type, extra_fields);

    // Set optional fields
    token.set_expires_in(expires_in);
    token.set_refresh_token(refresh_token);

    token
}
