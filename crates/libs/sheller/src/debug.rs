use crate::Sheller;
use std::process;

/// Run a shell command.
///
/// # Panics
/// Panics if the command fails.
pub fn sh(script: &str) {
    let mut command = Sheller::new(script).build();
    command
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped());
    let status = command.status().unwrap();
    assert!(status.success());
}
