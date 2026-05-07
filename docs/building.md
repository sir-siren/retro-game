# Building

## Prerequisites

- Rust 1.85.0+ (edition 2024)
- A terminal with UTF-8 support

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Quick Start

```bash
# dev build — fast compile, no optimisations
cargo run

# release build — optimised for your CPU
RUSTFLAGS="-C target-cpu=native" cargo build --release
./target/release/arcade
```

The `target-cpu=native` flag tells the compiler to use your CPU's full instruction set. Don't use it for binaries you plan to distribute — they'll crash on CPUs that don't share your instruction set.

## mise (recommended)

If you use [mise](https://mise.jdx.dev), all common tasks are wired up:

```bash
mise run run      # dev mode
mise run build    # release, native CPU
mise run start    # run last release build
mise run test     # nextest
mise run lint     # clippy with full pedantic flags
mise run ci       # fmt > check > lint > test > build
```

## Cross-compilation

### Linux static (portable, no libc dependency)

```bash
sudo apt install musl-tools
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

### Linux ARM64

```bash
sudo apt install gcc-aarch64-linux-gnu
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

Linker override in `.cargo/config.toml`:

```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

### macOS Apple Silicon

```bash
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

### Windows

```bash
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

### Android (Termux, native)

Termux runs native ARM binaries. No cross-compilation needed — just build on device:

```bash
pkg install rust git
cargo build --release
```

### All platforms (CI)

The GitHub Actions workflow in `.github/workflows/release.yml` builds for all platforms on every push to `main`. It uses [`cross`](https://github.com/cross-rs/cross) (Docker-based) for non-native targets and `cargo` directly for native ones.

## Code Quality

```bash
cargo fmt -- --check
cargo clippy -- -D clippy::all -D clippy::pedantic -D clippy::nursery
cargo nextest run
cargo doc --no-deps --open
```

The project enforces `clippy::pedantic` + `clippy::nursery` via `#![deny(...)]` in `main.rs`. Lint errors are compile errors.

## Release Profile

Defined in `Cargo.toml`:

| Setting           | Value     | Effect                                              |
| ----------------- | --------- | --------------------------------------------------- |
| `opt-level`       | `"z"`     | Optimise for size over raw speed                    |
| `lto`             | `true`    | Full LTO, removes dead code across crate boundaries |
| `codegen-units`   | `1`       | Single codegen unit, best inter-procedural analysis |
| `strip`           | `true`    | Strip all debug symbols                             |
| `panic`           | `"abort"` | No unwind tables, smaller binary                    |
| `overflow-checks` | `false`   | No runtime overflow detection                       |

If you want max throughput instead of min size, swap `opt-level = "z"` for `opt-level = 3`.

## WSL

Build and run from WSL — output renders in Windows Terminal:

```bash
cd /mnt/d/terminal-arcade
RUSTFLAGS="-C target-cpu=native -C link-arg=-fuse-ld=mold" cargo build --release
./target/release/arcade
```

`mold` cuts incremental link time significantly. Install with `sudo apt install mold`, no other config needed.
