//! Tests for Language enum and its string conversions.

use crate::esi::request::Language;

/// Tests English language code conversion.
///
/// Verifies that the English variant correctly converts to the
/// ISO 639-1 language code "en".
///
/// Expected: as_str() returns "en"
#[test]
fn test_english() -> Result<(), crate::Error> {
    assert_eq!(Language::English.as_str(), "en");

    Ok(())
}

/// Tests German language code conversion.
///
/// Verifies that the German variant correctly converts to the
/// ISO 639-1 language code "de".
///
/// Expected: as_str() returns "de"
#[test]
fn test_german() -> Result<(), crate::Error> {
    assert_eq!(Language::German.as_str(), "de");

    Ok(())
}

/// Tests French language code conversion.
///
/// Verifies that the French variant correctly converts to the
/// ISO 639-1 language code "fr".
///
/// Expected: as_str() returns "fr"
#[test]
fn test_french() -> Result<(), crate::Error> {
    assert_eq!(Language::French.as_str(), "fr");

    Ok(())
}

/// Tests Japanese language code conversion.
///
/// Verifies that the Japanese variant correctly converts to the
/// ISO 639-1 language code "ja".
///
/// Expected: as_str() returns "ja"
#[test]
fn test_japanese() -> Result<(), crate::Error> {
    assert_eq!(Language::Japanese.as_str(), "ja");

    Ok(())
}

/// Tests Russian language code conversion.
///
/// Verifies that the Russian variant correctly converts to the
/// ISO 639-1 language code "ru".
///
/// Expected: as_str() returns "ru"
#[test]
fn test_russian() -> Result<(), crate::Error> {
    assert_eq!(Language::Russian.as_str(), "ru");

    Ok(())
}

/// Tests Chinese language code conversion.
///
/// Verifies that the Chinese variant correctly converts to the
/// ISO 639-1 language code "zh".
///
/// Expected: as_str() returns "zh"
#[test]
fn test_chinese() -> Result<(), crate::Error> {
    assert_eq!(Language::Chinese.as_str(), "zh");

    Ok(())
}

/// Tests Korean language code conversion.
///
/// Verifies that the Korean variant correctly converts to the
/// ISO 639-1 language code "ko".
///
/// Expected: as_str() returns "ko"
#[test]
fn test_korean() -> Result<(), crate::Error> {
    assert_eq!(Language::Korean.as_str(), "ko");

    Ok(())
}

/// Tests Spanish language code conversion.
///
/// Verifies that the Spanish variant correctly converts to the
/// ISO 639-1 language code "es".
///
/// Expected: as_str() returns "es"
#[test]
fn test_spanish() -> Result<(), crate::Error> {
    assert_eq!(Language::Spanish.as_str(), "es");

    Ok(())
}

/// Tests Clone trait implementation on Language.
///
/// Verifies that Language enum can be cloned and that the cloned
/// instance equals the original.
///
/// Expected: Cloned language equals original
#[test]
fn test_clone() -> Result<(), crate::Error> {
    let lang = Language::English;
    let cloned = lang.clone();
    assert_eq!(lang, cloned);

    Ok(())
}

/// Tests Copy trait implementation on Language.
///
/// Verifies that Language enum implements Copy, allowing it to be
/// copied implicitly without consuming the original value.
///
/// Expected: Both original and copied values are usable
#[test]
fn test_copy() -> Result<(), crate::Error> {
    let lang = Language::German;
    let copied = lang;
    assert_eq!(lang, copied);
    assert_eq!(lang.as_str(), "de");

    Ok(())
}

/// Tests PartialEq trait implementation on Language.
///
/// Verifies that Language enum correctly implements equality comparison,
/// with identical variants comparing as equal and different variants
/// comparing as not equal.
///
/// Expected: Same variants equal, different variants not equal
#[test]
fn test_equality() -> Result<(), crate::Error> {
    assert_eq!(Language::English, Language::English);
    assert_ne!(Language::English, Language::German);

    Ok(())
}

/// Tests Debug trait implementation on Language.
///
/// Verifies that Language enum has a Debug implementation that
/// produces a string containing the variant name.
///
/// Expected: Debug output contains the variant name
#[test]
fn test_debug() -> Result<(), crate::Error> {
    let lang = Language::French;
    let debug_str = format!("{:?}", lang);
    assert!(debug_str.contains("French"));

    Ok(())
}
