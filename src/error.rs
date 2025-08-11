use thiserror::Error;

#[derive(Error, Debug)]
pub enum EsiError {
    #[error(transparent)]
    OAuthError(OAuthError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

/// Errors that can occur during EVE Online's OAuth2 authentication process.
///
/// This enum represents various error conditions that may arise when attempting to
/// authenticate with EVE Online's OAuth2 process. It provides detailed error messages
/// with instructions on how to resolve each issue.
///
/// # Variants
/// - `MissingClientId` - The required client ID for EVE Online OAuth2 was not provided.
/// - `MissingClientSecret` - The required client secret for EVE Online OAuth2 was not provided.
/// - `MissingCallbackUrl` - The required callback URL for EVE Online OAuth2 was not provided.
/// - `InvalidAuthUrl` - The provided EVE OAuth2 authorization URL is in an invalid format.
/// - `InvalidTokenUrl` - The provided EVE OAuth2 token URL is in an invalid format.
/// - `InvalidCallbackUrl` - The provided EVE OAuth2 callback URL is in an invalid format.
///
/// # Usage
/// These errors are typically returned when:
/// - Setting up the OAuth2 authentication process with `initiate_oauth_login`
/// - Attempting to exchange an authorization code for an access token
/// - Refreshing OAuth2 tokens
///
/// # Example
/// ```
/// let esi_client = eve_esi::EsiClient::new("MyApp/1.0");
///
/// // Missing client ID will cause an error
/// let scopes = eve_esi::oauth2::ScopeBuilder::new()
///     .public_data()
///     .build();
/// let result = esi_client.initiate_oauth_login(scopes);
///
/// assert!(matches!(result, Err(eve_esi::error::OAuthError::MissingClientId)));
/// ```
#[derive(Error, Debug)]
pub enum OAuthError {
    /// Error returned when the ESI client ID is missing.
    ///
    /// This error occurs when attempting to access EVE Online's OAuth2
    /// without first setting the client ID on the EsiClient.
    ///
    /// # Resolution
    /// To fix this:
    /// - Set `esi_client.set_client_id(client_id)`
    /// - You can obtain a client ID at:
    ///   https://developers.eveonline.com/applications
    #[error(
        "Missing ESI client ID.\n\
        \n\
        To fix this:\n\
          - Set `esi_client.set_client_id(client_id)`\n\
          - You can obtain a client ID at:\n\
              https://developers.eveonline.com/applications\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    MissingClientId,

    /// Error returned when the ESI client secret is missing.
    ///
    /// This error occurs when attempting to access EVE Online's OAuth2
    /// without first setting the client secret on the EsiClient.
    ///
    /// # Resolution
    /// To fix this:
    /// - Set `esi_client.set_client_secret(client_secret)`
    /// - You can obtain a client secret at:
    ///   https://developers.eveonline.com/applications
    #[error(
        "Missing ESI client secret.\n\
        \n\
        To fix this:\n\
          - Set `esi_client.set_client_secret(client_secret)`\n\
          - You can obtain a client secret at:\n\
              https://developers.eveonline.com/applications\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    MissingClientSecret,

    /// Error returned when the ESI callback URL is missing.
    ///
    /// This error occurs when attempting to access EVE Online's OAuth2
    /// without first setting the callback URL on the EsiClient.
    ///
    /// # Resolution
    /// To fix this:
    /// - Set `esi_client.set_callback_url(callback_url)`
    /// - Ensure it matches the callback URL set at:
    ///   https://developers.eveonline.com/applications
    #[error(
        "Missing ESI callback URL.\n\
        \n\
        To fix this:\n\
          - Set `esi_client.set_callback_url(callback_url)`\n\
          - Ensure it matches the callback URL set at:\n\
              https://developers.eveonline.com/applications\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    MissingCallbackUrl,

    /// Error returned when the EVE OAuth2 URL is invalid.
    ///
    /// This error occurs when the `esi_client.eve_auth_url` variable is changed
    /// from the default URL provided by the EsiClient and the url is not correctly formatted.
    ///
    /// # Resolution
    /// To fix this:
    /// - Use the default URL provided by the EsiClient
    /// - Validate the url set using `esi_client.eve_auth_url = auth_url`
    ///   is using a url that is correctly formatted
    ///
    ///   e.g. https://login.eveonline.com/v2/oauth/authorize
    #[error(
        "Invalid EVE OAuth2 URL:\n\
        \n\
        To fix this:\n\
          - Use the default URL provided by the EsiClient\n\
          - Validate the url set using `esi_client.eve_auth_url = auth_url`\n\
            is using a url that is correctly formatted\n\
            e.g. https://login.eveonline.com/v2/oauth/authorize\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    InvalidAuthUrl,

    /// Error returned when the EVE OAuth2 token URL is invalid.
    ///
    /// This error occurs when the `esi_client.eve_token_url` variable is changed
    /// from the default URL provided by the EsiClient and the url is not correctly formatted.
    ///
    /// # Resolution
    /// To fix this:
    /// - Use the default URL provided by the EsiClient
    /// - Validate the url set using `esi_client.eve_token_url = token_url`
    ///   is using a url that is correctly formatted
    ///
    ///   e.g. https://login.eveonline.com/v2/oauth/token
    #[error(
        "Invalid EVE OAuth2 token URL:\n\
        \n\
        To fix this:\n\
          - Use the default URL provided by the EsiClient\n\
          - Validate the url set using `esi_client.eve_token_url = token_url`\n\
            is using a url that is correctly formatted\n\
            e.g. https://login.eveonline.com/v2/oauth/token\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    InvalidTokenUrl,

    /// Error returned when the EVE OAuth2 callback URL is invalid.
    ///
    /// This error occurs when the callback url set using `esi_client.set_callback_url(callback_url)`
    /// is not correctly formatted.
    ///
    /// # Resolution
    /// To fix this:
    /// - Validate the url set using `esi_client.set_callback_url(callback_url)`
    ///   is using a url that is correctly formatted
    ///
    ///   e.g. https://example.com/callback
    #[error(
        "Invalid EVE OAuth2 callback URL:\n\
        \n\
        To fix this:\n\
          - Use the default URL provided by the EsiClient\n\
          - Validate the url set using `esi_client.set_callback_url(callback_url)`\n\
            is using a url that is correctly formatted\n\
            e.g. https://example.com/callback\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    InvalidCallbackUrl,
}
