# Sheller

🐚 Sheller is a shell command builder library written in Rust.  

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
sheller = "0.1"
```

Below is an example of calling `npm install`.

```rust
use sheller::Sheller;

let sheller
```