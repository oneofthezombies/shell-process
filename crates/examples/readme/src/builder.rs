// crates/examples/readme/src/builder.rs
use sheller::{new, CommandExt};

fn main() {
    let mut command = new!("echo hello").build();
    command.current_dir("/my/dir").run();
}
