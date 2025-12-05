//! # EVE ESI Assets Endpoints
//!
//! This module provides the [`AssetsEndpoints`] struct and associated methods for accessing
//! asset-related ESI endpoints.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer>
//!
//! ## Endpoints (6)
//!
//! ### Authenticated (6)
//!
//! | Endpoint                                             | Description                                                                                       |
//! | ---------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
//! | [`AssetsEndpoints::get_character_assets`]            | Get paginated list of assets for the provided character's ID                                      |
//! | [`AssetsEndpoints::get_character_asset_locations`]   | Get list of coordinates for items' location in space using item IDs & character's ID              |
//! | [`AssetsEndpoints::get_character_asset_names`]       | Get list of item names from list of item IDs & a character's ID                                   |
//! | [`AssetsEndpoints::get_corporation_assets`]          | Get paginated list of assets for the provided corporation ID                                      |
//! | [`AssetsEndpoints::get_corporation_asset_locations`] | Get list of coordinates for items' location in space using provided item IDs & a corporation's ID |
//! | [`AssetsEndpoints::get_corporation_asset_names`]     | Get list of item names from list of item IDs & a corporation's ID                                 |

use crate::{
    esi::EsiRequest,
    model::asset::{Asset, AssetLocation, AssetName},
    scope::AssetsScopes,
    Client, Error, ScopeBuilder,
};
use reqwest::Method;

/// Provides methods for accessing asset-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct AssetsEndpoints<'a> {
    client: &'a Client,
}

impl<'a> AssetsEndpoints<'a> {
    /// Creates a new instance of [`AssetsEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    define_esi_endpoint! {
        /// Get paginated list of assets for the provided character ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdAssets>
        ///
        /// # Required Scopes
        /// - [`AssetsScopes::read_assets`](crate::scope::AssetsScopes::read_assets):
        ///   `esi-assets.read_assets.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id`  (`i64`): The ID of the character to retrieve assets for.
        /// - `page`          (`i32`): The page of assets to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Ok(request)`: Request builder for a vector of assets
        /// - `Err(Error::UrlParseError)`: Failed to construct the endpoint URL
        auth fn get_character_assets(
            access_token: &str,
            character_id: i64;
            page: i32
        ) -> Result<EsiRequest<Vec<Asset>>, Error>
        method = Method::GET;
        url = "{}/characters/{}/assets";
        required_scopes = ScopeBuilder::new()
            .assets(AssetsScopes::new().read_assets())
            .build();
    }

    define_esi_endpoint! {
        /// Get list of coordinates for items' location in space using provided item IDs & a character's ID
        ///
        /// You can get the item IDs using the [`AssetsEndpoints::get_character_assets`] method
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/PostCharactersCharacterIdAssetsLocations>
        ///
        /// # Required Scopes
        /// - [`AssetsScopes::read_assets`](crate::scope::AssetsScopes::read_assets):
        ///   `esi-assets.read_assets.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve asset locations for.
        /// - `item_ids`     (`Vec<i64>`): Vec of item IDs to get coordinates for (Limit of 1000 IDs per request)
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Ok(request)`: Request builder for a vector of asset locations
        /// - `Err(Error::UrlParseError)`: Failed to construct the endpoint URL
        auth fn get_character_asset_locations(
            access_token: &str,
            character_id: i64
        ) -> Result<EsiRequest<Vec<AssetLocation>>, Error>
        method = Method::POST;
        url = "{}/characters/{}/assets/locations";
        required_scopes = ScopeBuilder::new()
            .assets(AssetsScopes::new().read_assets())
            .build();
        body = item_ids: Vec<i64>;
    }

    define_esi_endpoint! {
        /// Get list of item names from list of item IDs & a character's ID
        ///
        /// Useful for retrieving the names of items with customizable names such as ships or
        /// containers.
        ///
        /// You can get the item IDs using the [`AssetsEndpoints::get_character_assets`] method
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/PostCharactersCharacterIdAssetsNames>
        ///
        /// # Required Scopes
        /// - [`AssetsScopes::read_assets`](crate::scope::AssetsScopes::read_assets):
        ///   `esi-assets.read_assets.v1`
        ///
        /// # Arguments
        /// - `access_token` (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `character_id` (`i64`): The ID of the character to retrieve asset names for.
        /// - `item_ids`     (`Vec<i64>`): Vec of item IDs to get names for (Limit of 1000 IDs per request)
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Ok(request)`: Request builder for a vector of asset names
        /// - `Err(Error::UrlParseError)`: Failed to construct the endpoint URL
        auth fn get_character_asset_names(
            access_token: &str,
            character_id: i64
        ) -> Result<EsiRequest<Vec<AssetName>>, Error>
        method = Method::POST;
        url = "{}/characters/{}/assets/names";
        required_scopes = ScopeBuilder::new()
            .assets(AssetsScopes::new().read_assets())
            .build();
        body = item_ids: Vec<i64>;
    }

    define_esi_endpoint! {
        /// Get paginated list of assets for the provided corporation ID
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/GetCorporationsCorporationIdAssets>
        ///
        /// # Required Scopes
        /// - [`AssetsScopes::read_corporation_assets`](crate::scope::AssetsScopes::read_corporation_assets):
        ///   `esi-assets.read_corporation_assets.v1`
        ///
        /// # Arguments
        /// - `access_token`    (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id`  (`i64`): The ID of the corporation to retrieve assets for.
        /// - `page`            (`i32`): The page of assets to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Ok(request)`: Request builder for a vector of assets
        /// - `Err(Error::UrlParseError)`: Failed to construct the endpoint URL
        auth fn get_corporation_assets(
            access_token: &str,
            corporation_id: i64;
            page: i32
        ) -> Result<EsiRequest<Vec<Asset>>, Error>
        method = Method::GET;
        url = "{}/corporations/{}/assets";
        required_scopes = ScopeBuilder::new()
            .assets(AssetsScopes::new().read_corporation_assets())
            .build();
    }

    define_esi_endpoint! {
        /// Get list of coordinates for items' location in space using provided item IDs & a corporation's ID
        ///
        /// You can get the item IDs using the [`AssetsEndpoints::get_corporation_assets`] method
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/PostCorporationsCorporationIdAssetsLocations>
        ///
        /// # Required Scopes
        /// - [`AssetsScopes::read_corporation_assets`](crate::scope::AssetsScopes::read_corporation_assets):
        ///   `esi-assets.read_corporation_assets.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id` (`i64`): The ID of the corporation to retrieve asset locations for.
        /// - `item_ids`       (`Vec<i64>`): Vec of item IDs to get coordinates for (Limit of 1000 IDs per request)
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Ok(request)`: Request builder for a vector of asset locations
        /// - `Err(Error::UrlParseError)`: Failed to construct the endpoint URL
        auth fn get_corporation_asset_locations(
            access_token: &str,
            corporation_id: i64
        ) -> Result<EsiRequest<Vec<AssetLocation>>, Error>
        method = Method::POST;
        url = "{}/corporations/{}/assets/locations";
        required_scopes = ScopeBuilder::new()
            .assets(AssetsScopes::new().read_corporation_assets())
            .build();
        body = item_ids: Vec<i64>;
    }

    define_esi_endpoint! {
        /// Get list of item names from list of item IDs & a corporation's ID
        ///
        /// Useful for retrieving the names of items with customizable names such as ships or
        /// containers.
        ///
        /// You can get the item IDs using the [`AssetsEndpoints::get_character_assets`] method
        ///
        /// For an overview & usage examples, see the [endpoints module documentation](super)
        ///
        /// # ESI Documentation
        /// - <https://developers.eveonline.com/api-explorer#/operations/PostCorporationsCorporationIdAssetsNames>
        ///
        /// # Required Scopes
        /// - [`AssetsScopes::read_corporation_assets`](crate::scope::AssetsScopes::read_corporation_assets):
        ///   `esi-assets.read_corporation_assets.v1`
        ///
        /// # Arguments
        /// - `access_token`   (`&str`): Access token used for authenticated ESI routes in string format.
        /// - `corporation_id` (`i64`): The ID of the corporation to retrieve asset names for.
        /// - `item_ids`       (`Vec<i64>`): Vec of item IDs to get names for (Limit of 1000 IDs per request)
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Ok(request)`: Request builder for a vector of asset names
        /// - `Err(Error::UrlParseError)`: Failed to construct the endpoint URL
        auth fn get_corporation_asset_names(
            access_token: &str,
            corporation_id: i64
        ) -> Result<EsiRequest<Vec<AssetName>>, Error>
        method = Method::POST;
        url = "{}/corporations/{}/assets/names";
        required_scopes = ScopeBuilder::new()
            .assets(AssetsScopes::new().read_corporation_assets())
            .build();
        body = item_ids: Vec<i64>;
    }
}
