//! Data structures for EVE Online ESI API.
//!
//! This module provides Rust structs and types that represent core entities in the EVE Online universe,
//! as defined by the EVE Online Stable Infrastructure (ESI) API. These models are used for serialization and
//! deserialization of data exchanged with the ESI endpoints.
//!
//! # Submodules
//! - `alliance`: Types for representing alliances.
//! - `character`: Types for representing characters and their affiliations.
//! - `corporation`: Types for representing corporations.
//! - `oauth2`: Types for handling OAuth2 authentication data.
//!
//! # Usage
//! Use these models to interact with EVE ESI API responses and requests in a type-safe manner.
//!
//! # References
//! - [ESI API Documentation](https://developers.eveonline.com/api-explorer)
//! - [EVE SSO Documentation](https://developers.eveonline.com/docs/services/sso/)

pub mod alliance;
pub mod asset;
pub mod calendar;
pub mod character;
pub mod clones;
pub mod corporation;
pub mod enums;
pub mod market;
pub mod oauth2;
pub mod standing;
