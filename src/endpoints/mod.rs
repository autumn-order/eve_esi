use crate::EsiClient;

pub mod alliances;
pub mod characters;
pub mod corporations;

impl EsiClient {
    /// Access to Alliance ESI endpoints
    ///
    /// Returns an API client for interacting with alliance-related endpoints.
    pub fn alliances(&self) -> self::alliances::AllianceApi<'_> {
        self::alliances::AllianceApi::new(self)
    }

    /// Access to Character ESI endpoints
    ///
    /// Returns an API client for interacting with character-related endpoints.
    pub fn characters(&self) -> self::characters::CharacterApi<'_> {
        self::characters::CharacterApi::new(self)
    }

    /// Access to Corporation ESI endpoints
    ///
    /// Returns an API client for interacting with corporation-related endpoints.
    pub fn corporations(&self) -> self::corporations::CorporationApi<'_> {
        self::corporations::CorporationApi::new(self)
    }
}
