// Common macro for handling the shared implementation parts
macro_rules! esi_common_impl {
    (
        $label:expr,
        $url:expr,
        $api_call:expr,
        ($($param_name:ident),*)
        $(, $handler:expr)?
    ) => {
        {
            endpoint_debug_log!($label, $url $(, ($param_name))*);

            let start_time = std::time::Instant::now();

            let result = $api_call.await;

            let elapsed = start_time.elapsed();
            match result {
                Ok(response) => {
                    endpoint_info_log!($label, elapsed.as_millis() $(, ($param_name))*);

                    $(
                        let response = ($handler)(response);
                    )?

                    Ok(response)
                }
                Err(err) => {
                    endpoint_error_log!($label, elapsed.as_millis(), err $(, ($param_name))*);

                    Err(err.into())
                }
            }
        }
    };
}

// Macro for defining public & authenticated ESI endpoints
macro_rules! define_endpoint {
    // GET endpoint macro
    (
        $(#[$attr:meta])*
        pub_get $fn_name:ident(
            $(&self,)?
            $($param_name:ident: $param_type:ty),* $(,)?
        ) -> Result<$return_type:ty, Error>
        url = $url:expr;
        label = $label:expr;
        $(response_handler = $handler:expr;)?
    ) => {
        $(#[$attr])*
        pub async fn $fn_name(&self, $($param_name: $param_type),*) -> Result<$return_type, Error> {
            let url = format!($url, self.client.inner.esi_url, $($param_name),*);

            let esi = self.client.esi();
            let api_call = esi
                .get_from_public_esi::<$return_type>(&url);

            esi_common_impl!($label, url, api_call, ($($param_name),*) $(, $handler)?)
        }
    };

    // POST endpoint macro
    (
        $(#[$attr:meta])*
        pub_post $fn_name:ident(
            $(&self,)?
            body: $body_type:ty,
            $($param_name:ident: $param_type:ty),* $(,)?
        ) -> Result<$return_type:ty, Error>
        url = $url:expr;
        label = $label:expr;
        $(response_handler = $handler:expr;)?
    ) => {
        $(#[$attr])*
        pub async fn $fn_name(&self, body: $body_type, $($param_name: $param_type),*) -> Result<$return_type, Error> {
            let url = format!($url, self.client.inner.esi_url, $($param_name),*);

            let esi = self.client.esi();
            let api_call = esi
                .post_to_public_esi::<$return_type, $body_type>(&url, &body);

            esi_common_impl!($label, url, api_call, ($($param_name),*) $(, $handler)?)
        }
    };

    // Authenticated GET endpoint macro
    (
        $(#[$attr:meta])*
        auth_get $fn_name:ident(
            $(&self,)?
            access_token: &str,
            $($param_name:ident: $param_type:ty),* $(,)?
        ) -> Result<$return_type:ty, Error>
        url = $url:expr;
        label = $label:expr;
        required_scopes = $required_scopes:expr;
        $(response_handler = $handler:expr;)?
    ) => {
        $(#[$attr])*
        pub async fn $fn_name(&self, access_token: &str, $($param_name: $param_type),*) -> Result<$return_type, Error> {
            let url = format!($url, self.client.inner.esi_url, $($param_name),*);

            let esi = self.client.esi();
            let api_call = esi
                .get_from_authenticated_esi::<$return_type>(&url, &access_token, $required_scopes);

            esi_common_impl!($label, url, api_call, ($($param_name),*) $(, $handler)?)
        }
    };
}
