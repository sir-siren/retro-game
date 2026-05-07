# 🕹️ Terminal Arcade

Four classic arcade games in your terminal. No GPU. No browser. No Electron. Just Rust, raw TTY mode, and Unicode box-drawing characters.

```
╔═══════════════════════╗
║   TERMINAL  ARCADE    ║
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

### 🚗 Runner

4-lane highway dodger. Oncoming traffic. Speed is your multiplier and your problem.

| Key                 | Action     |
| ------------------- | ---------- |
| `↑` / `W`           | Lane up    |
| `↓` / `S`           | Lane down  |
| `→` / `D` / `Space` | Accelerate |
| `←` / `A`           | Brake      |
| `Q`                 | Menu       |

### 🧱 Bricks

Breakout clone. 5 levels. Armored bricks appear at level 3. Lives shown as `♥♥♥`.

| Key       | Action       |
| --------- | ------------ |
| `←` / `A` | Paddle left  |
| `→` / `D` | Paddle right |
| `Q`       | Menu         |

### 🐍 Snake

Eat food, grow, don't die. Speed increases as you eat. Classic rules.

| Key                | Action |
| ------------------ | ------ |
| `↑ ↓ ← →` / `WASD` | Turn   |
| `Q`                | Menu   |

### 🦖 Dino

Chrome T-Rex clone. Jump cacti, duck birds, survive as long as possible.

| Key                 | Action      |
| ------------------- | ----------- |
| `↑` / `W` / `Space` | Jump        |
| `↓` / `S`           | Duck (hold) |
| `Q`                 | Menu        |

---

## Install

### Build from source

Requires Rust 1.85+.

```bash
git clone https://github.com/sir-siren/terminal-arcade.git
cd terminal-arcade
RUSTFLAGS="-C target-cpu=native" cargo build --release
./target/release/arcade
```

---

## Stack

- [`crossterm`](https://github.com/crossterm-rs/crossterm) — cross-platform raw terminal I/O
- [`thiserror`](https://github.com/dtolnay/thiserror) — error types
- [`anyhow`](https://github.com/dtolnay/anyhow) — error propagation

No TUI framework. The renderer is a hand-rolled double-buffered character grid that only writes cells that changed since the last frame. 30fps, flicker-free.

---

## Requirements

- UTF-8 terminal with Unicode support
- Minimum 60×20 terminal size (80×24 recommended)

---

## License

MIT — see [LICENSE](./LICENSE).
