use std::time::Duration;

// Default EVE Online API URLs
/// Default EVE Online ESI URL
pub static DEFAULT_ESI_URL: &str = "https://esi.evetech.net";
/// Default EVE Online authorization URL for single sign-on
pub static DEFAULT_AUTH_URL: &str = "https://login.eveonline.com/v2/oauth/authorize";
/// Default EVE Online OAuth2 token URL used for retrieving access tokens
pub static DEFAULT_TOKEN_URL: &str = "https://login.eveonline.com/v2/oauth/token";
/// Default EVE Online OAuth2 JWK URL used for validating access tokens
pub static DEFAULT_JWK_URL: &str = "https://login.eveonline.com/oauth/jwks";

// Default JWT key cache settings
/// Default JWT key cache lifetime before expiration (3600 seconds representing 1 hour)
pub static DEFAULT_JWK_CACHE_TTL: Duration = Duration::from_secs(3600);
// Default JWT key cache empty or expired cache settings
/// Default maximum number of retries for JWT key refresh when cache is empty or expired (2 retries)
pub static DEFAULT_JWK_REFRESH_MAX_RETRIES: u32 = 2;
/// Default backoff period after a JWT key refresh failure when cache is empty or expired (100 milliseconds)
pub static DEFAULT_JWK_REFRESH_BACKOFF: Duration = Duration::from_millis(100);
/// Default timeout when waiting for another thread to refresh JWT key (5 seconds)
pub static DEFAULT_JWK_REFRESH_TIMEOUT: Duration = Duration::from_secs(5);
/// Default cooldown period after a failed set of JWT key refresh attempts (default 60 seconds)
pub static DEFAULT_JWK_REFRESH_COOLDOWN: Duration = Duration::from_secs(60);

// Default JWT key cache background refresh settings
/// Default percentage of JWK_CACHE_TTL for when the background JWT key refresh is triggered (80%)
pub static DEFAULT_JWK_BACKGROUND_REFRESH_THRESHOLD_PERCENT: u64 = 80;

// Default JWT settings
/// Default EVE Online login server URL which represents the expected issuer of tokens
pub static DEFAULT_JWT_ISSUER: &str = "https://login.eveonline.com";
/// Default intended audience which JWT tokens will be used with
pub static DEFAULT_JWT_AUDIENCE: &str = "EVE Online";
