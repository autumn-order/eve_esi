// Macro which creates the success and error response for tests

macro_rules! authenticated_esi_request_success_test {
    (
        $test_name:ident,
        $endpoint:ident,
        $method:ident [$($args:expr),* $(,)?],
        $request_type:expr,
        $url:expr,
        $required_scopes:expr,
        $mock_response:expr
    ) => {
        paste::paste! {
            #[tokio::test]
            pub async fn [<test_ $test_name _success>]() {
                let (esi_client, mut mock_server, mock_jwt_key_endpoint) =
                    authenticated_endpoint_test_setup().await;
                let access_token = mock_access_token_with_scopes($required_scopes);

                let mock_endpoint = mock_server
                    .mock($request_type, $url)
                    .with_status(200)
                    .with_header("content-type", "application/json")
                    // Expect access token for authenticated route
                    .with_header("Authorization", &format!("Bearer {}", access_token))
                    .with_body($mock_response.to_string())
                    .create();

                let endpoints = esi_client.$endpoint();
                let request = endpoints.$method(&access_token, $($args),*);
                let result = request.send().await;

                // Assert JWT keys were fetched for token validation prior to request
                mock_jwt_key_endpoint.assert();

                // Assert 1 request & expected access token was received for mock endpoint
                mock_endpoint.assert();

                assert!(result.is_ok(), "Error: {:?}", result);
            }
        }
    };
}

macro_rules! authenticated_esi_request_error_test {
    (
        $test_name:ident,
        $endpoint:ident,
        $method:ident [$($args:expr),* $(,)?],
        $request_type:expr,
        $url:expr,
        $required_scopes:expr
    ) => {
        paste::paste! {
            #[tokio::test]
            pub async fn [<test_ $test_name _error>]() {
                let (esi_client, mut mock_server, mock_jwt_key_endpoint) =
                    authenticated_endpoint_test_setup().await;
                let access_token = mock_access_token_with_scopes($required_scopes);

                let mock_endpoint = mock_server
                    .mock($request_type, $url)
                    .with_status(500)
                    .with_header("content-type", "application/json")
                    // Expect access token for authenticated route
                    .with_header("Authorization", &format!("Bearer {}", access_token))
                    .with_body(r#"{"error": "Internal server error"}"#)
                    .create();

                let endpoints = esi_client.$endpoint();
                let request = endpoints.$method(&access_token, $($args),*);
                let result = request.send().await;

                // Assert JWT keys were fetched for token validation prior to request
                mock_jwt_key_endpoint.assert();

                // Assert 1 request & expected access token was received for mock endpoint
                mock_endpoint.assert();

                assert!(result.is_err());

                assert!(
                    matches!(result, Err(eve_esi::Error::EsiError(ref e)) if e.status == 500)
                );
            }
        }
    };
}

macro_rules! authenticated_esi_request_test {
    (
        $test_name:ident,
        $endpoint:ident,
        $method:ident [$($args:expr),* $(,)?],
        request_type = $request_type:expr,
        url = $url:expr,
        required_scopes = $required_scopes:expr;
        mock_response = $mock_response:expr,
    ) => {
        authenticated_esi_request_success_test! {$test_name, $endpoint, $method[$($args),*], $request_type, $url, $required_scopes, $mock_response}
        authenticated_esi_request_error_test! {$test_name, $endpoint, $method[$($args),*], $request_type, $url, $required_scopes}
    };
}
