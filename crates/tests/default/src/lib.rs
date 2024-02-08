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
        assert_eq!(Config::default().prefix, "🐚 $ ");
    }

    #[test]
    fn config_custom() {
        assert_eq!(
            Config {
                prefix: "🦀 $ ".to_string(),
                ..Default::default()
            }
            .prefix,
            "🦀 $ "
        );
    }

    #[test]
    fn run() {
        Sheller::new("echo hello").run();
    }

    #[test]
    fn try_run() {
        Sheller::new("echo hello").try_run().unwrap();
    }

    #[test]
    fn run_with_config() {
        Sheller::new("echo hello").run_with_config(&Config::default());
    }

    #[test]
    fn run_with_config_custom() {
        let config = Config {
            prefix: "🦀 $ ".to_string(),
            ..Default::default()
        };
        Sheller::new("echo hello").run_with_config(&config);
    }

    #[test]
    fn run_with_config_custom_without_let() {
        Sheller::new("echo hello").run_with_config(&Config {
            prefix: "🦀 $ ".to_string(),
            ..Default::default()
        });
    }

    #[test]
    fn try_run_with_config() {
        Sheller::new("echo hello")
            .try_run_with_config(&Config::default())
            .unwrap();
    }

    #[test]
    fn try_run_with_config_custom() {
        Sheller::new("echo hello")
            .try_run_with_config(&Config {
                prefix: "🦀 $ ".to_string(),
                ..Default::default()
            })
            .unwrap();
    }

    #[test]
    fn command_ext_run() {
        let mut command = std::process::Command::new("echo");
        command.arg("hello").run();
    }

    #[test]
    fn command_ext_try_run() {
        let mut command = std::process::Command::new("echo");
        command.arg("hello").try_run().unwrap();
    }

    #[test]
    fn command_ext_run_with_config() {
        let mut command = std::process::Command::new("echo");
        let config = Config::default();
        command.arg("hello").run_with_config(&config);
    }

    #[test]
    fn command_ext_try_run_with_config() {
        let mut command = std::process::Command::new("echo");
        let config = Config::default();
        command.arg("hello").try_run_with_config(&config).unwrap();
    }

    #[test]
    fn command_ext_run_with_config_custom() {
        let mut command = std::process::Command::new("echo");
        let config = Config {
            prefix: "🦀 $ ".to_string(),
            ..Default::default()
        };
        command.arg("hello").run_with_config(&config);
    }

    #[test]
    fn command_ext_try_run_with_config_custom() {
        let mut command = std::process::Command::new("echo");
        let config = Config {
            prefix: "🦀 $ ".to_string(),
            ..Default::default()
        };
        command.arg("hello").try_run_with_config(&config).unwrap();
    }

    #[test]
    fn build_run() {
        let mut command = Sheller::new("echo hello").build();
        command.run();
    }

    #[test]
    fn build_try_run() {
        let mut command = Sheller::new("echo hello").build();
        command.try_run().unwrap();
    }

    #[test]
    fn build_run_with_config() {
        let mut command = Sheller::new("echo hello").build();
        let config = Config::default();
        command.run_with_config(&config);
    }

    #[test]
    fn build_try_run_with_config() {
        let mut command = Sheller::new("echo hello").build();
        let config = Config::default();
        command.try_run_with_config(&config).unwrap();
    }

    #[test]
    fn build_run_with_config_custom() {
        let mut command = Sheller::new("echo hello").build();
        let config = Config {
            prefix: "🦀 $ ".to_string(),
            ..Default::default()
        };
        command.run_with_config(&config);
    }

    #[test]
    fn build_try_run_with_config_custom() {
        let mut command = Sheller::new("echo hello").build();
        let config = Config {
            prefix: "🦀 $ ".to_string(),
            ..Default::default()
        };
        command.try_run_with_config(&config).unwrap();
    }
}
