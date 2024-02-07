#[cfg(test)]
mod tests {
    use sheller::Sheller;
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
    fn call_sh() {
        use sheller::debug::sh;
        sh("echo hello");
    }
}
