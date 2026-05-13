# Building

## Requirements

- Rust 1.85.0 or newer
- Cargo from the same toolchain
- A UTF-8 terminal with enough room for terminal graphics

Check the toolchain:

```sh
rustc --version
cargo --version
rustup show active-toolchain
```

Install or update Rust with `rustup` when needed:

```sh
rustup update stable
```

## Quick Start

Run the arcade in development mode:

```sh
cargo run
```

Run a specific game:

```sh
cargo run -- --game snake
cargo run -- --game=invaders
```

Valid direct game keys are:

```text
runner
bricks
snake
dino
tetris
pong
invaders
minesweeper
flappy
```

## Release Build

Build the release binary:

```sh
cargo build --release
./target/release/arcade
```

For a local-only binary, you can ask LLVM to optimize for the current CPU:

```sh
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

Do not use `target-cpu=native` for a portable release artifact. It can emit
instructions that older or different CPUs do not support.

## mise Tasks

The repository includes `mise.toml` with these tasks:

| Task                 | Command                                                                 | Purpose                          |
| -------------------- | ----------------------------------------------------------------------- | -------------------------------- |
| `mise run run`       | `cargo run`                                                             | Launch in dev mode.              |
| `mise run build`     | `RUSTFLAGS="-C target-cpu=native" cargo build --release`                | Build a local optimized release. |
| `mise run start`     | `./target/release/arcade`                                               | Run the last release binary.     |
| `mise run check`     | `cargo check`                                                           | Type-check the main target.      |
| `mise run test`      | `cargo nextest run`                                                     | Run tests with cargo-nextest.    |
| `mise run lint`      | `cargo clippy -- -D clippy::all -D clippy::pedantic -D clippy::nursery` | Run strict Clippy.               |
| `mise run fmt`       | `cargo fmt`                                                             | Format source.                   |
| `mise run fmt:check` | `cargo fmt -- --check`                                                  | Check formatting.                |
| `mise run watch`     | `cargo watch -x run`                                                    | Re-run on file changes.          |
| `mise run ci`        | fmt, check, lint, nextest, release build                                | Local CI-style pipeline.         |
| `mise run clean`     | `cargo clean`                                                           | Remove build artifacts.          |

`cargo nextest` and `cargo watch` are external Cargo subcommands. If they are not
installed, use the plain Cargo equivalents:

```sh
cargo test
cargo run
```

## Recommended Checks

Use these before handing off changes:

```sh
cargo fmt -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

The project has strict lint configuration in `Cargo.toml`, including
`unsafe_code = "forbid"` and denied Clippy `all`, `pedantic`, and `nursery`
groups.

## Release Profile

The current `Cargo.toml` release profile is:

| Setting           | Value     | Notes                                              |
| ----------------- | --------- | -------------------------------------------------- |
| `opt-level`       | `"z"`     | Optimize for binary size.                          |
| `lto`             | `false`   | Link-time optimization is disabled.                |
| `codegen-units`   | `1`       | Improves optimization at the cost of compile time. |
| `strip`           | `true`    | Removes symbols from the release binary.           |
| `overflow-checks` | `false`   | Disables runtime overflow checks in release.       |
| `panic`           | `"abort"` | Avoids unwinding tables.                           |

## Platform Notes

`rusqlite` is built with the `bundled` feature, so the SQLite C source is built
with the project. Some targets may need a working C compiler in addition to the
Rust target.

For Linux musl builds:

```sh
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

For Linux ARM64 cross-builds, install an ARM64 linker and add the Rust target:

```sh
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

For macOS Apple Silicon from macOS:

```sh
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

For Windows MSVC from a Windows toolchain:

```sh
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

This repository does not currently include project-specific cross-compilation
linker config or a GitHub Actions release workflow.

## WSL

If Rust is installed in WSL, run the same commands with the WSL toolchain. For
best build performance, keep the project under the WSL filesystem, such as
`~/projects/terminal-arcade`, instead of under `/mnt/c/...`.
