# Cerebrallib

Cerebrallib is a library for creating a brainfuck VM.

![Rust](https://github.com/dsouzadyn/cerebrallib/workflows/Rust/badge.svg?branch=master)

### Installation
1. Create a new cargo project
```bash
$ cargo new project_name
$ cd project_name
```
2. Install `cargo-edit`
```bash
$ cargo install cargo-edit
```
3. Install `cerebrallib`
```bash
$ cargo add cerebrallib --git https://github.com/dsouzadyn/cerebrallib
```
4. Done

### Usage
1. Code
```rust
// src/main.rs code
use cerebrallib::cerebral;
use std::io
fn main() {
    let code = String::from("++++");
    let mut vm = cerebral::CerebralVM::new(code, io::stdin(), io::stdout());
    vm.execute();
}
```
2. Build
```bash
$ cargo build
```
3. Run
```bash
$ cargo run
```
