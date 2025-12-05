/// Macro for constructing an ESI endpoint request URL
macro_rules! build_endpoint_url {
    // No query params
    ($self_ident:ident, $fmt:expr, ($($path:ident),* $(,)?)) => {{
        format!($fmt, $self_ident.client.inner.esi_url, $($path),* )
    }};

    // One or more query params
    ($self_ident:ident, $fmt:expr, ($($path:ident),* $(,)?), ($($query:ident),+ $(,)?)) => {{
        let mut url = format!($fmt, $self_ident.client.inner.esi_url, $($path),* );

        let mut ser = url::form_urlencoded::Serializer::new(String::new());

        $(
            // Serialize to JSON and add to query string
            // If serialization fails, we use a placeholder value
            // Real errors will be caught when the request is sent
            let val = serde_json::to_string(&$query).unwrap_or_else(|_| String::from("null"));
            ser.append_pair(stringify!($query), &val);
        )*

        let query_string = ser.finish();
        if !query_string.is_empty() {
            url.push('?');
            url.push_str(&query_string);
        }

        url
    }};
}

/// Internal macro for building the EsiRequest with common logic
macro_rules! build_esi_request_internal {
    // Public endpoint with body
    (
        url = $url:expr,
        method = $method:expr,
        return_type = $return_type:ty,
        body = $body_name:ident
    ) => {{
        // Serialize body - if it fails, store null and let send() handle the error
        let body_value = serde_json::to_value(&$body_name).unwrap_or(serde_json::Value::Null);
        EsiRequest::<$return_type>::new($url)
            .with_method($method)
            .with_body_json(body_value)
    }};

    // Public endpoint without body
    (
        url = $url:expr,
        method = $method:expr,
        return_type = $return_type:ty
    ) => {{
        EsiRequest::<$return_type>::new($url).with_method($method)
    }};

    // Authenticated endpoint with body
    (
        url = $url:expr,
        method = $method:expr,
        return_type = $return_type:ty,
        body = $body_name:ident,
        access_token = $access_token:ident,
        required_scopes = $required_scopes:expr
    ) => {{
        // Serialize body - if it fails, store null and let send() handle the error
        let body_value = serde_json::to_value(&$body_name).unwrap_or(serde_json::Value::Null);
        EsiRequest::<$return_type>::new($url)
            .with_method($method)
            .with_access_token($access_token)
            .with_required_scopes($required_scopes)
            .with_body_json(body_value)
    }};

    // Authenticated endpoint without body
    (
        url = $url:expr,
        method = $method:expr,
        return_type = $return_type:ty,
        access_token = $access_token:ident,
        required_scopes = $required_scopes:expr
    ) => {{
        EsiRequest::<$return_type>::new($url)
            .with_method($method)
            .with_access_token($access_token)
            .with_required_scopes($required_scopes)
    }};
}

/// Macro for defining ESI endpoints that return EsiRequest builders
///
/// This macro generates endpoint methods that return `EsiRequest<T>` structs,
/// allowing users to add custom headers before sending the request.
///
/// For an overview of methods and a usage example, please see the [module-level documentation](super)
macro_rules! define_esi_endpoint {
    // Public endpoint with body but no path parameters (e.g., character_affiliation)
    (
        $(#[$attr:meta])*
        pub fn $fn_name:ident(
            $(&self,)?
        ) -> EsiRequest<$return_type:ty>
        method = $method:expr;
        url = $url:expr;
        body = $body_name:ident: $body_type:ty;
    ) => {
        $(#[$attr])*
        pub fn $fn_name(&self, $body_name: $body_type) -> EsiRequest<$return_type> {
            let url = format!($url, self.client.inner.esi_url);

            build_esi_request_internal!(
                url = url,
                method = $method,
                return_type = $return_type,
                body = $body_name
            )
        }
    };

    // Public endpoint (no authentication)
    (
        $(#[$attr:meta])*
        pub fn $fn_name:ident(
            $(&self,)?
            $($path_name:ident: $path_ty:ty),* $(,)?
            $(; $($query_name:ident: $query_ty:ty),* $(,)?)?
        ) -> EsiRequest<$return_type:ty>
        method = $method:expr;
        url = $url:expr;
        $(body = $body_name:ident: $body_type:ty;)?
    ) => {
        $(#[$attr])*
        pub fn $fn_name(&self, $($path_name: $path_ty),* $(, $($query_name: $query_ty),* )? $( , $body_name: $body_type )? ) -> EsiRequest<$return_type> {
            let url = build_endpoint_url!(self, $url, ($($path_name),*) $(, ($($query_name),*) )? );

            build_esi_request_internal!(
                url = url,
                method = $method,
                return_type = $return_type
                $(, body = $body_name)?
            )
        }
    };

    // Authenticated endpoint
    (
        $(#[$attr:meta])*
        auth fn $fn_name:ident(
            $(&self,)?
            access_token: &str,
            $($path_name:ident: $path_ty:ty),* $(,)?
            $(; $($query_name:ident: $query_ty:ty),* $(,)?)?
        ) -> EsiRequest<$return_type:ty>
        method = $method:expr;
        url = $url:expr;
        required_scopes = $required_scopes:expr;
        $(body = $body_name:ident: $body_type:ty;)?
    ) => {
        $(#[$attr])*
        pub fn $fn_name(&self, access_token: &str, $($path_name: $path_ty),* $(, $($query_name: $query_ty),* )? $( , $body_name: $body_type )? ) -> EsiRequest<$return_type> {
            let url = build_endpoint_url!(self, $url, ($($path_name),*) $(, ($($query_name),*) )? );

            build_esi_request_internal!(
                url = url,
                method = $method,
                return_type = $return_type
                $(, body = $body_name)?
                , access_token = access_token
                , required_scopes = $required_scopes
            )
        }
    };
}
