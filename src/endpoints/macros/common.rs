// Common macro for handling the shared implementation parts
macro_rules! esi_common_impl {
    (
        $label:expr,
        $url:expr,
        $api_call:expr,
        $($param_name:ident),*
        $(, $handler:expr)?
    ) => {
        {
            endpoint_debug_log!($label, $url $(, ($param_name))*);

            let start_time = std::time::Instant::now();

            let result = $api_call;

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
