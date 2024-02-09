#[macro_export]
macro_rules! run {
    ($script_fmt:expr) => {
        $crate::Sheller::new(format!($script_fmt)).run();
    };
    ($script_fmt:expr, $($arg:tt)*) => {
        $crate::Sheller::new(format!($script_fmt, $($arg)*)).run();
    };
}

#[macro_export]
macro_rules! try_run {
    ($script_fmt:expr) => {{
        let result = $crate::Sheller::new(format!($script_fmt)).try_run();
        result
    }};
    ($script_fmt:expr, $($arg:tt)*) => {{
        let result = $crate::Sheller::new(format!($script_fmt, $($arg)*)).try_run();
        result
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn run_literal() {
        run!("echo hello");
    }

    #[test]
    fn run_string() {
        let echo_hello = "echo hello".to_string();
        run!("{}", echo_hello);
    }

    #[test]
    fn run_format_positional_parameters() {
        run!("echo {}", "hello");
    }

    #[test]
    fn run_format_named_parameters() {
        run!("echo {hello}", hello = "hello");
    }

    #[test]
    fn run_format_positional_and_named_parameters() {
        run!("echo {} {world}", "hello", world = "world");
    }

    #[test]
    fn run_format_named_parameters_in_scope() {
        let world = "world";
        run!("echo {world}");
    }

    #[test]
    fn run_format_named_parameters_in_scope_and_positional_parameters() {
        let world = "world";
        run!("echo {} {world}", "hello");
    }

    #[test]
    fn run_format_all_parameters() {
        let name = "sheller";
        run!("echo {} {world}, {name}", "hello", world = "world");
    }

    #[test]
    fn try_run_literal() {
        try_run!("echo hello").unwrap();
    }

    #[test]
    fn try_run_string() {
        let echo_hello = "echo hello".to_string();
        try_run!("{}", echo_hello).unwrap();
    }

    #[test]
    fn try_run_format_positional_parameters() {
        try_run!("echo {}", "hello").unwrap();
    }

    #[test]
    fn try_run_format_named_parameters() {
        try_run!("echo {hello}", hello = "hello").unwrap();
    }

    #[test]
    fn try_run_format_positional_and_named_parameters() {
        try_run!("echo {} {world}", "hello", world = "world").unwrap();
    }

    #[test]
    fn try_run_format_named_parameters_in_scope() {
        let world = "world";
        try_run!("echo {world}").unwrap();
    }

    #[test]
    fn try_run_format_named_parameters_in_scope_and_positional_parameters() {
        let world = "world";
        try_run!("echo {} {world}", "hello").unwrap();
    }

    #[test]
    fn try_run_format_all_parameters() {
        let name = "sheller";
        try_run!("echo {} {world}, {name}", "hello", world = "world").unwrap();
    }
}
