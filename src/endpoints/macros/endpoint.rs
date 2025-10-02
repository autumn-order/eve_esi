// Common macro for handling the shared implementation parts
macro_rules! esi_common_impl {
    (
        $label:expr,
        $url:expr,
        $api_call:expr,
        ($($param_name:ident),*)
    ) => {
        {
            endpoint_debug_log!($label, $url $(, ($param_name))*);

            let start_time = std::time::Instant::now();

            let result = $api_call.await;

            let elapsed = start_time.elapsed();
            match result {
                Ok(response) => {
                    endpoint_info_log!($label, elapsed.as_millis() $(, ($param_name))*);

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
    ) => {
        $(#[$attr])*
        pub async fn $fn_name(&self, $($param_name: $param_type),*) -> Result<$return_type, Error> {
            let url = format!($url, self.client.inner.esi_url, $($param_name),*);

            let esi = self.client.esi();
            let api_call = esi
                .get_from_public_esi::<$return_type>(&url);

            esi_common_impl!($label, url, api_call, ($($param_name),*))
        }
    };

    // POST endpoint macro
    (
        $(#[$attr:meta])*
        pub_post $fn_name:ident(
            $(&self,)?
            $body_name:ident: $body_type:ty,
            $($path_name:ident: $path_ty:ty),* $(,)?
            $(; $($query_name:ident: $query_ty:ty),* $(,)?)?
        ) -> Result<$return_type:ty, Error>
        url = $url:expr;
        label = $label:expr;
    ) => {
        $(#[$attr])*
        pub async fn $fn_name(&self, $body_name: $body_type, $(, $($path_name: $path_ty),* )? $(, $($query_name: $query_ty),* )? ) -> Result<$return_type, Error> {
            // Add URL path params
            let url = url::Url::parse(&format!(
                $url, self.client.inner.esi_url, $($path_name),*
            ))?;

            // Add query params
            $(
                let mut url = url;

                {
                    let mut ser = url::form_urlencoded::Serializer::new(String::new());

                        $(
                            let val = serde_json::to_string(&$query_name).map_err(|e| Error::from(e))?;

                            ser.append_pair(stringify!($query_name), &val);
                        )*

                    let q = ser.finish();
                    if !q.is_empty() {
                        url.set_query(Some(&q));
                    }
                }
            )?

            let esi = self.client.esi();
            let api_call = esi
                .post_to_public_esi::<$return_type, $body_type>(url.as_str(), &$body_name);

            esi_common_impl!($label, url, api_call, ($($path_name),*))
        }
    };

    // Authenticated GET endpoint macro
    (
        $(#[$attr:meta])*
        auth_get $fn_name:ident(
            $(&self,)?
            access_token: &str,
            $($path_name:ident: $path_type:ty),* $(,)? ;
            $($query_name:ident: $query_type:ty),* $(,)?
        ) -> Result<$return_type:ty, Error>
        url = $url:expr;
        label = $label:expr;
        required_scopes = $required_scopes:expr;
    ) => {
        $(#[$attr])*
        pub async fn $fn_name(&self, access_token: &str, $($path_name: $path_type),*, $($query_name: $query_type),*) -> Result<$return_type, Error> {
            // Add URL path params
            let mut url = url::Url::parse(&format!(
                $url, self.client.inner.esi_url, $($path_name),*
            ))?;

            // Add query params
            {
                let mut ser = url::form_urlencoded::Serializer::new(String::new());

                $(
                    let val = serde_json::to_string(&$query_name).map_err(|e| Error::from(e))?;

                    ser.append_pair(stringify!($query_name), &val);
                )*

                let q = ser.finish();
                if !q.is_empty() {
                    url.set_query(Some(&q));
                }
            }

            let esi = self.client.esi();
            let api_call = esi
                .get_from_authenticated_esi::<$return_type>(url.as_str(), access_token, $required_scopes);

            esi_common_impl!($label, url, api_call, ($($path_name),*))
        }
    };

    // Authenticated POST endpoint macro
    (
        $(#[$attr:meta])*
        auth_post $fn_name:ident(
            $(&self,)?
            access_token: &str,
            $body_name:ident: $body_type:ty,
            $($path_name:ident: $path_type:ty),* $(,)? ;
            $($query_name:ident: $query_type:ty),* $(,)?
        ) -> Result<$return_type:ty, Error>
        url = $url:expr;
        label = $label:expr;
        required_scopes = $required_scopes:expr;
    ) => {
        $(#[$attr])*
        pub async fn $fn_name(&self, access_token: &str, $body_name: $body_type, $($path_name: $path_type),*, $($query_name: $query_type),*) -> Result<$return_type, Error> {
            // Add URL path params
            let mut url = url::Url::parse(&format!(
                $url, self.client.inner.esi_url, $($path_name),*
            ))?;

            // Add query params
            {
                let mut ser = url::form_urlencoded::Serializer::new(String::new());

                $(
                    let val = serde_json::to_string(&$query_name).map_err(|e| Error::from(e))?;

                    ser.append_pair(stringify!($query_name), &val);
                )*

                let q = ser.finish();
                if !q.is_empty() {
                    url.set_query(Some(&q));
                }
            }

            let esi = self.client.esi();
            let api_call = esi
                .post_to_authenticated_esi::<$return_type, $body_type>(url.as_str(), &$body_name, access_token, $required_scopes);

            esi_common_impl!($label, url, api_call, ($($path_name),*))
        }
    };

    // Authenticated PUT endpoint macro
    (
        $(#[$attr:meta])*
        auth_put $fn_name:ident(
            $(&self,)?
            access_token: &str,
            $body_name:ident: $body_type:ty,
            $($path_name:ident: $path_type:ty),* $(,)? ;
            $($query_name:ident: $query_type:ty),* $(,)?
        ) -> Result<$return_type:ty, Error>
        url = $url:expr;
        label = $label:expr;
        required_scopes = $required_scopes:expr;
    ) => {
        $(#[$attr])*
        pub async fn $fn_name(&self, access_token: &str, $body_name: $body_type, $($path_name: $path_type),*, $($query_name: $query_type),*) -> Result<$return_type, Error> {
            // Add URL path params
            let mut url = url::Url::parse(&format!(
                $url, self.client.inner.esi_url, $($path_name),*
            ))?;

            // Add query params
            {
                let mut ser = url::form_urlencoded::Serializer::new(String::new());

                $(
                    let val = serde_json::to_string(&$query_name).map_err(|e| Error::from(e))?;

                    ser.append_pair(stringify!($query_name), &val);
                )*

                let q = ser.finish();
                if !q.is_empty() {
                    url.set_query(Some(&q));
                }
            }

            let esi = self.client.esi();
            let api_call = esi
                .put_to_authenticated_esi::<$return_type, $body_type>(url.as_str(), &$body_name, access_token, $required_scopes);

            esi_common_impl!($label, url, api_call, ($($path_name),*))
        }
    };


    // Authenticated DELETE endpoint macro
    (
        $(#[$attr:meta])*
        auth_delete $fn_name:ident(
            $(&self,)?
            access_token: &str,
            $($path_name:ident: $path_type:ty),* $(,)? ;
            $($query_name:ident: $query_type:ty),* $(,)?
        ) -> Result<$return_type:ty, Error>
        url = $url:expr;
        label = $label:expr;
        required_scopes = $required_scopes:expr;
    ) => {
        $(#[$attr])*
        pub async fn $fn_name(&self, access_token: &str, $($path_name: $path_type),*, $($query_name: $query_type),*) -> Result<$return_type, Error> {
            // Add URL path params
            let mut url = url::Url::parse(&format!(
                $url, self.client.inner.esi_url, $($path_name),*
            ))?;

            // Add query params
            {
                let mut ser = url::form_urlencoded::Serializer::new(String::new());

                $(
                    let val = serde_json::to_string(&$query_name).map_err(|e| Error::from(e))?;

                    ser.append_pair(stringify!($query_name), &val);
                )*

                let q = ser.finish();
                if !q.is_empty() {
                    url.set_query(Some(&q));
                }
            }

            let esi = self.client.esi();
            let api_call = esi
                .delete_from_authenticated_esi::<$return_type>(url.as_str(), &access_token, $required_scopes);

            esi_common_impl!($label, url, api_call, ($($path_name),*))
        }
    };
}
