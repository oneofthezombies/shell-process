#[cfg(test)]
mod tests {
    use sheller::Sheller;
    use std::{env, ffi::OsStr, process};

    #[cfg(unix)]
    fn get_program() -> String {
        if let Some(_) = env::var("GITHUB_ACTIONS").ok() {
            "/bin/bash".to_string()
        } else {
            env::var("SHELL").unwrap()
        }
    }

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
    #[cfg(unix)]
    fn default_unix() {
        let sheller = Sheller::new();
        let command = sheller.build();
        let mut expected_command = process::Command::new(get_program());
        expected_command.arg("-c");
        assert_eq!(command.get_program(), expected_command.get_program());
        assert_eq!(
            command.get_args().collect::<Vec<&OsStr>>(),
            expected_command.get_args().collect::<Vec<&OsStr>>()
        );
    }
}
