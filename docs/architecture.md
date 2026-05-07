# Architecture

## Overview

One binary. Four games. A shared engine. No shared state between games — each game owns its own state struct and the engine drives all of them through the same two traits.

The engine handles terminal I/O, input translation, and the game loop. The games handle everything else: state, logic, collision, rendering. Neither side knows the other's internals.

## Module Layout

```
src/
├── main.rs               entry point, raw mode setup, panic hook
├── menu.rs               game selection, routing, menu rendering
├── engine/
│   ├── input.rs          crossterm events → Key enum
│   ├── loop_.rs          tick-based game loop, GameLoop trait
│   ├── renderer.rs       double-buffered character grid
│   └── terminal.rs       viewport size, screen clear
├── games/
│   ├── rand.rs           LCG PRNG (no external dependency)
│   ├── runner/           highway dodger
│   ├── bricks/           breakout
│   ├── snake/            snake
│   └── dino/             chrome t-rex
└── types/
    ├── error.rs          AppError, GameError
    ├── game.rs           Game trait, GameResult
    └── geometry.rs       Vec2, TerminalSize, Direction, Score, Level, Lives
```

Each game module follows the same four-file pattern:

```
games/<name>/
├── mod.rs      GameLoop + Game trait impls, wires everything together
├── state.rs    pure data structs, no logic
├── logic.rs    state mutation — input, physics, collision, spawning
└── render.rs   reads state, writes to Buffer
```

This keeps logic testable without a terminal and rendering free of business logic.

## Renderer

`Buffer` holds two `Vec<char>` grids of identical size: `cells` (current frame) and `prev` (last flushed frame). On `flush()` it walks both grids simultaneously and emits cursor moves + character writes only for cells that differ. The result is minimal stdout writes per frame regardless of screen size.

```
game.render(&mut buffer)   // game writes chars into cells[]
buffer.flush(...)          // diff against prev[], write only changes
                           // then swap prev = cells
```

Frame rate is ~30fps (33ms tick). The loop measures elapsed time and sleeps only the remainder, so the tick rate is stable under light load.

## Game Loop

`run_loop<G: GameLoop>` drives any game that implements `GameLoop`:

```rust
pub trait GameLoop {
    fn resize(&mut self, size: TerminalSize);
    fn handle_input(&mut self, key: Key);
    fn tick(&mut self);
    fn render(&self, buffer: &mut Buffer);
    fn status(&self) -> Option<GameResult>;
}
```

Each iteration: poll for input → handle it → tick → check status → render → flush → sleep.

`Key::Quit` is handled by the loop itself before reaching the game. Terminal resize is detected on `Key::None` (the timeout path) and propagated via `resize()`.

## Input

Raw crossterm key events are translated into a small semantic enum before reaching any game code:

```rust
pub enum Key {
    Quit,
    Dir(Direction),  // Up / Down / Left / Right
    Action,          // Space or Enter
    Number(u8),      // 1–9
    None,            // timeout or unrecognised key
}
```

Games never import crossterm. If the terminal layer changes, only `input.rs` needs updating.

## Game Trait

`Game` is what `menu.rs` calls. It has two methods:

```rust
pub trait Game {
    fn name(&self) -> &str;
    fn run(&mut self, viewport: TerminalSize) -> anyhow::Result<GameResult>;
}
```

`run()` calls `run_loop` internally. The menu doesn't know or care what's inside.

## Error Handling

Two error types:

- `GameError` — I/O errors that happen inside a game loop (crossterm calls, terminal queries). Propagated with `?`.
- `AppError` — top-level wrapper, converts from both `GameError` and `io::Error`.

`anyhow::Result` is used at the `Game::run` boundary so the menu can handle game errors without importing game-internal types.

## PRNG

`games/rand.rs` exports a single `const fn fast_rand(seed: u64) -> u64` — a xorshift64 variant. Used by obstacle spawners in Runner and Dino. No `rand` crate, no thread-local state, fully deterministic per seed. Each game derives its seed from tick count XOR'd with the current score so patterns don't repeat the same way each run.

## Release Profile

The `Cargo.toml` release profile uses `opt-level = "z"`, full LTO, single codegen unit, stripped symbols, and `panic = "abort"`. The final binary is small enough to distribute as a single file with no runtime dependencies.