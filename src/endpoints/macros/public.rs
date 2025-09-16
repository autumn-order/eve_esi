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

            let api_call = self
                .client
                .esi()
                .get_from_public_esi::<$return_type>(&url)
                .await;

            esi_common_impl!($label, url, api_call, $($param_name),* $(, $handler)?)
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

            let api_call = self
                .client
                .esi()
                .post_to_public_esi::<$return_type, $body_type>(&url, &body)
                .await;

            esi_common_impl!($label, url, api_call, $($param_name),* $(, $handler)?)
        }
    };
}
