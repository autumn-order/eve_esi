//! # EVE ESI Request Utilities
//!
//! Utilities particularly for making authenticated ESI requests. Provides methods to
//! validate tokens prior to making authenticated requests to catch possible errors before
//! making a request.
//!
//! See the [module-level documentation](super) for an overview, methods, & usage example.

use super::EsiApi;
use crate::{model::oauth2::EveJwtClaims, Error, OAuthError};

impl<'a> EsiApi<'a> {
    /// Utilty function which returns an error if token is invalid, expired, or is missing required scopes
    pub(super) async fn validate_token_before_request(
        &self,
        access_token: &str,
        required_scopes: Vec<String>,
    ) -> Result<(), Error> {
        if self.client.inner.esi_validate_token_before_request {
            log::trace!("Validating token prior to expiration & scope checks");

            let claims = self
                .client
                .oauth2()
                .validate_token(access_token.to_string())
                .await?;

            check_token_expiration(&claims)?;

            check_token_scopes(&claims, required_scopes)?;

            log::debug!("Access token passed validation, expiration, and scope checks successfully prior to authenticated ESI request.");
        };

        Ok(())
    }
}

/// Utility function for providing an error when token claims are expired
pub(super) fn check_token_expiration(access_token_claims: &EveJwtClaims) -> Result<(), Error> {
    if access_token_claims.is_expired() {
        let error = OAuthError::AccessTokenExpired();

        log::error!(
            "Failed to make request to authenticated ESI route due to token being expired: {:?}",
            error
        );

        return Err(Error::OAuthError(error));
    }

    log::trace!("Checked access token for expiration prior to authenticated ESI request, token is not expired.");

    Ok(())
}

/// Utility function for providing an error when token claims is missing required scopes
pub(super) fn check_token_scopes(
    access_token_claims: &EveJwtClaims,
    required_scopes: Vec<String>,
) -> Result<(), Error> {
    if !access_token_claims.has_scopes(&required_scopes) {
        let error = OAuthError::AccessTokenMissingScopes(required_scopes);

        log::error!("Failed to make request to authenticated ESI route due to missing required scopes: {:?}", error);

        return Err(Error::OAuthError(error));
    }

    log::trace!("Checked access token for required scopes prior to authenticated ESI request, all required scopes are present: {:?}", required_scopes);

    Ok(())
}

#[cfg(test)]
mod check_token_expiration_tests {
    use std::time::Duration;

    use chrono::Utc;

    use super::check_token_expiration;
    use crate::{tests::util::create_mock_jwt_claims, Error, OAuthError};

    /// No errors due to token not being expired
    #[test]
    fn test_check_token_expiration_success() {
        let mock_claims = create_mock_jwt_claims();

        let result = check_token_expiration(&mock_claims);

        assert!(result.is_ok())
    }

    /// Error occurs due to token being expired
    #[test]
    fn test_check_token_expiration_error() {
        let mut mock_claims = create_mock_jwt_claims();
        mock_claims.exp = Utc::now() - Duration::from_secs(60); // expired 1 minute ago
        mock_claims.iat = Utc::now() - Duration::from_secs(960); // created 16 minutes ago

        let result = check_token_expiration(&mock_claims);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(Error::OAuthError(OAuthError::AccessTokenExpired()))
        ))
    }
}

#[cfg(test)]
mod test_check_token_scopes {
    use super::check_token_scopes;
    use crate::{tests::util::create_mock_jwt_claims, Error, OAuthError};

    /// No errors due to token having all required scopes
    #[test]
    fn test_check_token_claims_success() {
        let required_scopes = vec!["publicData".to_string()];

        let mut mock_claims = create_mock_jwt_claims();
        mock_claims.scp = required_scopes.clone();

        let result = check_token_scopes(&mock_claims, required_scopes);

        assert!(result.is_ok())
    }

    /// Error occurs due to token missing required scopes
    #[tokio::test]
    async fn test_check_token_claims_scope_error() {
        let required_scopes = vec!["publicData".to_string()];

        let mut mock_claims = create_mock_jwt_claims();
        mock_claims.scp = Vec::new();

        let result = check_token_scopes(&mock_claims, required_scopes);

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(Error::OAuthError(OAuthError::AccessTokenMissingScopes(_)))
        ))
    }
}
