# 🕹️ Terminal Arcade

A retro terminal arcade suite built in Rust — four classic games with a monochrome OLED aesthetic, rendered directly in your terminal using raw TTY mode.

```
╔═══════════════════════╗
║   TERMINAL  ARCADE   ║
╚═══════════════════════╝

  ┌═┐
1 │█│  Runner
  └═┘
  ▓▓▓
2 ▓▓▓  Bricks
  ███
3 █□   Snake
  ▄██
4 ██   Dino

5      Quit
──────────────────────
   Select: 1-5
```

## Games

### 🚗 Runner (Highway Dodger)

A 4-lane traffic avoidance game. Dodge oncoming cars while managing your speed. The faster you go, the higher your score — but the harder it gets.

| Control             | Action             |
| ------------------- | ------------------ |
| `↑` / `W`           | Move up one lane   |
| `↓` / `S`           | Move down one lane |
| `→` / `D` / `Space` | Speed up           |
| `←` / `A`           | Slow down          |
| `Q`                 | Quit to menu       |

### 🧱 Bricks (Breakout)

Classic brick-breaking arcade game. Destroy all bricks to advance through 5 levels. Lives shown as hearts (♥♥♥).

| Control   | Action            |
| --------- | ----------------- |
| `←` / `A` | Move paddle left  |
| `→` / `D` | Move paddle right |
| `Q`       | Quit to menu      |

### 🐍 Snake

Navigate the snake to eat food (□) and grow. Hit a wall or yourself and it's game over. Speed increases as you eat.

| Control   | Action       |
| --------- | ------------ |
| `↑` / `W` | Turn up      |
| `↓` / `S` | Turn down    |
| `←` / `A` | Turn left    |
| `→` / `D` | Turn right   |
| `Q`       | Quit to menu |

### 🦖 Dino (Chrome T-Rex)

An infinite runner inspired by Chrome's offline dinosaur game. Jump over cacti, duck under birds, survive as long as you can.

| Control             | Action       |
| ------------------- | ------------ |
| `↑` / `W` / `Space` | Jump         |
| `↓` / `S`           | Duck (hold)  |
| `Q`                 | Quit to menu |

## Installation

### From Releases

Download the latest binary for your platform from the [Releases](../../releases) page:

| Platform              | Binary                      |
| --------------------- | --------------------------- |
| Linux x86_64          | `arcade-linux-x86_64`       |
| Linux x86_64 (static) | `arcade-linux-x86_64-musl`  |
| Linux ARM64 (Termux)  | `arcade-linux-aarch64`      |
| macOS Intel           | `arcade-macos-x86_64`       |
| macOS Apple Silicon   | `arcade-macos-aarch64`      |
| Windows               | `arcade-windows-x86_64.exe` |

```bash
chmod +x arcade-linux-x86_64
./arcade-linux-x86_64
```

### Build from Source

Requires Rust 1.85.0 or later.

```bash
git clone https://github.com/yourusername/terminal-arcade.git
cd terminal-arcade
cargo build --release
./target/release/arcade
```

### Termux (Android)

```bash
pkg install rust
git clone https://github.com/yourusername/terminal-arcade.git
cd terminal-arcade
cargo build --release
./target/release/arcade
```

## Features

- Pure terminal rendering — no external TUI frameworks
- Double-buffered diff-based renderer for flicker-free 30fps gameplay
- Monochrome OLED retro aesthetic with Unicode box-drawing characters
- Cross-platform: Windows, macOS, Linux, and Android (Termux)
- Zero dependencies beyond `crossterm`, `anyhow`, and `thiserror`
- Optimized release builds with LTO and stripped symbols

## Architecture

See the [docs](./docs/) directory for detailed documentation:

- [Architecture](./docs/architecture.md) — Module structure and data flow
- [Building](./docs/building.md) — Build instructions for all platforms
- [Gameplay](./docs/gameplay.md) — Detailed mechanics and scoring

## Requirements

- Terminal with Unicode support (UTF-8)
- Minimum terminal size: 60×20 characters
- Recommended: 80×24 or larger

## License

MIT License — see [LICENSE](./LICENSE) for details.
