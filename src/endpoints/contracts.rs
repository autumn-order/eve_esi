//! # EVE ESI Contracts Endpoints
//!
//! This module provides the [`ContractsEndpoints`] struct and associated methods for accessing
//! contract-related ESI endpoints.
//!
//! For an overview & usage examples, see the [endpoints module documentation](super)
use crate::{
    esi::EsiRequest,
    model::contract::{PublicContract, PublicContractItem},
    Client,
};
use reqwest::Method;

/// Provides methods for accessing contract-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct ContractsEndpoints<'a> {
    client: &'a Client,
}

impl<'a> ContractsEndpoints<'a> {
    /// Creates a new instance of [`ContractsEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    define_esi_endpoint! {
        /// Retrieves a list of paginated public contracts in the provided region ID
        ///
        /// # ESI Documentation
        /// <https://developers.eveonline.com/api-explorer#/operations/GetContractsPublicRegionId>
        ///
        /// # Arguments
        /// - `region_id`   (`i64`): The ID of the region to retrieve public contracts for.
        /// - `page`        (`i32`): The page of contracts to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a paginated vector of public contracts when sent.
        pub fn get_public_contracts(
            region_id: i64;
            page: i32
        ) -> EsiRequest<Vec<PublicContract>>
        method = Method::GET;
        path = "/contracts/public/{}";
    }

    define_esi_endpoint! {
        /// Retrieves a paginated list of items for the provided contract ID
        ///
        /// # ESI Documentation
        /// <https://developers.eveonline.com/api-explorer#/operations/GetContractsPublicItemsContractId>
        ///
        /// # Arguments
        /// - `contract_id`   (`i64`): The ID of the contract to retrieve items.
        /// - `page`        (`i32`): The page of contract items to retrieve, page numbers start at `1`
        ///
        /// # Returns
        /// An ESI request builder that returns a paginated vector of contract items when sent.
        pub fn get_public_contract_items(
            contract_id: i64;
            page: i32
        ) -> EsiRequest<Vec<PublicContractItem>>
        method = Method::GET;
        path = "/contracts/public/items/{}";
    }
}
