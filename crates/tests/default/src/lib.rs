#[cfg(test)]
mod tests {
    use sheller::Sheller;
    use std::ffi::OsStr;
    use std::process;

    #[test]
    #[cfg(windows)]
    fn default_windows() {
        let sheller = Sheller::new();
        let command = sheller.build();
        let mut expected_command = process::Command::new("C:\\WINDOWS\\system32\\cmd.exe");
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
        let mut expected_command = process::Command::new("/bin/sh");
        expected_command.arg("-c");
        assert_eq!(command.get_program(), expected_command.get_program());
        assert_eq!(
            command.get_args().collect::<Vec<&OsStr>>(),
            expected_command.get_args().collect::<Vec<&OsStr>>()
        );
    }
}
