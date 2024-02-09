// crates/examples/readme/src/run_with_log.rs
use sheller::run;

fn main() {
    init_log();

    run!("echo hello");
    // 2024-02-09T19:11:29.897389Z  INFO sheller: Running command. command="/bin/bash" "-c" "echo hello"
    // hello
    // 2024-02-09T19:11:29.898254Z  INFO sheller: Succeeded to run command with zero exit code. command="/bin/bash" "-c" "echo hello"
}

fn init_log() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::TRACE)
            .finish(),
    )
    .expect("setting default subscriber failed");
}
