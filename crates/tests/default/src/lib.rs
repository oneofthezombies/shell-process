#[cfg(test)]
mod tests {
    use sheller::Sheller;

    #[test]
    #[cfg(windows)]
    fn default_windows() {
        use std::{ffi::OsStr, path::Path};

        let sheller = Sheller::new();
        let command = sheller.build();
        let program = command.get_program().to_str().unwrap();
        let file_name = Path::new(program).file_name().unwrap().to_str().unwrap();
        assert_eq!(file_name, "cmd.exe");
        let args = command.get_args().collect::<Vec<&OsStr>>();
        assert_eq!(args, vec!["/D", "/S", "/C"]);
    }

    #[test]
    #[cfg(unix)]
    fn default_unix() {
        use std::{ffi::OsStr, path::Path};

        let sheller = Sheller::new("echo hello");
        let command = sheller.build();
        let program = command.get_program().to_str().unwrap();
        let file_name = Path::new(program).file_name().unwrap().to_str().unwrap();
        assert!(file_name.ends_with("sh"));
        let args = command.get_args().collect::<Vec<&OsStr>>();
        assert_eq!(args, vec!["-c", "echo hello"]);
    }
}
