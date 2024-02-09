// crates/examples/readme/src/run.rs
use sheller::run;

fn main() {
    run!("echo hello");
    // The log below is output to stdout.
    // 🐚 $ Running command: "/bin/bash" "-c" "echo hello"
    // hello
}
