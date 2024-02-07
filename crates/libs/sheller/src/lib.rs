use std::{env, process};
use tracing::debug;

#[cfg(debug_assertions)]
pub mod debug;

#[derive(Debug)]
struct Metadata<'a> {
    env_key: &'a str,
    program: &'a str,
    args: &'a [&'a str],
}

#[cfg(windows)]
static DEFAULT_METADATA: Metadata = Metadata {
    env_key: "COMSPEC",
    program: "cmd.exe",
    args: &["/D", "/S", "/C"],
};

#[cfg(unix)]
static DEFAULT_METADATA: Metadata = Metadata {
    env_key: "SHELL",
    program: "/bin/sh",
    args: &["-c"],
};

fn parse_program() -> String {
    env::var(DEFAULT_METADATA.env_key).unwrap_or_else(|e| {
        debug!(
            default_program = DEFAULT_METADATA.program,
            env_key = DEFAULT_METADATA.env_key,
            error = ?e,
            "Failed to get shell environment variable, falling back to default program."
        );
        DEFAULT_METADATA.program.to_string()
    })
}

/// Sheller is a builder for `std::process::Command` that sets the shell program and arguments.
///
/// Please see the `Sheller::new` method for more information.
#[derive(Debug)]
pub struct Sheller<'a> {
    program: String,
    args: Vec<&'a str>,
    script: &'a str,
}

impl Default for Sheller<'_> {
    fn default() -> Self {
        Self {
            program: parse_program(),
            args: DEFAULT_METADATA.args.to_vec(),
            script: "",
        }
    }
}

impl<'a> Sheller<'a> {
    /// Create a new `Sheller` with the given `script` and platform-specific defaults.
    ///
    /// # Platform-specific defaults
    ///
    /// ## Windows
    ///
    /// When `target_family` is `windows`.
    ///
    /// Set the `COMSPEC` environment variable to `program`, and if the environment variable is not set, use `cmd.exe` as the fallback program.
    ///
    /// Also set the `args` to `["/D", "/S", "/C"]`.
    ///
    /// ## Unix
    ///
    /// When `target_family` is `unix`.
    ///
    /// Set the `SHELL` environment variable to `program`, and if the environment variable is not set, use `/bin/sh` as the fallback program.
    ///
    /// Also set the `args` to `["-c"]`.
    ///
    /// # Arguments
    ///
    /// * `script` - The shell script to run. This is dependent on the shell program.
    ///
    /// # Examples
    ///
    /// ```
    /// use sheller::Sheller;
    /// let sheller = Sheller::new("echo hello");
    /// ```
    #[must_use]
    pub fn new(script: &'a str) -> Self {
        Self {
            script,
            ..Default::default()
        }
    }

    /// Returns `std::process::Command` with the shell program and arguments set.
    ///
    /// # Examples
    ///
    /// ```
    /// use sheller::Sheller;
    /// let sheller = Sheller::new("echo hello");
    /// let command = sheller.build();
    /// ```
    #[must_use]
    pub fn build(self) -> process::Command {
        let mut command = process::Command::new(&self.program);
        command.args(&self.args);
        command.arg(self.script);
        command
    }
}
