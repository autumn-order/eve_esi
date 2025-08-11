pub mod client;
pub mod error;
pub mod model;
pub mod oauth2;

#[deprecated(since = "0.3.0", note = "Use EsiClient instead")]
pub use crate::client::Client;

pub use crate::client::EsiClient;

mod endpoints;
mod esi;
