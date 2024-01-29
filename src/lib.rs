use std::os::windows::io::AsRawHandle;
use std::{env, process::Command};

fn is_debug() -> bool {
    env::var("SHELL_PROCESS_DEBUG").is_ok()
}

struct Metadata {
    program: String,
    args: Vec<&'static str>,
}

#[cfg(target_family = "unix")]
fn get_metadata() -> Metadata {
    static FALLBACK_PROGRAM: &str = "/bin/sh";
    let program = env::var("SHELL").unwrap_or_else(|e| {
        if is_debug() {
            eprintln!(
                "SHELL not found: {}. Using fallback program: {}.",
                e, FALLBACK_PROGRAM
            );
        }
        FALLBACK_PROGRAM.to_string()
    });
    let args = vec!["-c"];
    Metadata { program, args }
}

#[cfg(target_family = "windows")]
fn get_metadata() -> Metadata {
    static FALLBACK_PROGRAM: &str = "cmd.exe";
    let program = env::var("COMSPEC").unwrap_or_else(|e| {
        if is_debug() {
            eprintln!(
                "COMSPEC not found: {}. Using fallback program: {}.",
                e, FALLBACK_PROGRAM
            );
        }
        FALLBACK_PROGRAM.to_string()
    });
    let args = vec!["/D", "/S", "/C"];
    Metadata { program, args }
}

pub fn new_shell_command() -> Command {
    let metadata = get_metadata();
    let mut command = Command::new(&metadata.program);
    command.args(&metadata.args);
    command
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_family = "unix")]
    static LINE_ENDING: &str = "\n";

    #[cfg(target_family = "windows")]
    static LINE_ENDING: &str = "\r\n";

    #[test]
    fn echo_hello() {
        let mut command = new_shell_command();
        command.arg("echo hello");
        let output = command.output().unwrap();
        assert_eq!(output.status.success(), true);

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(stdout, format!("hello{}", LINE_ENDING));
    }

    #[test]
    fn tree_kill() {
        let mut command = new_shell_command();
        command.arg("echo hello");
        let child = command.spawn().unwrap();
        let pid = child.id();
        let handle = child.as_raw_handle();
    }
}
