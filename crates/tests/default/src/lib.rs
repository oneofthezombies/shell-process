#[cfg(test)]
mod tests {
    use sheller::Sheller;
    use std::{env, ffi::OsStr, process};

    #[test]
    #[cfg(windows)]
    fn env_shell_windows() {
        let default_shell = env::var("COMSPEC").unwrap();
        assert_eq!(default_shell, "C:\\WINDOWS\\system32\\cmd.exe");
    }

    #[test]
    #[cfg(windows)]
    fn default_windows() {
        let sheller = Sheller::new();
        let command = sheller.build();
        let mut expected_command = process::Command::new(env::var("COMSPEC").unwrap());
        expected_command.args(&["/D", "/S", "/C"]);
        assert_eq!(command.get_program(), expected_command.get_program());
        assert_eq!(
            command.get_args().collect::<Vec<&OsStr>>(),
            expected_command.get_args().collect::<Vec<&OsStr>>()
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn env_shell_linux() {
        let default_shell = env::var("SHELL").unwrap();
        assert_eq!(default_shell, "/bin/sh");
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn env_shell_macos() {
        let default_shell = env::var("SHELL").unwrap();
        let expected_shell = match env::var("GITHUB_ACTIONS") {
            Ok(_) => "/bin/bash",
            Err(_) => "/bin/zsh",
        };
        assert_eq!(default_shell, expected_shell);
    }

    #[test]
    #[cfg(unix)]
    fn default_unix() {
        let sheller = Sheller::new();
        let command = sheller.build();
        let mut expected_command = process::Command::new(env::var("SHELL").unwrap());
        expected_command.arg("-c");
        assert_eq!(command.get_program(), expected_command.get_program());
        assert_eq!(
            command.get_args().collect::<Vec<&OsStr>>(),
            expected_command.get_args().collect::<Vec<&OsStr>>()
        );
    }
}
