use clap::{Parser, Subcommand};
use std::{
    panic,
    process::{self, Stdio},
};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Check,
    Clippy,
    Fmt,
    Test {
        #[arg(short, long)]
        target: Option<String>,
    },
    PrePush,
}

fn run(program: &str, args: &[&str]) {
    let mut command = process::Command::new(program);
    command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(args);
    println!("Run {program} {args:?}");
    match command.status() {
        Ok(status) => {
            if !status.success() {
                eprintln!("Exit code: {:?}", status.code());
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {e:?}");
            std::process::exit(1);
        }
    }
}

fn check() {
    run("cargo", &["check", "--workspace"]);
}

fn clippy() {
    run(
        "cargo",
        &[
            "clippy",
            "--",
            "-D",
            "clippy::all",
            "-D",
            "clippy::pedantic",
        ],
    );
}

fn fmt() {
    run("cargo", &["fmt", "--", "--check"]);
}

fn test(target: Option<String>) {
    let Some(target) = target else {
        run("cargo", &["test", "--workspace"]);
        return;
    };

    run("cargo", &["test", "--target", target.as_str()]);
}

fn pre_push() {
    check();
    clippy();
    fmt();
    test(None);
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Check) => check(),
        Some(Command::Clippy) => clippy(),
        Some(Command::Fmt) => fmt(),
        Some(Command::Test { target }) => test(target),
        Some(Command::PrePush) => pre_push(),
        None => {
            panic!("No command");
        }
    }
}
