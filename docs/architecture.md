# Architecture

## Overview

Terminal Arcade is a single Rust binary named `arcade`. It contains nine games,
a shared terminal engine, a Ratatui-backed renderer, a menu, and SQLite
high-score persistence.

The design keeps terminal I/O outside of game logic. Each game owns its state,
input handling, update logic, collision rules, scoring, and rendering. The
engine drives each game through a small `GameLoop` trait, and the menu launches
games through the `Game` trait.

## Module Layout

```text
src/
  main.rs                 binary entry point, raw mode, alternate screen, panic cleanup
  menu.rs                 menu UI, game routing, score database integration
  engine/
    input.rs              crossterm key events to semantic Key values
    loop_.rs              countdown, pause handling, resize handling, timed game loop
    renderer.rs           styled character buffer rendered into a Ratatui frame
    terminal.rs           terminal size query
  games/
    rand.rs               deterministic SplitMix64 helper
    runner/               four-lane traffic dodger
    bricks/               Breakout-style brick breaker
    snake/                classic snake
    dino/                 side-scrolling runner
    tetris/               tetromino stacker
    pong/                 paddle game
    invaders/             Space Invaders-style shooter
    minesweeper/          cursor-driven mine board
    flappy/               pipe dodger
  persistence/
    mod.rs                persistence exports
    schema.rs             high-score table and index SQL
    score_db.rs           SQLite score database wrapper
  types/
    error.rs              AppError and GameError
    game.rs               Game trait and GameResult
    geometry.rs           shared Score, Level, Lives, Vec2, TerminalSize, Direction
  ui/
    components.rs         shared Ratatui overlays and HUD helpers
    theme.rs              shared color palette and styles
```

Each game directory follows the same pattern:

```text
games/<name>/
  mod.rs                  Game and GameLoop implementation
  state.rs                state structs and constants
  logic.rs                input handling, physics, rules, scoring, collision
  render.rs               state-to-buffer drawing
```

## Startup

`main.rs` enables raw mode, enters the terminal alternate screen, hides the
cursor, creates a `ratatui::Terminal<CrosstermBackend<_>>`, then either opens the
menu or launches a direct game selected with `--game <key>` or `--game=<key>`.

A panic hook restores the terminal before delegating to the default panic hook.
Normal shutdown also leaves the alternate screen, shows the cursor, and disables
raw mode.

## Input

`engine::input::parse_key` converts raw crossterm events into this semantic enum:

```rust
pub enum Key {
    Quit,
    Dir(Direction),
    Action,
    Retry,
    Pause,
    Hold,
    Flag,
    Number(u8),
    None,
}
```

Games do not import crossterm. Shared mappings include arrow keys and `WASD` for
directions, `Space`/`Enter` for `Action`, `Q`/`Ctrl+C` for `Quit`, `R` for
`Retry`, `P` for `Pause`, `C` for `Hold`, `F` for `Flag`, and `1` through `9`
for menu or difficulty choices.

## Game Loop

Every active game implements `GameLoop`:

```rust
pub trait GameLoop {
    fn resize(&mut self, size: TerminalSize);
    fn tick(&mut self);
    fn handle_input(&mut self, key: Key);
    fn render(&self, buffer: &mut Buffer);
    fn status(&self) -> Option<GameResult>;
}
```

`run_loop` owns the frame timing. It creates a `Buffer`, calls `resize`, renders
a three-second countdown, then repeatedly:

1. polls terminal input for the current tick duration,
2. dispatches key input or resize events,
3. handles pause mode when `P` is pressed,
4. calls `tick`,
5. checks `status`,
6. renders into the buffer,
7. draws the buffer into the Ratatui frame,
8. sleeps the remaining tick time.

Most games run at a 33 ms loop tick. Minesweeper runs at 80 ms because it is
turn-like and only needs a coarse timer.

## Game Trait

The menu talks to games through `Game`:

```rust
pub trait Game {
    fn name(&self) -> &str;
    fn run(&mut self, terminal: &mut ArcadeTerminal) -> anyhow::Result<GameResult>;
}
```

`GameResult` is one of `Quit`, `Retry { score, level }`,
`GameOver { score, level }`, or `Complete { score, level }`. The menu retries a
game when `should_retry()` is true and saves scores for results that carry a
score and level.

## Renderer

`engine::renderer::Buffer` is a flat row-major grid of styled cells:

```rust
struct StyledCell {
    ch: char,
    style: ratatui::style::Style,
}
```

Games draw with helpers such as `place`, `print`, `print_styled`,
`print_right`, `horizontal_line`, and `dashed_line`. `render_to` copies the
buffer into the active Ratatui frame buffer, centered in the available area.

This renderer is not a stdout diff renderer. Ratatui owns frame output and
terminal flushing through the `Terminal::draw` call.

## Persistence

`ScoreDb::open()` creates or opens `terminal-arcade/arcade.db` under the
platform data directory returned by `dirs::data_dir()`. The schema is:

```sql
CREATE TABLE IF NOT EXISTS high_scores (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    game      TEXT    NOT NULL,
    score     INTEGER NOT NULL,
    level     INTEGER NOT NULL DEFAULT 1,
    played_at TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_game_score
ON high_scores(game, score DESC);
```

The menu opens the database once, ignores database startup failures so games can
still run, displays each game's best score, and saves score-bearing results after
a game exits or retries.

## Error Handling

`GameError` wraps terminal I/O failures and terminal-size failures inside the
game loop. `AppError` wraps top-level I/O, game, and database errors. The game
boundary uses `anyhow::Result<GameResult>` so menu code does not need to know
each lower-level error type.

The crate forbids `unsafe_code` through `Cargo.toml` lints.

## Build Profile And Lints

The package is `terminal-games` version `1.2.0`, Rust edition 2024, with
`rust-version = "1.85.0"`. The binary target is named `arcade`.

Clippy `all`, `pedantic`, and `nursery` are denied in `Cargo.toml`, with local
allows for noisy naming lints and a few checked cast lints in rendering and game
math.

The release profile is tuned for a small terminal binary:

| Setting           | Value     |
| ----------------- | --------- |
| `opt-level`       | `"z"`     |
| `lto`             | `false`   |
| `codegen-units`   | `1`       |
| `strip`           | `true`    |
| `overflow-checks` | `false`   |
| `panic`           | `"abort"` |
