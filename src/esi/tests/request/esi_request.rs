//! Tests for EsiRequest builder and configuration methods.

use crate::esi::request::{EsiRequest, Language};
use crate::Client;
use reqwest::Method;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
struct TestResponse {
    test: String,
}

fn create_test_client() -> Client {
    Client::new("TestApp/1.0").unwrap()
}

/// Tests creating a new request with a leading slash in the endpoint path.
///
/// Verifies that EsiRequest correctly constructs the full URL when the
/// endpoint path starts with a forward slash, properly combining it with
/// the base ESI URL.
///
/// Expected: Endpoint URL contains "/status" and starts with "https://"
#[test]
fn test_new_with_leading_slash() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request = EsiRequest::<TestResponse>::new(&client, "/status/");

    assert!(request.endpoint().contains("/status"));
    assert!(request.endpoint().starts_with("https://"));

    Ok(())
}

/// Tests creating a new request without a leading slash in the endpoint path.
///
/// Verifies that EsiRequest correctly constructs the full URL when the
/// endpoint path does not start with a forward slash, properly combining
/// it with the base ESI URL.
///
/// Expected: Endpoint URL contains "/status" and starts with "https://"
#[test]
fn test_new_without_leading_slash() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request = EsiRequest::<TestResponse>::new(&client, "status/");

    assert!(request.endpoint().contains("/status"));
    assert!(request.endpoint().starts_with("https://"));

    Ok(())
}

/// Tests default HTTP method is GET.
///
/// Verifies that a newly created EsiRequest uses GET as the default
/// HTTP method when no explicit method is set.
///
/// Expected: method() returns Method::GET
#[test]
fn test_default_method_is_get() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request = EsiRequest::<TestResponse>::new(&client, "/status/");

    assert_eq!(request.method(), &Method::GET);

    Ok(())
}

/// Tests setting a custom HTTP method.
///
/// Verifies that the with_method builder method correctly sets
/// the HTTP method to a non-default value (POST).
///
/// Expected: method() returns the configured Method::POST
#[test]
fn test_with_method() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request = EsiRequest::<TestResponse>::new(&client, "/status/").with_method(Method::POST);

    assert_eq!(request.method(), &Method::POST);

    Ok(())
}

/// Tests setting an access token for authentication.
///
/// Verifies that the with_access_token builder method correctly stores
/// the OAuth2 access token for use in authenticated requests.
///
/// Expected: access_token() returns Some with the provided token
#[test]
fn test_with_access_token() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request =
        EsiRequest::<TestResponse>::new(&client, "/status/").with_access_token("test_token_123");

    assert_eq!(request.access_token(), Some("test_token_123"));

    Ok(())
}

/// Tests default access token is None.
///
/// Verifies that a newly created EsiRequest has no access token set
/// by default, indicating an unauthenticated request.
///
/// Expected: access_token() returns None
#[test]
fn test_access_token_none_by_default() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request = EsiRequest::<TestResponse>::new(&client, "/status/");

    assert_eq!(request.access_token(), None);

    Ok(())
}

/// Tests setting the X-Compatibility-Date header.
///
/// Verifies that the with_compatibility_date builder method correctly
/// adds the X-Compatibility-Date header with the specified value.
///
/// Expected: Headers contain X-Compatibility-Date with the provided date
#[test]
fn test_with_compatibility_date() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request =
        EsiRequest::<TestResponse>::new(&client, "/status/").with_compatibility_date("2025-11-06");

    assert_eq!(
        request.headers().get("X-Compatibility-Date"),
        Some(&"2025-11-06".to_string())
    );

    Ok(())
}

/// Tests setting the X-Tenant header.
///
/// Verifies that the with_tenant builder method correctly adds the
/// X-Tenant header for specifying the EVE server (e.g., tranquility).
///
/// Expected: Headers contain X-Tenant with the provided value
#[test]
fn test_with_tenant() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request = EsiRequest::<TestResponse>::new(&client, "/status/").with_tenant("tranquility");

    assert_eq!(
        request.headers().get("X-Tenant"),
        Some(&"tranquility".to_string())
    );

    Ok(())
}

/// Tests setting the Accept-Language header.
///
/// Verifies that the with_language builder method correctly adds the
/// Accept-Language header using the Language enum for type-safe values.
///
/// Expected: Headers contain Accept-Language with the language code
#[test]
fn test_with_language() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request =
        EsiRequest::<TestResponse>::new(&client, "/status/").with_language(Language::German);

    assert_eq!(
        request.headers().get("Accept-Language"),
        Some(&"de".to_string())
    );

    Ok(())
}

/// Tests setting the If-Match header for conditional requests.
///
/// Verifies that the with_if_match builder method correctly adds the
/// If-Match header with an ETag for conditional updates.
///
/// Expected: Headers contain If-Match with the provided ETag
#[test]
fn test_with_if_match() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request = EsiRequest::<TestResponse>::new(&client, "/status/").with_if_match("\"etag123\"");

    assert_eq!(
        request.headers().get("If-Match"),
        Some(&"\"etag123\"".to_string())
    );

    Ok(())
}

/// Tests setting a custom header.
///
/// Verifies that the with_header builder method correctly adds
/// arbitrary custom headers to the request.
///
/// Expected: Headers contain the custom header with the provided value
#[test]
fn test_with_header() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request = EsiRequest::<TestResponse>::new(&client, "/status/")
        .with_header("X-Custom-Header", "custom_value");

    assert_eq!(
        request.headers().get("X-Custom-Header"),
        Some(&"custom_value".to_string())
    );

    Ok(())
}

/// Tests setting required OAuth2 scopes.
///
/// Verifies that the with_required_scopes builder method correctly stores
/// a vector of scope strings required for authenticated endpoints.
///
/// Expected: required_scopes() returns the provided scope vector
#[test]
fn test_with_required_scopes() -> Result<(), crate::Error> {
    let client = create_test_client();
    let scopes = vec!["scope1".to_string(), "scope2".to_string()];
    let request =
        EsiRequest::<TestResponse>::new(&client, "/status/").with_required_scopes(scopes.clone());

    assert_eq!(request.required_scopes(), &scopes);

    Ok(())
}

/// Tests default required scopes is empty.
///
/// Verifies that a newly created EsiRequest has an empty vector of
/// required scopes by default, indicating no scope requirements.
///
/// Expected: required_scopes() returns an empty vector
#[test]
fn test_required_scopes_empty_by_default() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request = EsiRequest::<TestResponse>::new(&client, "/status/");

    assert!(request.required_scopes().is_empty());

    Ok(())
}

/// Tests setting JSON request body.
///
/// Verifies that the with_body_json builder method correctly stores
/// a JSON value for use in POST, PUT, or PATCH requests.
///
/// Expected: body_json() returns Some with the provided JSON value
#[test]
fn test_with_body_json() -> Result<(), crate::Error> {
    let client = create_test_client();
    let body = serde_json::json!({
        "key": "value",
        "number": 42
    });
    let request = EsiRequest::<TestResponse>::new(&client, "/status/").with_body_json(body.clone());

    assert_eq!(request.body_json(), Some(&body));

    Ok(())
}

/// Tests default JSON body is None.
///
/// Verifies that a newly created EsiRequest has no JSON body set
/// by default, appropriate for GET requests.
///
/// Expected: body_json() returns None
#[test]
fn test_body_json_none_by_default() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request = EsiRequest::<TestResponse>::new(&client, "/status/");

    assert_eq!(request.body_json(), None);

    Ok(())
}

/// Tests chaining multiple builder methods.
///
/// Verifies that all builder methods can be chained together in a
/// fluent API style and that all configurations are correctly applied
/// to the resulting request instance.
///
/// Expected: All configured values are present in the final request
#[test]
fn test_chaining_multiple_methods() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request = EsiRequest::<TestResponse>::new(&client, "/characters/12345/")
        .with_method(Method::POST)
        .with_access_token("token123")
        .with_compatibility_date("2025-11-06")
        .with_tenant("tranquility")
        .with_language(Language::English)
        .with_header("X-Custom", "value")
        .with_required_scopes(vec!["scope1".to_string()]);

    assert_eq!(request.method(), &Method::POST);
    assert_eq!(request.access_token(), Some("token123"));
    assert_eq!(
        request.headers().get("X-Compatibility-Date"),
        Some(&"2025-11-06".to_string())
    );
    assert_eq!(
        request.headers().get("X-Tenant"),
        Some(&"tranquility".to_string())
    );
    assert_eq!(
        request.headers().get("Accept-Language"),
        Some(&"en".to_string())
    );
    assert_eq!(
        request.headers().get("X-Custom"),
        Some(&"value".to_string())
    );
    assert_eq!(request.required_scopes().len(), 1);

    Ok(())
}

/// Tests Clone trait implementation on EsiRequest.
///
/// Verifies that EsiRequest can be cloned and that all configuration
/// including access token and HTTP method are preserved in the clone.
///
/// Expected: Cloned request has identical configuration
#[test]
fn test_clone() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request = EsiRequest::<TestResponse>::new(&client, "/status/")
        .with_access_token("token")
        .with_method(Method::POST);

    let cloned = request.clone();
    assert_eq!(cloned.access_token(), Some("token"));
    assert_eq!(cloned.method(), &Method::POST);

    Ok(())
}

/// Tests multiple headers can be set.
///
/// Verifies that the headers HashMap correctly stores multiple
/// custom headers added via the with_header method.
///
/// Expected: Headers map contains all added headers
#[test]
fn test_headers_are_mutable() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request = EsiRequest::<TestResponse>::new(&client, "/status/")
        .with_header("Key1", "Value1")
        .with_header("Key2", "Value2");

    assert_eq!(request.headers().len(), 2);
    assert!(request.headers().contains_key("Key1"));
    assert!(request.headers().contains_key("Key2"));

    Ok(())
}

/// Tests endpoint URL construction with complex paths.
///
/// Verifies that the endpoint URL is correctly constructed for complex
/// paths with multiple segments and that no double slashes are introduced
/// in the path.
///
/// Expected: URL contains the path correctly without double slashes
#[test]
fn test_endpoint_url_construction() -> Result<(), crate::Error> {
    let client = create_test_client();
    let request = EsiRequest::<TestResponse>::new(&client, "/universe/types/34/");

    let endpoint = request.endpoint();
    assert!(endpoint.contains("universe/types/34"));
    assert!(!endpoint.contains("//universe")); // No double slashes

    Ok(())
}
