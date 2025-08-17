//! JSON Web Keys (JWK) Management for EVE ESI OAuth2
//!
//! This module provides a comprehensive implementation for managing JSON Web Keys (JWKs)
//! used in OAuth2 authentication with EVE Online's ESI API. It handles fetching, caching,
//! validation, and automatic background refreshing of JWT keys with thread-safe operations.
//!
//! # Submodules
//!
//! The JWT key management is divided into several components:
//!
//! - **Core API** [`jwk`]: Provides the public interface for JWT key operations
//! - **Cache Management** [`cache`]: Handles the storage and retrieval of JWT keys
//! - **Task Management** [`task`]: Implements background refresh tasks and retry logic
//! - **Utilities** [`util`]: Contains helper functions for cache expiry, backoff, etc.
//!
//! # Key Features
//!
//! - **Thread-safe caching**: All operations are designed for concurrent access
//! - **Proactive refreshing**: Keys are refreshed before they expire
//! - **Efficient coordination**: Prevents redundant refresh operations
//! - **Backoff mechanism**: Implements progressive backoff for API failures
//! - **Timeout handling**: Prevents indefinite waiting for refresh operations
//!
//! # Usage
//!
//! The main entry point for this module is the [`super::OAuth2Api::get_jwt_keys`] method, which:
//! 1. Returns cached keys if they're valid
//! 2. Triggers background refresh if keys are approaching expiry
//! 3. Fetches new keys if the cache is empty or expired
//! 4. Coordinates with other threads if a refresh is already in progress
//!
//! # Thread Safety
//!
//! This module is designed for concurrent access with appropriate synchronization:
//! - Read locks for cache access
//! - Atomic flags for refresh coordination
//! - Notification mechanisms for efficient waiting

mod cache;
pub mod jwk;
mod task;
mod util;
