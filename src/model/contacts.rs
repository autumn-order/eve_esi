//! # EVE ESI Clone Models
//!
//! Provides clone-related structs for EVE Online
//!
//! ## ESI Documentation
//! - <https://developers.eveonline.com/api-explorer#>
//!
//! ## Models
//! | Model               | Description                     |
//! | ------------------- | ------------------------------- |
//! | [`AllianceContact`] | A contact entry for an alliance |

use serde::{Deserialize, Serialize};

use crate::model::enums::contacts::ContactType;

/// A contact entry for an alliance
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/AlliancesAllianceIdContactsGet>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AllianceContact {
    /// Unique ID of the contact
    contact_id: i64,
    /// Indicates whether contact ID is a `character`, `corporation`, `alliance`, or `faction`
    contact_type: ContactType,
    /// List of unique IDs applied to the contact entry
    label_ids: Vec<i64>,
    /// Standings towards the contact
    standing: f64,
}
