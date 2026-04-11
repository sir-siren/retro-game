# Architecture

## Overview

Terminal Arcade is a monolithic Rust binary containing four arcade games sharing a common engine. The architecture follows a strict separation of concerns: the engine handles terminal I/O, input, and rendering, while each game module contains only pure game logic and state.

## Module Dependency Graph

```
main.rs
├── menu.rs (game selection and routing)
├── engine/
│   ├── input.rs     (keypress abstraction)
│   ├── loop_.rs     (tick-based game loop driver)
│   ├── renderer.rs  (double-buffered character grid)
│   └── terminal.rs  (terminal size and screen clearing)
├── games/
│   ├── rand.rs      (shared LCG random number generator)
│   ├── runner/      (Highway Dodger - 4-lane traffic)
│   │   ├── mod.rs   (GameLoop + Game trait impl)
│   │   ├── state.rs (pure data model)
│   │   ├── logic.rs (state transitions, collision, spawning)
│   │   └── render.rs(buffer drawing)
│   ├── bricks/      (Breakout clone)
│   ├── snake/       (Classic snake)
│   └── dino/        (Chrome T-Rex runner)
└── types/
    ├── error.rs     (AppError, GameError)
    ├── game.rs      (Game trait, GameResult enum)
    └── geometry.rs  (Vec2, TerminalSize, Direction, Score, etc.)
```

## Engine Architecture

### Double-Buffered Renderer (`renderer.rs`)

The renderer maintains two character grids: `cells` (current frame) and `prev` (last flushed frame). On each `flush()`, only cells that differ from the previous frame are written to stdout, minimizing terminal I/O and eliminating flicker.

```
Game Logic → buffer.clear() → buffer.place()/print() → buffer.flush()
                                                            ↓
                                              Only changed cells → stdout
```

### Game Loop (`loop_.rs`)

All games share a single loop driver via the `GameLoop` trait:

```
loop {
    poll_key()         → handle_input()
    tick()             → advance simulation
    status()           → check for game over
    render()           → draw to buffer
    flush()            → diff to terminal
    sleep(remaining)   → maintain frame rate
}
```

Each game runs at approximately 30fps (33ms tick interval).

### Input System (`input.rs`)

Raw crossterm events are translated into semantic `Key` variants: `Dir(Direction)`, `Action`, `Quit`, `Number(u8)`, or `None`. This abstraction keeps game logic completely decoupled from the terminal input layer.

## Game Module Pattern

Every game follows the same 4-file pattern:

| File        | Purpose                                                          |
| ----------- | ---------------------------------------------------------------- |
| `mod.rs`    | Implements `GameLoop` + `Game` traits, wires state/logic/render  |
| `state.rs`  | Pure data structures with no logic beyond constructors           |
| `logic.rs`  | All state mutation: input handling, physics, collision, spawning |
| `render.rs` | Projects state onto `Buffer` using character placement           |

This pattern enforces:

- **Single Responsibility**: State, logic, and rendering never mix
- **Testability**: Logic functions take `&mut State` and can be tested without a terminal
- **Dependency Inversion**: Games depend on `Buffer` (abstraction), not stdout (concretion)

## Data Flow

```
Terminal Events → Input Parser → Key enum
                                    ↓
                              Game.handle_input()
                                    ↓
                              Game.tick() → mutates state
                                    ↓
                              Game.render() → writes to Buffer
                                    ↓
                              Buffer.flush() → diffed output to terminal
```
