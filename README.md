# Sheller

🐚 Sheller is a shell command builder and standard command extension library written in Rust.  

[![Build Status][actions-badge]][actions-url]
[![Crates.io][crates-badge]][crates-url]

[actions-badge]: https://github.com/oneofthezombies/sheller/workflows/CI/badge.svg
[actions-url]: https://github.com/oneofthezombies/sheller/actions?query=workflow?CI+branch=main
[crates-badge]: https://img.shields.io/crates/v/sheller.svg
[crates-url]: https://crates.io/crates/sheller

## Why I Made This

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

## How to Use

Add `sheller` to your dependencies.

```toml
# Cargo.toml
[dependencies]
sheller = "0.3"
```

Below are examples using `sheller`.  

If you simply want to run a shell script, use it as follows.  

```rust
// crates/examples/readme/src/run.rs
use sheller::run;

fn main() {
    run!("echo hello");
    // The log below is output to stdout.
    // 🐚 $ Running command: "/bin/bash" "-c" "echo hello"
    // hello
}
```

If you don't want `panic`, you can use the `try_run` methods to receive and process `std::io::Result<()>`.  

```rust
// crates/examples/readme/src/try_run.rs
use sheller::try_run;

fn main() -> std::io::Result<()> {
    try_run!("echo hello")?;
    Ok(())
}
```

`run` and `try_run` use default configurations.  
Configuration has the values `prefix` and `writer`.  
The default value for `prefix` is `"🐚 $ "`.  
The default value for `writer` is `std::sync::Mutex::new(Box::new(std::io::stdout()))`.  
This `prefix` and `writer` are used to print which command is executed before actually running, or which command is called when an error occurs.  

If you want to change the configurations, please follow the example below.  

```rust
// crates/examples/readme/src/run_with_config.rs
use sheller::{new, Config};

fn main() {
    // binding to variable
    let config = Config {
        prefix: "🦀 $ ".to_string(),
        ..Default::default()
    };
    new!("echo hello").run_with_config(&config);

    // without binding to variable
    new!("echo hello").run_with_config(&Config {
        prefix: String::from("🦀 $ "),
        ..Default::default()
    })
}
```

The `Sheller::run_with_config` method generates `panic` of the command failes.  
If you do not want `panic` to occur, please use `Sheller::try_run_with_config` to process `std::io::Result<()>`.  

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

Likewise, `run`, `run_with_config`, `try_run` and `try_run_with_config` can all be used.  

In addition to the four methods above, you can of course also use the Rust official `std::process::Command` methods.  
For more information about `std::process::Command`, please check [the Rust official page](https://doc.rust-lang.org/std/process/struct.Command.html).  

The `run`, `run_with_config`, `try_run`, and `try_run_with_config` methods are implemented as `CommandExt`.  
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

## Internal Implementation

### Windows  

When `target_family` is `windows`.  
Set the `COMSPEC` environment variable to `program`, and if the environment variable is not set, use `cmd.exe` as the fallback program.  
Also set the `args` to `["/D", "/S", "/C"]`.  

### Unix

When `target_family` is `unix`.  
Set the `SHELL` environment variable to program, and if the environment variable is not set, use `/bin/sh` as the fallback program.  
Also set the `args` to `["-c"]`.
