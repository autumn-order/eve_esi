use thiserror::Error;

#[derive(Error, Debug)]
pub enum EsiError {
    #[error(
        "Missing ESI client ID.\n\
        \n\
        To fix this:\n\
          - Set `esi_client.set_client_id(client_id)`\n\
          - You can obtain a client ID at:\n\
              https://developers.eveonline.com/applications\n\
        \n\
        This is required for accessing EVE Online OAuth2 and gated ESI routes."
    )]
    MissingClientId,

    #[error(
        "Missing ESI client secret.\n\
        \n\
        To fix this:\n\
          - Set `esi_client.set_client_secret(client_secret)`\n\
          - You can obtain a client secret at:\n\
              https://developers.eveonline.com/applications\n\
        \n\
        This is required for accessing EVE Online OAuth2 and gated ESI routes."
    )]
    MissingClientSecret,
    #[error(
        "Missing ESI callback URL.\n\
        \n\
        To fix this:\n\
          - Set `esi_client.set_callback_url(callback_url)`\n\
          - Ensure it matches the callback URL set at:\n\
              https://developers.eveonline.com/applications\n\
        \n\
        This is required for accessing EVE Online OAuth2 and gated ESI routes."
    )]
    MissingCallbackUrl,

    #[error("Parse error:\n  {0}")]
    ParseError(String),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}
