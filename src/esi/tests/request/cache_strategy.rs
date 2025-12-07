//! Tests for CacheStrategy enum variants.

use crate::esi::request::CacheStrategy;
use chrono::Utc;

/// Tests IfNoneMatch variant with ETag value.
///
/// Verifies that the IfNoneMatch variant correctly stores and provides
/// access to an ETag string for use in conditional requests.
///
/// Expected: IfNoneMatch variant contains the provided ETag
#[test]
fn test_if_none_match() {
    let strategy = CacheStrategy::IfNoneMatch("\"abc123\"".to_string());
    match strategy {
        CacheStrategy::IfNoneMatch(etag) => {
            assert_eq!(etag, "\"abc123\"");
        }
        _ => panic!("Expected IfNoneMatch variant"),
    }
}

/// Tests IfModifiedSince variant with DateTime.
///
/// Verifies that the IfModifiedSince variant correctly stores and provides
/// access to a DateTime<Utc> timestamp for use in conditional requests.
///
/// Expected: IfModifiedSince variant contains the provided timestamp
#[test]
fn test_if_modified_since() {
    let date = Utc::now();
    let strategy = CacheStrategy::IfModifiedSince(date);
    match strategy {
        CacheStrategy::IfModifiedSince(d) => {
            assert_eq!(d, date);
        }
        _ => panic!("Expected IfModifiedSince variant"),
    }
}

/// Tests Both variant with ETag and DateTime.
///
/// Verifies that the Both variant correctly stores and provides access to
/// both an ETag string and a DateTime<Utc> timestamp for use in
/// conditional requests that employ both caching strategies.
///
/// Expected: Both variant contains both the ETag and timestamp
#[test]
fn test_both() {
    let date = Utc::now();
    let strategy = CacheStrategy::Both {
        etag: "\"xyz789\"".to_string(),
        modified_since: date,
    };
    match strategy {
        CacheStrategy::Both {
            etag,
            modified_since,
        } => {
            assert_eq!(etag, "\"xyz789\"");
            assert_eq!(modified_since, date);
        }
        _ => panic!("Expected Both variant"),
    }
}

/// Tests Clone trait implementation on CacheStrategy.
///
/// Verifies that CacheStrategy can be cloned and that the cloned
/// instance contains identical values to the original.
///
/// Expected: Cloned strategy has identical field values
#[test]
fn test_clone() {
    let strategy = CacheStrategy::IfNoneMatch("\"tag\"".to_string());
    let cloned = strategy.clone();
    match cloned {
        CacheStrategy::IfNoneMatch(etag) => {
            assert_eq!(etag, "\"tag\"");
        }
        _ => panic!("Expected IfNoneMatch variant"),
    }
}
