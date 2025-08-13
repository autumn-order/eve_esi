use eve_esi::model::oauth2::{EveJwtKey, EveJwtKeys};

/// Helper function to create mock JWT keys for testing
pub fn create_mock_jwt_keys() -> EveJwtKeys {
    EveJwtKeys {
        skip_unresolved_json_web_keys: false,
        keys: vec![
            EveJwtKey::RS256 {
                e: "AQAB".to_string(),
                kid: "JWT-Signature-Key-1".to_string(),
                kty: "RSA".to_string(),
                n: "nehPQ7FQ1YK-leKyIg-aACZaT-DbTL5V1XpXghtLX_bEC-fwxhdE_4yQKDF6cA-V4c-5kh8wMZbfYw5xxgM9DynhHGNLbZpmfmbQZ3X-ZUwpZ4ARuYKKM8vGXaUxOH7rKjF4SWjbaPZR8wZO9TcLRUvuRjBppP_8JM3DTCfs0nD-r3J_5uUvXWGR_bFQ1s-Ucn3_QxQqR_D5wDJRx5ZiKIxja2IZg4PGNp5WdBBY-KwmyMxzYQvKWLlcjv5FRJVupKWcJgJ0uLgqBYLiKJFja3RSlQnK1ph__gIEFMnjXEQJhEQb5JdV9H8JaP_MxQi2-8SdCG4ZpAQwTZoIgQ".to_string(),
                r#use: "sig".to_string(),
            },
            EveJwtKey::ES256 {
                crv: "P-256".to_string(),
                kid: "JWT-Signature-Key-2".to_string(),
                kty: "EC".to_string(),
                r#use: "sig".to_string(),
                x: "ITcDYJ8WVpDO4QtZ169xXUt7GB1Y6-oMKIwJ3nK1tFU".to_string(),
                y: "ZAJr0f4V2Eu7xBgLMgQBdJ2DZ2mp8JykOhX4XgU_UEY".to_string(),
            },
        ],
    }
}

/// Helper function to create mock JWT keys for testing cache refresh
pub fn create_mock_jwt_keys_alternative() -> EveJwtKeys {
    EveJwtKeys {
        skip_unresolved_json_web_keys: true, // Different from the other mock
        keys: vec![
            EveJwtKey::RS256 {
                e: "AQAB".to_string(),
                kid: "JWT-Signature-Key-3".to_string(), // Different kid
                kty: "RSA".to_string(),
                n: "vX1oo9bD4DQBZa4qP0W0HZK2sNM3JRj3n5UZ1qJ9WqFpOvG43UqKVeSoK5jIIZ9OyTQCJFN3WUuGfFWuXIQUQ-YQgNzBu9NrGfSqZjgS5j3xgxWTQ2aaCQC8CyNDwIPvHFsB3nI9SPjVJxwoKaceTLMV98_5IMydZYpDXWv8qahA1wIbjrwFkDm6uKxRkUwRWjOcK3GVtYjBnmrcaQK5_6gbfBgOt2kkE3QRFNZdUSkvU6M0DTQj4JpJ8zUFRB0Z3HVarJ_LXzlRkXAjggItTYINijMNzcROLfLdQA9U0q-JiU8EhRkD9LJXSQgQXE5hXRQwGjSH_QJWIoQcdQ".to_string(), // Different n
                r#use: "sig".to_string(),
            },
            EveJwtKey::ES256 {
                crv: "P-256".to_string(),
                kid: "JWT-Signature-Key-4".to_string(), // Different kid
                kty: "EC".to_string(),
                r#use: "sig".to_string(),
                x: "F0KvrJXqZJ8avKyHx3EZpGbIHwYZPiBgmX0oRbbO9_c".to_string(), // Different x
                y: "NluE_RjRJbxNCFnG9oqB_3KJq0bLiQJGlXrfEiT6Oig".to_string(), // Different y
            },
        ],
    }
}
