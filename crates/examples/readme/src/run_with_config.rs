// crates/examples/readme/src/run_with_config.rs
use sheller::{new, Config};

fn main() {
    // binding to variable
    let config = Config {
        prefix: "🦀 $ ".to_string(),
    };
    new!("echo hello").run_with_config(&config);

    // without binding to variable
    new!("echo hello").run_with_config(&Config {
        prefix: String::from("🦀 $ "),
    });
}
