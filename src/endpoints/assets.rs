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
    model::asset::{Asset, AssetLocation, AssetName},
    scope::AssetsScopes,
    Client, Error, ScopeBuilder,
};

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

    define_endpoint! {
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
        /// - `Vec<`[`Asset`]>`: Paginated list of assets for the provided character ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_character_assets(
            access_token: &str,
            character_id: i64,
            page: i32
        ) -> Result<Vec<Asset>, Error>
        url = "{}/characters/{}/assets?page={}";
        label = "assets";
        required_scopes = ScopeBuilder::new()
            .assets(AssetsScopes::new().read_assets())
            .build();
    }

    define_endpoint! {
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
        /// - `item_ids`     (`Vec<i64>`): Vec of item IDs to get coordinates for (Limit of 1000 IDs per request)
        /// - `character_id` (`i64`): The ID of the character to retrieve asset locations for.
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`Asset`]`>`: List of structs containing coordinates for items' location in space
        /// - [`Error`]: An error if the fetch request fails
        auth_post get_character_asset_locations(
            access_token: &str,
            item_ids: Vec<i64>,
            character_id: i64;
        ) -> Result<Vec<AssetLocation>, Error>
        url = "{}/characters/{}/assets/locations";
        label = "asset locations";
        required_scopes = ScopeBuilder::new()
            .assets(AssetsScopes::new().read_assets())
            .build();
    }

    define_endpoint! {
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
        /// - `item_ids`     (`Vec<i64>`): Vec of item IDs to get names for (Limit of 1000 IDs per request)
        /// - `character_id` (`i64`): The ID of the character to retrieve asset locations for.
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`AssetName`]`>`: List of item names from list of item IDs & a character's ID
        /// - [`Error`]: An error if the fetch request fails
        auth_post get_character_asset_names(
            access_token: &str,
            item_ids: Vec<i64>,
            character_id: i64;
        ) -> Result<Vec<AssetName>, Error>
        url = "{}/characters/{}/assets/names";
        label = "asset names";
        required_scopes = ScopeBuilder::new()
            .assets(AssetsScopes::new().read_assets())
            .build();
    }

    define_endpoint! {
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
        /// - `Vec<`[`Asset`]`>`: Paginated list of assets for the provided corporation's ID
        /// - [`Error`]: An error if the fetch request fails
        auth_get get_corporation_assets(
            access_token: &str,
            corporation_id: i64,
            page: i32
        ) -> Result<Vec<Asset>, Error>
        url = "{}/corporations/{}/assets?page={}";
        label = "assets";
        required_scopes = ScopeBuilder::new()
            .assets(AssetsScopes::new().read_corporation_assets())
            .build();
    }

    define_endpoint! {
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
        /// - `item_ids`       (`Vec<i64>`): Vec of item IDs to get coordinates for (Limit of 1000 IDs per request)
        /// - `corporation_id` (`i64`): The ID of the corporation to retrieve asset locations for.
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`Asset`]`>`: List of structs containing coordinates for items' location in space
        /// - [`Error`]: An error if the fetch request fails
        auth_post get_corporation_asset_locations(
            access_token: &str,
            item_ids: Vec<i64>,
            corporation_id: i64;
        ) -> Result<Vec<AssetLocation>, Error>
        url = "{}/corporations/{}/assets/locations";
        label = "asset locations";
        required_scopes = ScopeBuilder::new()
            .assets(AssetsScopes::new().read_corporation_assets())
            .build();
    }

    define_endpoint! {
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
        /// - `item_ids`       (`Vec<i64>`): Vec of item IDs to get names for (Limit of 1000 IDs per request)
        /// - `corporation_id` (`i64`): The ID of the corporation to retrieve asset locations for.
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - `Vec<`[`AssetName`]`>`: List of item names from list of item IDs & a corporation's ID
        /// - [`Error`]: An error if the fetch request fails
        auth_post get_corporation_asset_names(
            access_token: &str,
            item_ids: Vec<i64>,
            corporation_id: i64;
        ) -> Result<Vec<AssetName>, Error>
        url = "{}/corporations/{}/assets/names";
        label = "asset names";
        required_scopes = ScopeBuilder::new()
            .assets(AssetsScopes::new().read_corporation_assets())
            .build();
    }
}
