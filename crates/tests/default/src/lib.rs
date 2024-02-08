#[cfg(test)]
mod tests {
    use sheller::{CommandExt, Config, Sheller};
    use std::{ffi::OsStr, path::Path};

    #[test]
    #[cfg(windows)]
    fn default_windows() {
        let command = Sheller::new("echo hello").build();
        let program = command.get_program().to_str().unwrap();
        let file_name = Path::new(program).file_name().unwrap().to_str().unwrap();
        assert_eq!(file_name, "cmd.exe");
        let args = command.get_args().collect::<Vec<&OsStr>>();
        assert_eq!(args, vec!["/D", "/S", "/C", "echo hello"]);
    }

    #[test]
    #[cfg(unix)]
    fn default_unix() {
        let sheller = Sheller::new("echo hello");
        let command = sheller.build();
        let program = command.get_program().to_str().unwrap();
        let file_name = Path::new(program).file_name().unwrap().to_str().unwrap();
        assert!(file_name.ends_with("sh"));
        let args = command.get_args().collect::<Vec<&OsStr>>();
        assert_eq!(args, vec!["-c", "echo hello"]);
    }

    #[test]
    fn config_default() {
        let config = Config::default();
        assert_eq!(config.prefix, "🐚 $ ");
    }

    #[test]
    fn config_custom() {
        let config = Config {
            prefix: "🦀 $ ".to_string(),
            ..Default::default()
        };
        assert_eq!(config.prefix, "🦀 $ ");
    }

    #[test]
    fn run() {
        let sheller = Sheller::new("echo hello");
        sheller.run();
    }

    #[test]
    fn try_run() {
        let sheller = Sheller::new("echo hello");
        sheller.try_run().unwrap();
    }

    #[test]
    fn run_with_config() {
        let sheller = Sheller::new("echo hello");
        let config = Config::default();
        sheller.run_with_config(&config);
    }

    #[test]
    fn run_with_config_custom() {
        let sheller = Sheller::new("echo hello");
        let config = Config {
            prefix: "🦀 $ ".to_string(),
            ..Default::default()
        };
        sheller.run_with_config(&config);
    }

    #[test]
    fn try_run_with_config() {
        let sheller = Sheller::new("echo hello");
        let config = Config::default();
        sheller.try_run_with_config(&config).unwrap();
    }

    #[test]
    fn try_run_with_config_custom() {
        let sheller = Sheller::new("echo hello");
        let config = Config {
            prefix: "🦀 $ ".to_string(),
            ..Default::default()
        };
        sheller.try_run_with_config(&config).unwrap();
    }

    #[test]
    fn command_ext_run() {
        let mut command = std::process::Command::new("echo");
        command.arg("hello");
        command.run();
    }

    #[test]
    fn command_ext_try_run() {
        let mut command = std::process::Command::new("echo");
        command.arg("hello");
        command.try_run().unwrap();
    }

    #[test]
    fn command_ext_run_with_config() {
        let mut command = std::process::Command::new("echo");
        command.arg("hello");
        let config = Config::default();
        command.run_with_config(&config);
    }

    #[test]
    fn command_ext_try_run_with_config() {
        let mut command = std::process::Command::new("echo");
        command.arg("hello");
        let config = Config::default();
        command.try_run_with_config(&config).unwrap();
    }

    #[test]
    fn command_ext_run_with_config_custom() {
        let mut command = std::process::Command::new("echo");
        command.arg("hello");
        let config = Config {
            prefix: "🦀 $ ".to_string(),
            ..Default::default()
        };
        command.run_with_config(&config);
    }

    #[test]
    fn command_ext_try_run_with_config_custom() {
        let mut command = std::process::Command::new("echo");
        command.arg("hello");
        let config = Config {
            prefix: "🦀 $ ".to_string(),
            ..Default::default()
        };
        command.try_run_with_config(&config).unwrap();
    }

    #[test]
    fn build_run() {
        let sheller = Sheller::new("echo hello");
        sheller.build().run();
    }

    #[test]
    fn build_try_run() {
        let sheller = Sheller::new("echo hello");
        sheller.build().try_run().unwrap();
    }

    #[test]
    fn build_run_with_config() {
        let sheller = Sheller::new("echo hello");
        let config = Config::default();
        sheller.build().run_with_config(&config);
    }

    #[test]
    fn build_try_run_with_config() {
        let sheller = Sheller::new("echo hello");
        let config = Config::default();
        sheller.build().try_run_with_config(&config).unwrap();
    }

    #[test]
    fn build_run_with_config_custom() {
        let sheller = Sheller::new("echo hello");
        let config = Config {
            prefix: "🦀 $ ".to_string(),
            ..Default::default()
        };
        sheller.build().run_with_config(&config);
    }

    #[test]
    fn build_try_run_with_config_custom() {
        let sheller = Sheller::new("echo hello");
        let config = Config {
            prefix: "🦀 $ ".to_string(),
            ..Default::default()
        };
        sheller.build().try_run_with_config(&config).unwrap();
    }
}
