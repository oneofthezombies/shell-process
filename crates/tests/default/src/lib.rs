#[cfg(test)]
mod tests {
    use sheller::Sheller;

    #[test]
    #[cfg(windows)]
    fn env_shell_windows() {
        use std::{env, ffi::OsStr, process};

        let default_shell = env::var("COMSPEC").unwrap();
        assert_eq!(default_shell, "C:\\WINDOWS\\system32\\cmd.exe");
    }

    #[test]
    #[cfg(windows)]
    fn default_windows() {
        use std::{env, ffi::OsStr, process};

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
        use std::{ffi::OsStr, path::Path};

        let sheller = Sheller::new();
        let command = sheller.build();
        let program = command.get_program().to_str().unwrap();
        let file_name = Path::new(program).file_name().unwrap().to_str().unwrap();
        assert!(file_name.ends_with("sh"));
        let args = command.get_args().collect::<Vec<&OsStr>>();
        assert_eq!(args, vec!["-c"]);
    }
}
