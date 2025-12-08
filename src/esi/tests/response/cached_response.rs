//! Tests for CachedResponse type.

use crate::esi::response::{CachedResponse, EsiResponse};

/// Tests is_fresh method on Fresh variant.
///
/// Verifies that when a CachedResponse contains fresh data,
/// the is_fresh method returns true and is_not_modified returns false.
///
/// Expected: is_fresh() = true, is_not_modified() = false
#[test]
fn test_is_fresh() -> Result<(), crate::Error> {
    let response = EsiResponse::new(vec![1, 2, 3]);
    let cached = CachedResponse::Fresh(response);

    assert!(cached.is_fresh());
    assert!(!cached.is_not_modified());

    Ok(())
}

/// Tests is_not_modified method on NotModified variant.
///
/// Verifies that when a CachedResponse indicates not modified (304),
/// the is_not_modified method returns true and is_fresh returns false.
///
/// Expected: is_fresh() = false, is_not_modified() = true
#[test]
fn test_is_not_modified() -> Result<(), crate::Error> {
    let cached: CachedResponse<EsiResponse<Vec<i32>>> = CachedResponse::NotModified;

    assert!(!cached.is_fresh());
    assert!(cached.is_not_modified());

    Ok(())
}

/// Tests pattern matching on Fresh variant.
///
/// Verifies that the Fresh variant can be pattern matched and that
/// the inner EsiResponse data is accessible and correct.
///
/// Expected: Pattern match succeeds and extracts correct data
#[test]
fn test_fresh_pattern_matching() -> Result<(), crate::Error> {
    let response = EsiResponse::new("test");
    let cached = CachedResponse::Fresh(response);

    match cached {
        CachedResponse::Fresh(data) => {
            assert_eq!(data.data, "test");
        }
        CachedResponse::NotModified => {
            panic!("Expected Fresh variant");
        }
    }

    Ok(())
}

/// Tests pattern matching on NotModified variant.
///
/// Verifies that the NotModified variant can be pattern matched
/// and correctly distinguishes from the Fresh variant.
///
/// Expected: Pattern match succeeds on NotModified arm
#[test]
fn test_not_modified_pattern_matching() -> Result<(), crate::Error> {
    let cached: CachedResponse<EsiResponse<String>> = CachedResponse::NotModified;

    match cached {
        CachedResponse::Fresh(_) => {
            panic!("Expected NotModified variant");
        }
        CachedResponse::NotModified => {
            // Success
        }
    }

    Ok(())
}

/// Tests Clone trait implementation on Fresh variant.
///
/// Verifies that a CachedResponse with fresh data can be cloned
/// and that the cloned instance is also identified as fresh.
///
/// Expected: Cloned instance is_fresh() returns true
#[test]
fn test_fresh_clone() -> Result<(), crate::Error> {
    let response = EsiResponse::new(42);
    let cached = CachedResponse::Fresh(response);
    let cloned = cached.clone();

    assert!(cloned.is_fresh());

    Ok(())
}

/// Tests Clone trait implementation on NotModified variant.
///
/// Verifies that a CachedResponse with NotModified status can be cloned
/// and that the cloned instance maintains the not modified status.
///
/// Expected: Cloned instance is_not_modified() returns true
#[test]
fn test_not_modified_clone() -> Result<(), crate::Error> {
    let cached: CachedResponse<EsiResponse<i32>> = CachedResponse::NotModified;
    let cloned = cached.clone();

    assert!(cloned.is_not_modified());

    Ok(())
}
