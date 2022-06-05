# kitten
A fast, lightweight `cat` and `more` implementation for ANSI terminals (Linux, macOS, Redox, etc.), written purely in Rust.

## Install from release
Head over to the [releases page](https://github.com/appleplectic/kitten/releases/latest) and grab the latest binary. If a binary for your computer is not available, or you would like to contribute, see the section below.

## Building from source
Grab a copy of Rust from [the Rust website](https://www.rust-lang.org/) or from your package manager, clone this repository (either through `git clone` or "Download ZIP"), and `cd` to the repo with your terminal. 

To build a release-ready binary:
```
cargo build --release
```
This binary will be at `target/release/kitten`


To build a debug binary:
```
cargo build
```
This binary will be at `target/debug/kitten`


To run the program:
```
cargo run
```
