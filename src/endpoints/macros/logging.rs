macro_rules! endpoint_debug_log {
    // Case with no parameters
    ($label:expr, $url:expr) => {
        debug!(concat!("Fetching ", $label, " from \"{}\""), $url);
    };

    // Case with parameters (at least one)
    ($label:expr, $url:expr, ($param_name:ident)) => {
        debug!(
            concat!(
                "Fetching ",
                $label,
                " for ",
                stringify!($param_name),
                " {} from \"{}\""
            ),
            $param_name, $url
        );
    };

    // Handle multiple parameters by using the first one
    ($label:expr, $url:expr, ($param_name:ident), $($rest:tt)*) => {
        endpoint_debug_log!($label, $url, ($param_name));
    };
}

macro_rules! endpoint_info_log {
    // Case with no parameters
    ($label:expr, $elapsed:expr) => {
        info!(concat!("Fetching ", $label, " (took {}ms)"), $elapsed);
    };

    // Case with parameters (at least one)
    ($label:expr, $elapsed:expr, ($param_name:ident)) => {
        info!(
            concat!(
                "Successfully fetched ",
                $label,
                " for ",
                stringify!($param_name),
                " {} (took {}ms)"
            ),
            $param_name, $elapsed
        );
    };

    // Handle multiple parameters by using the first one
    ($label:expr, $elapsed:expr, ($param_name:ident), $($rest:tt)*) => {
        endpoint_info_log!($label, $elapsed, ($param_name));
    };
}

macro_rules! endpoint_error_log {
    // Case with no parameters
    ($label:expr, $elapsed:expr, $err:expr) => {
        error!(
            concat!("Failed to fetch ", $label, " after {}ms due to error: {:?}"),
            $elapsed, $err
        );
    };

    // Case with parameters (at least one)
    ($label:expr, $elapsed:expr, $err:expr, ($param_name:ident)) => {
        error!(
            concat!(
                "Failed to fetch ",
                $label,
                " for ",
                stringify!($param_name),
                " {} after {}ms due to error: {:?}"
            ),
            $param_name, $elapsed, $err
        );
    };

    // Handle multiple parameters by using the first one
    ($label:expr, $elapsed:expr, $err:expr, ($param_name:ident), $($rest:tt)*) => {
        endpoint_error_log!($label, $elapsed, $err, ($param_name));
    };
}
