# Sheller

🐚 Sheller is a shell command builder and standard command extension library written in Rust.  

[![Build Status][actions-badge]][actions-url]
[![Crates.io][crates-badge]][crates-url]

[actions-badge]: https://github.com/oneofthezombies/sheller/workflows/CI/badge.svg
[actions-url]: https://github.com/oneofthezombies/sheller/actions?query=workflow?CI+branch=main
[crates-badge]: https://img.shields.io/crates/v/sheller.svg
[crates-url]: https://crates.io/crates/sheller

## Why I Created This

### TL;DR  

---

I created it because I want to call `npm install` from Rust on multiplatforms.  
(`npm` is installed with the file name `npm.cmd` on Windows platforms. and Rust `std::process::Command` does not support PATHEXT-based executable search like `cmd.exe`, `pwsh.exe` or `go`)

--- 

I use Rust to write utility functions for managing Rust library and application projects.  
For example, calling `cargo clippy -- -D clippy::all -D clippy::pedantic`.  
Currently, this project is also a Rust project. Therefore, I wrote utility functions in Rust.  

Let's take an example of the Git pre-push hook that verifies before pushing to Github.  
The `.cargo-husky/hooks/pre-push` file of this project is a file that is copied to `.git/hooks/pre-push` when the Cargo project is set up, and when `git push` is called, this script is called before a push is actually made.  
The implementation of this script is, to put it exaggeratedly, one line: `cargo run --package tool-dev -- pre-push`.  
The script calls the Rust command line in one line. And the actual implementation is written in Rust code.  
Within the code, call the command line again, such as `cargo check ...`, `cargo clippy ...`, `cargo fmt ...`, and `cargo test ...`.  

There are three reasons why it was written in Rust.  

First, the syntax of Unix Shell or Windows Batch scripts is very difficult.  
There is no major problem when writing one or two lines.  
However, management requirements are gradually increading.  
Conditional statements, loop statements, and function syntax are also not intuitive by my standards.  
Since I can't memorize the syntax, I have to look up Stack Overflow or ask Chat GPT every time.  

Second, to use easy JavaScript or Python, I need to add a dependency to the project development environment.  
In fact, installing Node.js or Python runtime in my development environment doesn't require much effort.  
However, Rust also has a convenient tool called `cargo run`.  
Depending on the size of the project, the build may take quite a while.  
However, once dependent libraries are built, no additional builds occur even if they are called multiple times.  
So, the developer experience is not bad after the first build.  

Third, I am a Rust newbie.  
The Rust project was first started in 2024, when this article was written.  
To become proficient, whenever I get a chance to program something, I try to write it all in Rust.  

The introduction was long.  
To get straight to the point, it's because of `npm install`.  
To explain cause and effect, it is as follows.  
I started a project to write a TypeScript compiler in Rust.  
I was writing code in Rust to install the TypeScript sample project in the development environment.  
To call `npm install` in Rust, I must call the shell command and pass `npm install` as an argument.  
This is because the `npm` command is a script, not an executable file.  
Because it is a script, it must be passed as an argument to the shell command.  
And this shell command is largely different for each Windows and Unix platform.  
The goal is not to create platform-independent scripts.  
The goal is to be able to call command lines in a shell environment.  

For this reason, I created the Sheller library.  
This is to write utility functions in the Rust project I use, or to use them in a Rust application if needed in the future.  

## How does it Work

If you want to call `echo hello` then,

### Windows  

When `target_family` is `windows`.  
Set the `COMSPEC` environment variable to `program`, and if the environment variable is not set, used `cmd.exe` as the fallback program.  
Also set the `args` to `["/D", "/S", "/C", "echo hello"]`.  

### Unix

When `target_family` is `unix`.  
Set the `SHELL` environment variable to program, and if the environment variable is not set, used `/bin/sh` as the fallback program.  
Also set the `args` to `["-c", "echo hello"]`.

## How to Use

Add `sheller` to your dependencies.

```toml
# Cargo.toml
[dependencies]
sheller = "0.5"
```

Below are examples using `sheller`.  

If you simply want to run a shell script, use it as follows.  

```rust
// crates/examples/readme/src/run.rs
use sheller::run;

fn main() {
    run!("echo hello");
    // It will be printed as below, or panicked.
    // hello
}
```

If you don't want `panic`, you can use the `try_run` methods to receive and process `sheller::Result<()>`.  

```rust
// crates/examples/readme/src/try_run.rs
use sheller::try_run;

fn main() -> sheller::Result<()> {
    try_run!("echo hello")
}
```

📢 If you want output of which command line is executed, add the [tracing](https://github.com/tokio-rs/tracing) to your dependencies.  
Sheller internally uses `tracing`, a pupular centralized structured logging system.  

Below is an example using `tracing`.

```toml
# Cargo.toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

```rust
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
```

👀 For more information on how to use tracing, please check the [tracing documentation](https://docs.rs/tracing/latest/tracing/index.html).

`Sheller` uses `std::process::Command`.  
If you want to change the current working path, stdout/stderr or environment variables, use the `Sheller::build` method.  
This method returns `std::process::Command`.  

Below is an example of changing the current working path.  

⚠️ If you don't see the `run` method, check `use sheller::CommandExt`.  

```rust
// crates/examples/readme/src/builder.rs
use sheller::{new, CommandExt};

fn main() {
    let mut command = new!("echo hello").build();
    command.current_dir("/my/dir").run();
}
```

Likewise, `run` and `try_run` can all be used.  

If you want to pipe stdout, please see the example below.  

```rust
// crates/examples/readme/src/pipe.rs
use sheller::new;

static EOL: &str = if cfg!(windows) { "\r\n" } else { "\n" };

fn main() {
    let output = new!("echo hello")
        .build()
        .stdout(std::process::Stdio::piped())
        .output()
        .unwrap();
    assert_eq!(output.stdout, format!("hello{EOL}").as_bytes());
}
```

In addition to the methods above, you can of course also use the Rust official `std::process::Command` methods.  
For more information about `std::process::Command`, please check [the Rust official page](https://doc.rust-lang.org/std/process/struct.Command.html).  

The `run` and `try_run` methods are implemented as `CommandExt`.  
The purpose of these methods is utility.  
So you don't necessarily have to use `Sheller`.  

Below is an example that uses only `CommandExt` without using `Sheller`.  

```rust
// crates/examples/readme/src/command_ext.rs
use sheller::CommandExt;

fn main() {
    let mut command = std::process::Command::new("echo");
    command.arg("hello").run();
}
```
