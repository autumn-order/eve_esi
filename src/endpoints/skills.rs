//! # EVE ESI Skill Endpoints
//!
//! This module provides the [`SkillsEndpoints`] struct and associated methods for accessing
//! skill-related ESI endpoints.

use crate::Client;

/// Provides methods for accessing skill-related endpoints of the EVE Online ESI API.
///
/// For an overview & usage examples, see the [endpoints module documentation](super)
pub struct SkillsEndpoints<'a> {
    client: &'a Client,
}

impl<'a> SkillsEndpoints<'a> {
    /// Creates a new instance of [`SkillsEndpoints`].
    ///
    /// # Arguments
    /// - `client` (&[`Client`]): ESI client used for making HTTP requests to the ESI endpoints.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
