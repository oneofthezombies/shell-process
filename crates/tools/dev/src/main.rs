use clap::{Parser, Subcommand};
use sheller::Sheller;
use std::panic;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Init {
        #[arg(short, long)]
        target: Option<String>,
    },
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
    Sheller::new("cargo check --workspace").run();
}

fn clippy() {
    Sheller::new("cargo clippy -- -D clippy::all -D clippy::pedantic").run();
}

fn fmt() {
    Sheller::new("cargo fmt -- --check").run();
}

fn test(target: Option<String>) {
    let Some(target) = target else {
        Sheller::new("cargo test --workspace").run();
        return;
    };

    Sheller::new(format!("cargo test --target {target}").as_str()).run();
}

fn pre_push() {
    check();
    clippy();
    fmt();
    test(None);
}

fn init(target: Option<String>) {
    Sheller::new("rustup install nightly").run();
    Sheller::new("rustup component add rustfmt --toolchain nightly").run();
    Sheller::new("rustup override set nightly").run();

    if let Some(target) = target {
        Sheller::new(format!("rustup component add clippy --toolchain nightly-{target}").as_str())
            .run();
    };
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Init { target }) => init(target),
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
