# Terminal Arcade

Nine classic terminal games in one Rust binary. Terminal Arcade uses raw
keyboard input through `crossterm`, styled terminal rendering through `ratatui`,
and a small SQLite database for high scores.

```sh
╭───────────────────────────────────────────────────────────────────────────────────────────╮
│                                                                                           │
│                                     TERMINAL ARCADE                                       │
│                                                                                           │
╰───────────────────────────────────────────────────────────────────────────────────────────╯
╭───────────────────────────────────────────────────────────────────────────────────────────╮
│> Runner  HI: 1411                                                                         │
│  Bricks  HI: 40                                                                           │
│  Snake  HI: 150                                                                           │
│  Dino  HI: 412                                                                            │
│  Tetris  HI: ---                                                                          │
│  Pong  HI: ---                                                                            │
│  Space Invaders  HI: 820                                                                  │
│  Minesweeper  HI: 33                                                                      │
│  Flappy Bird  HI: ---                                                                     │
│                                                                                           │
│                                                                                           │
│                                                                                           │
│                                                                                           │
│                                                                                           │
│                                                                                           │
│                                                                                           │
│                                                                                           │
│                                                                                           │
│                                                                                           │
│                                                                                           │
╰───────────────────────────────────────────────────────────────────────────────────────────╯
                    [Up/Down] Navigate  [Enter] Play  [1-9] Quick  [Q] Quit

```

## Games

| Game           | Key           | Summary                                                                              |
| -------------- | ------------- | ------------------------------------------------------------------------------------ |
| Runner         | `runner`      | Four-lane traffic dodger with speed-based scoring.                                   |
| Bricks         | `bricks`      | Breakout-style paddle game with five levels and armored bricks.                      |
| Snake          | `snake`       | Classic snake with queued turns and increasing speed.                                |
| Dino           | `dino`        | Side-scrolling runner with cacti, birds, speed ramping, and day/night styling.       |
| Tetris         | `tetris`      | 10x20 tetromino stacker with next piece, hold, ghost piece, line clears, and levels. |
| Pong           | `pong`        | Paddle game against the CPU, with an early two-player toggle.                        |
| Space Invaders | `invaders`    | Alien waves, shields, player shots, alien shots, and row-based scoring.              |
| Minesweeper    | `minesweeper` | Cursor-driven Easy, Medium, and Hard boards with a safe first reveal.                |
| Flappy Bird    | `flappy`      | Flap through pipes as gaps shrink and scrolling speed increases.                     |

## Controls

The menu uses `Up`/`Down` to select, `Enter` to launch, `1` through `9` for
quick launch, and `Q` to quit.

| Key                                           | Action                        |
| --------------------------------------------- | ----------------------------- |
| `Up` `Down` `Left` `Right` or `W` `A` `S` `D` | Directional input             |
| `Space` or `Enter`                            | Main action                   |
| `P`                                           | Pause the active game         |
| `R`                                           | Retry from a game-over screen |
| `Q` or `Ctrl+C`                               | Quit the game or menu         |
| `C`                                           | Hold piece in Tetris          |
| `F`                                           | Flag a Minesweeper cell       |

Game-specific details are in [docs/gameplay.md](docs/gameplay.md).

## Build And Run

Requires Rust 1.85.0 or newer and a UTF-8 terminal. The crate uses Rust edition 2024.

```sh
cargo run
```

Build the release binary:

```sh
cargo build --release
./target/release/arcade
```

Build for the local CPU if you do not plan to distribute the binary:

```sh
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

Run one game directly:

```sh
cargo run -- --game snake
cargo run -- --game=invaders
./target/release/arcade --game tetris
```

Valid direct game keys are `runner`, `bricks`, `snake`, `dino`, `tetris`,
`pong`, `invaders`, `minesweeper`, and `flappy`.

## High Scores

High scores are stored in a SQLite database under the platform data directory in
a `terminal-arcade` folder. On Linux this is usually:

```text
~/.local/share/terminal-arcade/arcade.db
```

The menu shows the best saved score for each game. Scores are saved after
game-over, completion, or retry results that return a score.

## Development

Common checks:

```sh
cargo fmt -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

If you use `mise`, the repository includes tasks for `run`, `build`, `start`,
`check`, `test`, `lint`, `fmt`, `fmt:check`, `watch`, `ci`, and `clean`.

More build notes are in [docs/building.md](docs/building.md). Architecture notes
are in [docs/architecture.md](docs/architecture.md).

## Stack

- `crossterm` for raw terminal input and alternate-screen control
- `ratatui` for terminal frames, widgets, styles, and buffer output
- `rusqlite` with bundled SQLite for high-score persistence
- `dirs` for platform data-directory lookup
- `thiserror` for typed application errors
- `anyhow` for top-level error propagation

## License

MIT. See [LICENSE](LICENSE).
