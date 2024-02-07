use std::{env, process};
use tracing::debug;

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

#[derive(Debug)]
pub struct Sheller<'a> {
    program: String,
    args: Vec<&'a str>,
}

impl Default for Sheller<'_> {
    fn default() -> Self {
        Self {
            program: parse_program(),
            args: DEFAULT_METADATA.args.to_vec(),
        }
    }
}

impl<'a> Sheller<'a> {
    #[must_use]
    pub fn new() -> Sheller<'a> {
        Self::default()
    }

    pub fn arg(&mut self, arg: &'a str) {
        self.args.push(arg);
    }

    pub fn args(&mut self, args: &'a [&'a str]) {
        self.args.extend_from_slice(args);
    }

    #[must_use]
    pub fn build(self) -> process::Command {
        let mut command = process::Command::new(self.program);
        command.args(self.args);
        command
    }
}
