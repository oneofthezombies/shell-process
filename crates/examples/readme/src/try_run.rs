// crates/examples/readme/src/try_run.rs
use sheller::try_run;

fn main() -> sheller::Result<()> {
    try_run!("echo hello")
}
