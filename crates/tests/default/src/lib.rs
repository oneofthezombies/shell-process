#[cfg(test)]
mod tests {
    use sheller::{sh, Sheller};
    use std::{ffi::OsStr, path::Path};

    #[test]
    #[cfg(windows)]
    fn default_windows() {
        let sheller = Sheller::new("echo hello");
        let command = sheller.build();
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
    fn macro_expect() {
        let sheller = Sheller::new("echo hello");
        let command1 = sheller.build();
        let command2 = sh!("echo hello");
        assert_eq!(command1.get_program(), command2.get_program());
        assert_eq!(
            command1.get_args().collect::<Vec<_>>(),
            command2.get_args().collect::<Vec<_>>()
        );
    }
}
