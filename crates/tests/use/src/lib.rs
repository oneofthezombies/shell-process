#[cfg(test)]
mod tests {
    use sheller::{new, CommandExt};
    use std::{ffi::OsStr, path::Path};

    #[test]
    #[cfg(windows)]
    fn default_windows() {
        let command = new!("echo hello").build();
        let program = command.get_program().to_str().unwrap();
        let file_name = Path::new(program).file_name().unwrap().to_str().unwrap();
        assert_eq!(file_name, "cmd.exe");
        let args = command.get_args().collect::<Vec<&OsStr>>();
        assert_eq!(args, vec!["/D", "/S", "/C", "echo hello"]);
    }

    #[test]
    #[cfg(unix)]
    fn default_unix() {
        let sheller = new!("echo hello");
        let command = sheller.build();
        let program = command.get_program().to_str().unwrap();
        let file_name = Path::new(program).file_name().unwrap().to_str().unwrap();
        assert!(file_name.ends_with("sh"));
        let args = command.get_args().collect::<Vec<&OsStr>>();
        assert_eq!(args, vec!["-c", "echo hello"]);
    }

    #[test]
    fn run() {
        new!("echo hello").run();
    }

    #[test]
    fn try_run() {
        new!("echo hello").try_run().unwrap();
    }

    #[test]
    #[cfg(unix)]
    fn command_ext_run() {
        let mut command = std::process::Command::new("echo");
        command.arg("hello").run();
    }

    #[test]
    #[cfg(unix)]
    fn command_ext_try_run() {
        let mut command = std::process::Command::new("echo");
        command.arg("hello").try_run().unwrap();
    }

    #[test]
    #[cfg(windows)]
    fn command_ext_run() {
        let mut command = std::process::Command::new("cmd.exe");
        command.args(["/D", "/S", "/C", "echo hello"]).run();
    }

    #[test]
    #[cfg(windows)]
    fn command_ext_try_run() {
        let mut command = std::process::Command::new("cmd.exe");
        command
            .args(["/D", "/S", "/C", "echo hello"])
            .try_run()
            .unwrap();
    }

    #[test]
    fn build_run() {
        let mut command = new!("echo hello").build();
        command.run();
    }

    #[test]
    fn build_try_run() {
        let mut command = new!("echo hello").build();
        command.try_run().unwrap();
    }

    #[test]
    fn build_pipe() {
        let output = new!("echo hello")
            .build()
            .stdout(std::process::Stdio::piped())
            .output()
            .unwrap();
        let eol = if cfg!(windows) { "\r\n" } else { "\n" };
        assert_eq!(output.stdout, format!("hello{}", eol).as_bytes());
    }
}
