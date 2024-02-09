use clap::{Parser, Subcommand};
use sheller::run;
use std::{env, panic};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Init,
    Check,
    Clippy,
    Fmt,
    Test {
        #[arg(short, long)]
        target: Option<String>,
    },
    PrePush,
}

fn check() {
    run!("cargo check --workspace");
}

fn clippy() {
    run!("cargo clippy -- -D clippy::all -D clippy::pedantic");
}

fn fmt() {
    run!("cargo fmt -- --check");
}

fn test(target: Option<String>) {
    let Some(target) = target else {
        run!("cargo test --workspace");
        return;
    };

    run!("cargo test --target {target}");
}

fn pre_push() {
    check();
    clippy();
    fmt();
    test(None);
}

/// During Github Actions Workflow, when running `rustup install nightly` inside a `cargo run --package tool-dev -- init` command on a Windows platform, it will fail with the following error:
/// ```text
/// error: could not create link from 'C:\Users\runneradmin\.cargo\bin\rustup.exe' to 'C:\Users\runneradmin\.cargo\bin\cargo.exe'
/// ```
/// So for Github Action, I changed to call `rustup install nightly` before calling `cargo run --package tool-dev -- init`.
/// Please see the workflow file at `.github/workflows/CI.yml`.
fn init() {
    if env::var("GITHUB_ACTIONS").is_err() {
        run!("rustup install nightly");
    }

    run!("rustup component add rustfmt clippy --toolchain nightly");
    run!("rustup override set nightly");
}

fn init_log() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::TRACE)
            .finish(),
    )
    .expect("setting default subscriber failed");
}

fn main() {
    init_log();

    let cli = Cli::parse();
    match cli.command {
        Some(Command::Init) => init(),
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
