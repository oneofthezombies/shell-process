// crates/examples/readme/src/command_ext.rs
use sheller::CommandExt;

fn main() {
    let mut command = std::process::Command::new("echo");
    command.arg("hello").run();
}
