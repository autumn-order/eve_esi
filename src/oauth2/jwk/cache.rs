//! Provides the JwtKeyCache struct for caching JWT keys
//!
//! For details, see the [`JwtKeyCache`] struct.
//! For a higher level overview of the usage of JWT keys, see [module-level documentation](super)

use std::sync::atomic::AtomicBool;
use std::time::Instant;

use tokio::sync::{Notify, RwLock};

use crate::model::oauth2::EveJwtKeys;

/// OAuth2 JWT key cache
///
/// A cache providing a tuple of [`EveJwtKeys`] and an [`Instant`] timestamp of when the keys
/// were last updated.
///
/// Used by methods [`get_jwt_keys`](crate::oauth2::OAuth2Api::get_jwt_keys) &
/// [`fetch_and_update_cache`](crate::oauth2::OAuth2Api::fetch_and_update_cache) to cache & refresh
/// JWT keys used to validate tokens retrieved from EVE Online's OAuth2 API.
///
/// Provides fields used to coordinate concurrency across multiple theads such as simulatenous reads,
/// acquiring a lock to prevent duplicate refresh attempts, and a notifier for when a refresh completes.
///
/// # Concurrency
/// - [`RwLock`]: To allow for simultaneous reads of the cache and the last refresh failure timestamp
/// - [`AtomicBool`]: To manage a high volume of simultaneous attempts to acquire a refresh lock
/// - [`Notify`]: To provide notifications of when the cache has been updated
///
/// # Fields
/// - `jwt_key_cache` (RwLock<Option<([`EveJwtKeys`], [`Instant`])>>): RwLock with a tuple containing JWT keys and timestamp of when keys were updated
/// - `jwt_key_refresh_lock` ([`AtomicBool`]): AtomicBool indicating whether a JWT key refresh is currently in progress
/// - `jwt_key_refresh_notifier` ([`Notify`]): Notifier for when a JWT key refresh is completed
/// - `jwt_key_last_refresh_failure` (RwLock<Option<[`Instant`]>): RwLock with a timestamp of last failed set of JWT key refresh attemmpts
pub struct JwtKeyCache {
    /// RwLock with a tuple containing JWT keys and timestamp of when keys were updated
    pub cache: RwLock<Option<(EveJwtKeys, Instant)>>,
    /// AtomicBool indicating whether a JWT key refresh is currently in progress
    pub refresh_lock: AtomicBool,
    /// Notifier for when a JWT key refresh is completed
    pub refresh_notifier: Notify,
    /// RwLock with a timestamp of last failed set of JWT key refresh attemmpts
    pub last_refresh_failure: RwLock<Option<Instant>>,
}

impl JwtKeyCache {
    /// Creates a new instance of [`JwtKeyCache`]
    ///
    /// The cache will start empty and will need to be updated using one of the update
    /// methods such as [`get_jwt_keys`](crate::oauth2::OAuth2Api::get_jwt_keys)
    /// or [`fetch_and_update_cache`](crate::oauth2::OAuth2Api::fetch_and_update_cache).
    ///
    /// # Returns
    /// - [`JwtKeyCache`]: Default cache instance that contains no keys initially
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(None),
            refresh_lock: AtomicBool::new(false),
            refresh_notifier: Notify::new(),
            last_refresh_failure: RwLock::new(None),
        }
    }
}
