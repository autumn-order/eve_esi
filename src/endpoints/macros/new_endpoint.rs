/// Macro for constructing an ESI endpoint request URL
macro_rules! build_endpoint_url {
    // No query params
    ($self_ident:ident, $fmt:expr, ($($path:ident),* $(,)?)) => {{
        url::Url::parse(&format!($fmt, $self_ident.client.inner.esi_url, $($path),* ))?
    }};

    // One or more query params
    ($self_ident:ident, $fmt:expr, ($($path:ident),* $(,)?), ($($query:ident),+ $(,)?)) => {{
        let mut url = url::Url::parse(&format!($fmt, $self_ident.client.inner.esi_url, $($path),* ))?;

        let mut ser = url::form_urlencoded::Serializer::new(String::new());

        $(
            let val = serde_json::to_string(&$query).map_err(|e| Error::from(e))?;
            ser.append_pair(stringify!($query), &val);
        )*

        url.set_query(Some(&ser.finish()));

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
        let request = EsiRequest::<$return_type>::new($url.as_str())
            .with_method($method)
            .with_body_json(serde_json::to_value(&$body_name)?);
        Ok(request)
    }};

    // Public endpoint without body
    (
        url = $url:expr,
        method = $method:expr,
        return_type = $return_type:ty
    ) => {{
        let request = EsiRequest::<$return_type>::new($url.as_str()).with_method($method);
        Ok(request)
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
        let request = EsiRequest::<$return_type>::new($url.as_str())
            .with_method($method)
            .with_access_token($access_token)
            .with_required_scopes($required_scopes)
            .with_body_json(serde_json::to_value(&$body_name)?);
        Ok(request)
    }};

    // Authenticated endpoint without body
    (
        url = $url:expr,
        method = $method:expr,
        return_type = $return_type:ty,
        access_token = $access_token:ident,
        required_scopes = $required_scopes:expr
    ) => {{
        let request = EsiRequest::<$return_type>::new($url.as_str())
            .with_method($method)
            .with_access_token($access_token)
            .with_required_scopes($required_scopes);
        Ok(request)
    }};
}

/// Macro for defining ESI endpoints that return EsiRequest builders
///
/// This macro generates endpoint methods that return `EsiRequest<T>` structs,
/// allowing users to add custom headers before sending the request.
///
/// For an overview of methods and a usage example, please see the [module-level documentation](super)
macro_rules! define_esi_endpoint {
    // Public endpoint (no authentication)
    (
        $(#[$attr:meta])*
        pub fn $fn_name:ident(
            $(&self,)?
            $($path_name:ident: $path_ty:ty),* $(,)?
            $(; $($query_name:ident: $query_ty:ty),* $(,)?)?
        ) -> Result<EsiRequest<$return_type:ty>, Error>
        method = $method:expr;
        url = $url:expr;
        $(body = $body_name:ident: $body_type:ty;)?
    ) => {
        $(#[$attr])*
        pub fn $fn_name(&self, $($path_name: $path_ty),* $(, $($query_name: $query_ty),* )? $( , $body_name: $body_type )? ) -> Result<EsiRequest<$return_type>, Error> {
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
        ) -> Result<EsiRequest<$return_type:ty>, Error>
        method = $method:expr;
        url = $url:expr;
        required_scopes = $required_scopes:expr;
        $(body = $body_name:ident: $body_type:ty;)?
    ) => {
        $(#[$attr])*
        pub fn $fn_name(&self, access_token: &str, $($path_name: $path_ty),* $(, $($query_name: $query_ty),* )? $( , $body_name: $body_type )? ) -> Result<EsiRequest<$return_type>, Error> {
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
