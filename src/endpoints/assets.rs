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
//! ## Endpoints (0)
//!
//! ### Authenticated (0)
//!
//! | Endpoint                                           | Description                                                                          |
//! | -------------------------------------------------- | ------------------------------------------------------------------------------------ |
//! | [`AssetsEndpoints::get_character_assets`]          | Get paginated list of assets for the provided character's ID                         |
//! | [`AssetsEndpoints::get_character_asset_locations`] | Get list of coordinates for items' location in space using item IDs & character's ID |

use crate::{
    model::asset::{Asset, AssetLocation},
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
        /// Get paginated list of assets for the provided character's ID
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
        /// - Vec<[`Asset`]>: Paginated list of assets for the provided character's ID
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
        /// Get list of coordinates for items' location in space using item IDs & character's ID
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
        /// - `item_ids`     (`Vec<i64>`): Vec of item IDs to get coordinates for
        /// - `character_id` (`i64`): The ID of the character to retrieve asset locations for.
        ///
        /// # Returns
        /// Returns a [`Result`] containing either:
        /// - Vec<[`Asset`]>: List of structs containing coordinates for items' location in space
        /// - [`Error`]: An error if the fetch request fails
        auth_post get_character_asset_locations(
            access_token: &str,
            item_ids: Vec<i64>,
            character_id: i64,
        ) -> Result<Vec<AssetLocation>, Error>
        url = "{}/characters/{}/assets/locations";
        label = "asset locations";
        required_scopes = ScopeBuilder::new()
            .assets(AssetsScopes::new().read_assets())
            .build();
    }
}
