//! Integration tests for ESI client functionality.
//!
//! This module organizes client integration tests by public method:
//! - `new_request` - Tests for creating EsiRequest instances
//! - `request` - Tests for standard ESI requests
//! - `request_cached` - Tests for cached ESI requests with 304 support

mod new_request;
mod request;
mod request_cached;
