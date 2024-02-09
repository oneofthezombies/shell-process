// crates/examples/readme/src/run.rs
use sheller::run;

fn main() {
    run!("echo hello");
    // It will be printed as below, or panicked.
    // hello
}
