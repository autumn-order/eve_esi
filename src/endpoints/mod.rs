use crate::EsiClient;

pub mod alliance;
pub mod character;
pub mod corporation;

impl EsiClient {
    /// Access to Alliance ESI endpoints
    ///
    /// Returns an API client for interacting with alliance-related endpoints.
    pub fn alliance(&self) -> self::alliance::AllianceAPI {
        self::alliance::AllianceAPI::new(self)
    }
}
