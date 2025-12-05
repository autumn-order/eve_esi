// Macro which creates the success and error response for tests

macro_rules! public_esi_request_success_test {
    (
        $test_name:ident,
        $endpoint:ident,
        $method:ident [$($args:expr),* $(,)?],
        $request_type:expr,
        $url:expr,
        $mock_response:expr
    ) => {
        paste::paste! {
            #[tokio::test]
            pub async fn [<test_ $test_name _success>]() {
                let (esi_client, mut mock_server) = integration_test_setup().await;

                let mock_endpoint = mock_server
                    .mock($request_type, $url)
                    .with_status(200)
                    .with_header("content-type", "application/json")
                    .with_body($mock_response.to_string())
                    .create();

                let endpoints = esi_client.$endpoint();
                let request = endpoints.$method($($args),*);
                let result = request.send().await;

                // Assert 1 request was received for mock endpoint
                mock_endpoint.assert();

                assert!(result.is_ok(), "Error: {:?}", result);
            }
        }
    };
}

macro_rules! public_esi_request_error_test {
    (
        $test_name:ident,
        $endpoint:ident,
        $method:ident [$($args:expr),* $(,)?],
        $request_type:expr,
        $url:expr
    ) => {
        paste::paste! {
            #[tokio::test]
            pub async fn [<test_ $test_name _error>]() {
                let (esi_client, mut mock_server) = integration_test_setup().await;

                let mock_endpoint = mock_server
                    .mock($request_type, $url)
                    .with_status(500)
                    .with_header("content-type", "application/json")
                    .with_body(r#"{"error": "Internal server error"}"#)
                    .create();

                let endpoints = esi_client.$endpoint();
                let request = endpoints.$method($($args),*);
                let result = request.send().await;

                // Assert 1 request was received for mock endpoint
                mock_endpoint.assert();

                assert!(result.is_err());

                assert!(
                    matches!(result, Err(eve_esi::Error::ReqwestError(ref e)) if e.status() == Some(reqwest::StatusCode::INTERNAL_SERVER_ERROR))
                );
            }
        }
    };
}

macro_rules! public_esi_request_test {
    (
        $test_name:ident,
        $endpoint:ident,
        $method:ident [$($args:expr),* $(,)?],
        request_type = $request_type:expr,
        url = $url:expr,
        mock_response = $mock_response:expr
    ) => {
        public_esi_request_success_test! {$test_name, $endpoint, $method[$($args),*], $request_type, $url, $mock_response}
        public_esi_request_error_test! {$test_name, $endpoint, $method[$($args),*], $request_type, $url}
    };
}
