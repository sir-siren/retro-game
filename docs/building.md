# Building

## Prerequisites

- **Rust**: Version 1.85.0 or later (edition 2024)
- **Platform**: Windows, macOS, Linux, or Android (via Termux)
- **Terminal**: Must support UTF-8 and Unicode box-drawing characters

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Quick Build

```bash
cargo build --release
```

The binary will be at `./target/release/arcade` (or `arcade.exe` on Windows).

## Running

```bash
cargo run --release
```

Or run the binary directly:

```bash
./target/release/arcade
```

## Platform-Specific Instructions

### Linux (x86_64)

```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

### Linux Static (musl — for portable binaries)

```bash
sudo apt-get install musl-tools
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

### Linux ARM64 (for Raspberry Pi, Termux, etc.)

```bash
sudo apt-get install gcc-aarch64-linux-gnu
rustup target add aarch64-unknown-linux-gnu

mkdir -p .cargo
echo '[target.aarch64-unknown-linux-gnu]' > .cargo/config.toml
echo 'linker = "aarch64-linux-gnu-gcc"' >> .cargo/config.toml

cargo build --release --target aarch64-unknown-linux-gnu
```

### macOS (Intel)

```bash
cargo build --release --target x86_64-apple-darwin
```

### macOS (Apple Silicon / M1+)

```bash
cargo build --release --target aarch64-apple-darwin
```

### Windows

```bash
cargo build --release --target x86_64-pc-windows-msvc
```

### Android (Termux)

Install Rust in Termux:

```bash
pkg install rust
```

Then build normally:

```bash
git clone <repo-url>
cd terminal-arcade
cargo build --release
./target/release/arcade
```

## WSL Build Notes

If you are developing on Windows but have Rust installed in WSL:

```bash
wsl
cd /mnt/d/retro-game
cargo build --release
cargo run --release
```

## Code Quality Checks

```bash
cargo clippy -- -D warnings
cargo fmt --check
cargo test
cargo doc --no-deps
```

## Release Profile

The `Cargo.toml` configures aggressive optimizations for release builds:

| Setting         | Value | Purpose                                     |
| --------------- | ----- | ------------------------------------------- |
| `opt-level`     | 3     | Maximum optimization                        |
| `lto`           | true  | Link-time optimization for smaller binaries |
| `codegen-units` | 1     | Single codegen unit for better optimization |
| `strip`         | true  | Strip debug symbols from binary             |
