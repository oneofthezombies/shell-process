#[macro_use]
extern crate lazy_static;

use tracing::debug;

lazy_static! {
    static ref GLOBAL_CONFIG: Config = Config::default();
}

/// Configuration for shell execution and error logging.
///
/// # Examples
///
/// initialize with default values:
/// ```
/// use sheller::Config;
/// let config = Config::default();
/// ```
///
/// initialize with custom values:
/// ```
/// use sheller::Config;
/// let config = Config {
///    prefix: "🦀 $ ".to_string(),
///   ..Default::default()
/// };
///
///
pub struct Config {
    pub prefix: String,
    pub writer: std::sync::Mutex<Box<dyn std::io::Write + Sync + Send>>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prefix: "🐚 $ ".to_string(),
            writer: std::sync::Mutex::new(Box::new(std::io::stdout())),
        }
    }
}

impl Config {
    fn try_println(&self, message: &str) -> std::io::Result<()> {
        let mut writer = self.writer.lock().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to lock writer: {:?}", e),
            )
        })?;
        writeln!(writer, "{}{}\n", self.prefix, message)?;
        writer.flush()?;
        Ok(())
    }
}

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
    ///
    /// let mut command = Sheller::new("echo hello").build();
    /// assert!(command.status().unwrap().success());
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

    pub fn run(self) {
        self.build().run();
    }

    pub fn try_run(self) -> Result<(), std::io::Error> {
        self.build().try_run()
    }

    pub fn run_with_config(self, config: &Config) {
        self.build().run_with_config(config);
    }

    pub fn try_run_with_config(self, config: &Config) -> Result<(), std::io::Error> {
        self.build().try_run_with_config(config)
    }
}

pub trait CommandExt {
    fn run(&mut self);
    fn try_run(&mut self) -> Result<(), std::io::Error>;
    fn run_with_config(&mut self, config: &Config);
    fn try_run_with_config(&mut self, config: &Config) -> Result<(), std::io::Error>;
}

impl CommandExt for std::process::Command {
    fn run(&mut self) {
        self.try_run().unwrap();
    }

    fn run_with_config(&mut self, config: &Config) {
        self.try_run_with_config(config).unwrap();
    }

    fn try_run(&mut self) -> Result<(), std::io::Error> {
        self.try_run_with_config(&GLOBAL_CONFIG)
    }

    fn try_run_with_config(&mut self, config: &Config) -> Result<(), std::io::Error> {
        config.try_println(&format!("Running command: {:?}", self))?;
        let mut command = self.spawn().or_else(|e| {
            config.try_println(&format!("Failed to spawn command: {:?}", e))?;
            Err(e)
        })?;
        let status = command.wait().or_else(|e| {
            config.try_println(&format!("Failed to wait for command: {:?}", e))?;
            Err(e)
        })?;
        if !status.success() {
            let message = format!(
                "Failed to run command: {:?} with status: {:?}",
                self, status
            );
            config.try_println(&message)?;
            return Err(std::io::Error::new(std::io::ErrorKind::Other, message));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_global() {
        assert_eq!(GLOBAL_CONFIG.prefix, "🐚 $ ");
    }
}
