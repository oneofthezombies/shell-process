// crates/examples/readme/src/pipe.rs
use sheller::new;

static EOL: &str = if cfg!(windows) { "\r\n" } else { "\n" };

fn main() {
    let output = new!("echo hello")
        .build()
        .stdout(std::process::Stdio::piped())
        .output()
        .unwrap();
    assert_eq!(output.stdout, format!("hello{}", EOL).as_bytes());
}
