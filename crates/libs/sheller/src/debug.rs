use crate::Sheller;
use std::process;

/// Run a shell script.
///
/// # Panics
/// Panics if the command fails.
pub fn sh(script: &str) {
    let mut command = Sheller::new(script).build();
    assert!(command
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .status()
        .unwrap()
        .success());
}
