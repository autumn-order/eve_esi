// Default EVE Online API URLs
pub static DEFAULT_ESI_URL: &str = "https://esi.evetech.net";
pub static DEFAULT_AUTH_URL: &str = "https://login.eveonline.com/v2/oauth/authorize";
pub static DEFAULT_TOKEN_URL: &str = "https://login.eveonline.com/v2/oauth/token";
pub static DEFAULT_JWK_URL: &str = "https://login.eveonline.com/oauth/jwks";

// Default jwk cache TTL for EsiClient
pub static DEFAULT_JWK_CACHE_TTL: u64 = 3600; // 1 hour cache TTL
