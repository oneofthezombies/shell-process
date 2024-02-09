// crates/examples/readme/src/run_with_log.rs
use sheller::run;

fn main() {
    init_log();

    run!("echo hello");
    // It will be printed as below, or panicked.
    // 2024-02-09T16:03:57.463018Z  INFO sheller: 🐚 $ Running command. command="/bin/bash" "-c" "echo hello"
    // hello
}

fn init_log() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::TRACE)
            .finish(),
    )
    .expect("setting default subscriber failed");
}
