# cliutils

Utilities for the CLI written in Rust

## Motivation

CLI Utils aims to help new entry-level Rust programmers learn to write software
using Rust. We believe real contributions following the Open Source practices
are great for learning how to develop software in a team or community.

## Structure

This project as many other Rust programming language projects uses the workspace
structure. Each program lives inside of the `src` directory and its referred
from the `Cargo.toml` file in the root directory.

## Getting Started

To run any of these utilities you must install The Rust Programming Language,
you can do that using [`rustup`'s installation here][1]. Then use `cargo` to
run any of the programs included.

```bash
cargo run -p <program name>
```

Each program name belongs to the name specified in the `Cargo.toml` file inside
of the sub directories available in the `src` directory.

[1]: https://rustup.rs

## Contributing

Everyone is welcome to contribute.
