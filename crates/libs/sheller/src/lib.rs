#![feature(exit_status_error)]

use tracing::{debug, error, info};

mod macros;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    ProcessExitStatus(std::process::ExitStatusError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::ProcessExitStatus(e) => write!(f, "Process exit status error: {e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<std::process::ExitStatusError> for Error {
    fn from(e: std::process::ExitStatusError) -> Self {
        Error::ProcessExitStatus(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

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
    std::env::var(DEFAULT_METADATA.env_key).unwrap_or_else(|e| {
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
pub struct Sheller {
    program: String,
    args: Vec<&'static str>,
    script: String,
}

impl Default for Sheller {
    fn default() -> Self {
        Self {
            program: parse_program(),
            args: DEFAULT_METADATA.args.into(),
            script: String::new(),
        }
    }
}

impl Sheller {
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
    ///
    /// let mut command = Sheller::new("echo hello").build();
    /// assert!(command.status().unwrap().success());
    /// ```
    #[must_use]
    pub fn new<T>(script: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            script: script.into(),
            ..Default::default()
        }
    }

    /// Returns `std::process::Command` with the shell program and arguments set.
    ///
    /// # Examples
    ///
    /// ```
    /// use sheller::Sheller;
    ///
    /// let mut command = Sheller::new("echo hello").build();
    /// assert!(command.status().unwrap().success());
    /// ```
    #[must_use]
    pub fn build(self) -> std::process::Command {
        let mut command = std::process::Command::new(&self.program);
        command.args(&self.args);
        command.arg(self.script);
        command
    }

    /// Run the shell command and panic if the command failed to run.
    ///
    /// # Examples
    /// ```
    /// use sheller::{CommandExt, Sheller};
    ///
    /// Sheller::new("echo hello").run();
    /// ```
    ///
    /// # Panics
    /// Panics if the command failed to run.
    pub fn run(self) {
        self.build().run();
    }

    /// Run the shell command and return a `Result`.
    ///
    /// # Examples
    /// ```
    /// use sheller::{CommandExt, Sheller};
    ///
    /// Sheller::new("echo hello").try_run().unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns an `Err` if the command failed to run.
    pub fn try_run(self) -> Result<()> {
        self.build().try_run()
    }
}

pub trait CommandExt {
    /// Run the command and panic if the command failed to run.
    ///
    /// # Examples
    /// ```
    /// use sheller::CommandExt;
    /// use std::process::Command;
    ///
    /// #[cfg(windows)]
    /// fn example() {
    ///     let mut command = Command::new("cmd.exe");
    ///     command.args(["/D", "/S", "/C", "echo hello"]).run();
    /// }
    ///
    /// #[cfg(unix)]
    /// fn example() {
    ///     let mut command = Command::new("echo");
    ///     command.arg("hello").run();
    /// }
    ///
    /// example();
    /// ```
    ///
    /// # Panics
    /// Panics if the command failed to run.
    fn run(&mut self);

    /// Run the command and return a `Result`.
    ///
    /// # Examples
    /// ```
    /// use sheller::CommandExt;
    /// use std::process::Command;
    ///
    /// #[cfg(windows)]
    /// fn example() {
    ///     let mut command = Command::new("cmd.exe");
    ///     command
    ///         .args(["/D", "/S", "/C", "echo hello"])
    ///         .try_run()
    ///         .unwrap();
    /// }
    ///
    /// #[cfg(unix)]
    /// fn example() {
    ///     let mut command = Command::new("echo");
    ///     command.arg("hello").try_run().unwrap();
    /// }
    ///
    /// example();
    /// ```
    ///
    /// # Errors
    /// Returns an `Err` if the command failed to run.
    fn try_run(&mut self) -> Result<()>;
}

impl CommandExt for std::process::Command {
    /// Run the command and panic if the command failed to run.
    ///
    /// # Examples
    /// ```
    /// use sheller::CommandExt;
    /// use std::process::Command;
    ///
    /// #[cfg(windows)]
    /// fn example() {
    ///     let mut command = Command::new("cmd.exe");
    ///     command.args(["/D", "/S", "/C", "echo hello"]).run();
    /// }
    ///
    /// #[cfg(unix)]
    /// fn example() {
    ///     let mut command = Command::new("echo");
    ///     command.arg("hello").run();
    /// }
    ///
    /// example();
    /// ```
    ///
    /// # Panics
    /// Panics if the command failed to run.
    fn run(&mut self) {
        self.try_run().unwrap();
    }

    /// Run the command and return a `Result` with `Ok` if the command was successful, and `Err` if the command failed.
    ///
    /// # Examples
    /// ```
    /// use sheller::CommandExt;
    /// use std::process::Command;
    ///
    /// #[cfg(windows)]
    /// fn example() {
    ///     let mut command = Command::new("cmd.exe");
    ///     command
    ///         .args(["/D", "/S", "/C", "echo hello"])
    ///         .try_run()
    ///         .unwrap();
    /// }
    ///
    /// #[cfg(unix)]
    /// fn example() {
    ///     let mut command = Command::new("echo");
    ///     command.arg("hello").try_run().unwrap();
    /// }
    ///
    /// example();
    /// ```
    ///
    /// # Errors
    /// Returns an `Err` if the command failed to run.
    fn try_run(&mut self) -> Result<()> {
        info!(command = ?self, "Running command.");
        let mut command = self.spawn().map_err(|e| {
            error!(command = ?self, error = ?e, "Failed to spawn command.");
            e
        })?;
        let status = command.wait().map_err(|e| {
            error!(command = ?self, error = ?e, "Failed to wait for command.");
            e
        })?;
        match status.exit_ok() {
            Ok(()) => {
                info!(command = ?self, "Succeeded to run command.");
                Ok(())
            }
            Err(e) => {
                error!(command = ?self, error = ?e, "Failed to run command.");
                Err(e.into())
            }
        }
    }
}
