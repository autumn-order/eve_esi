//! Configuration error types for the EVE ESI client.
//!
//! This module contains error types that occur during client or config construction,
//! typically related to invalid OAuth2 settings or malformed URLs.

use thiserror::Error;

/// Errors when building a new [`Client`](crate::Client) or [`Config`](crate::Config)
///
/// This enum represents the various errors which could occur due to an improper configuration such as an
/// improper URL format or an invalid JWT key background refresh threshold.
///
/// See the [module-level documentation](super) for an overview and usage example.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// The [crate::Client] is missing a `client_id`
    ///
    /// This error occurs when attempting to access EVE Online's OAuth2
    /// without first setting the client ID on the [`Client`](crate::Client).
    ///
    /// # Resolution
    /// To fix this:
    /// - Set `esi_client_builder.client_id(client_id)`
    /// - You can obtain a client ID at: <https://developers.eveonline.com/applications>
    #[error(
        "Missing ESI client ID.\n\
        \n\
        To fix this:\n\
          - Set `esi_client_builder.client_id(client_id)`\n\
          - You can obtain a client ID at:\n\
              https://developers.eveonline.com/applications\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    MissingClientId,

    /// The [crate::Client] is missing a `client_secret`
    ///
    /// This error occurs when attempting to access EVE Online's OAuth2
    /// without first setting the client secret on the [`Client`](crate::Client).
    ///
    /// # Resolution
    /// To fix this:
    /// - Set `esi_client_builder.client_secret(client_secret)`
    /// - You can obtain a client secret at: <https://developers.eveonline.com/applications>
    #[error(
        "Missing ESI client secret.\n\
        \n\
        To fix this:\n\
          - Set `esi_client_builder.client_secret(client_secret)`\n\
          - You can obtain a client secret at:\n\
              https://developers.eveonline.com/applications\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    MissingClientSecret,

    /// The [crate::Client] is missing a `callback_url`
    ///
    /// This error occurs when attempting to access EVE Online's OAuth2
    /// without first setting the callback URL on the [`Client`](crate::Client).
    ///
    /// # Resolution
    /// To fix this:
    /// - Set `esi_client_builder.callback_url(callback_url)`
    /// - Ensure it matches the callback URL set at: <https://developers.eveonline.com/applications>
    #[error(
        "Missing ESI callback URL.\n\
        \n\
        To fix this:\n\
          - Set `esi_client_builder.callback_url(callback_url)`\n\
          - Ensure it matches the callback URL set at:\n\
              https://developers.eveonline.com/applications\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    MissingCallbackUrl,

    /// The `callback_url` is in an invalid URL format.
    ///
    /// This error occurs when the callback url set using `esi_client_builder.callback_url(callback_url)`
    /// is not correctly formatted.
    ///
    /// # Resolution
    /// To fix this:
    /// - Validate the url set using `esi_client_builder.callback_url(callback_url)`
    ///   is using a url that is correctly formatted
    ///
    ///   e.g. `https://example.com/callback`
    #[error(
        "Invalid EVE OAuth2 callback URL:\n\
        \n\
        To fix this:\n\
          - Use the default URL provided by the Client\n\
          - Validate the url set using `esi_client_builder.callback_url(callback_url)`\n\
            is using a url that is correctly formatted\n\
            e.g. https://example.com/callback\n\
        \n\
        This is required for accessing EVE Online OAuth2."
    )]
    InvalidCallbackUrl,

    /// EVE OAuth2 authentication URL is invalid.
    ///
    /// This error occurs when the auth url is changed from the default URL
    /// on [`Config`](crate::Config) and is not correctly formatted.
    ///
    /// # Resolution
    /// To fix this:
    /// - Use the default URL provided by the default config
    /// - Validate the url set on [`Config`](crate::Config)
    ///   is using a url that is correctly formatted.
    ///
    ///   e.g. `https://login.eveonline.com/v2/oauth/authorize`
    #[error(
        "Invalid EVE OAuth2 URL:\n\
        \n\
        To fix this:\n\
          - Use the default URL provided by the default config\n\
          - Validate the url set using [`super::OAuth2Config`]\n\
            is using a url that is correctly formatted\n\
            e.g. https://login.eveonline.com/v2/oauth/authorize"
    )]
    InvalidAuthUrl,

    /// EVE OAuth2 token URL is invalid.
    ///
    /// This error occurs when the token url is changed from the default URL
    /// on [`Config`](crate::Config) and is not correctly formatted.
    ///
    /// # Resolution
    /// To fix this:
    /// - Use the default URL provided by the default config
    /// - Validate the url set on [`Config`](crate::Config)
    ///   is using a url that is correctly formatted.
    ///
    ///   e.g. `https://login.eveonline.com/v2/oauth/token`
    #[error(
        "Invalid EVE OAuth2 token URL:\n\
        \n\
        To fix this:\n\
          - Use the default URL provided by the default config\n\
          - Validate the url set using [`super::OAuth2Config`]\n\
            is using a url that is correctly formatted\n\
            e.g. https://login.eveonline.com/v2/oauth/token"
    )]
    InvalidTokenUrl,

    /// JWT key cache background refresh threshold percentage is not between 0 and 100
    ///
    /// This error occurs when the background refresh threshold percentage used to
    /// determine when to proactively refresh the JWT key cache is configured to a value
    /// that is not between 0 and 100.
    ///
    /// Extreme values would either cause the refresh to trigger in quick succession if set
    /// too low or to never trigger if set too high.
    ///
    /// # Resolution
    /// To fix this:
    /// - Use the default percentage provided by the default config
    /// - Validate the percentage set on [`Config`](crate::Config)
    ///   is between 0 and 100.
    #[error(
        "Invalid JWT key cache background refresh threshold:\n\
        \n\
        To fix this:\n\
          - Use the default percentage provided by the default config\n\
          - Validate the percentage set using [`super::OAuth2Config`]\n\
            is between 0 and 100."
    )]
    InvalidBackgroundRefreshThreshold,
}
