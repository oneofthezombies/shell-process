// crates/examples/readme/src/try_run.rs
use sheller::try_run;

fn main() -> std::io::Result<()> {
    try_run!("echo hello")?;
    Ok(())
}
