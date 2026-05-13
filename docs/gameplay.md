# Gameplay

## Menu

| Key               | Action                         |
| ----------------- | ------------------------------ |
| `Up` / `W`        | Move selection up              |
| `Down` / `S`      | Move selection down            |
| `Enter` / `Space` | Launch selected game           |
| `1` through `9`   | Launch a game by menu position |
| `Q` / `Ctrl+C`    | Quit                           |

The menu shows the saved high score for each game when the SQLite score database
opens successfully.

## Shared Controls

| Key                  | Action                                      |
| -------------------- | ------------------------------------------- |
| Arrow keys or `WASD` | Directional input                           |
| `Space` / `Enter`    | Main action                                 |
| `P`                  | Pause                                       |
| `R`                  | Retry from a game-over or completion screen |
| `Q` / `Ctrl+C`       | Quit to menu or exit a direct game          |

The game loop shows a three-second countdown before play starts. While paused,
press any key to resume; press `Q` to quit.

## Runner

A four-lane traffic dodger. The player car stays near the left side while
traffic enters from the right.

| Key                               | Action             |
| --------------------------------- | ------------------ |
| `Up` / `W`                        | Move up one lane   |
| `Down` / `S`                      | Move down one lane |
| `Right` / `D` / `Space` / `Enter` | Accelerate         |
| `Left` / `A`                      | Brake              |

- Speed starts at 60 mph, clamps between 30 and 200 mph, and changes in 5 mph steps.
- Score increases every 10 ticks by `speed / 10`.
- Level increases every 500 points up to level 5.
- Higher levels spawn traffic more often; level 3 and later can spawn two cars at once.
- Collision with traffic in the current lane ends the run.

## Bricks

A Breakout-style game with a paddle, bouncing ball, colored bricks, lives, and
five levels.

| Key           | Action            |
| ------------- | ----------------- |
| `Left` / `A`  | Move paddle left  |
| `Right` / `D` | Move paddle right |

- The paddle moves two columns per input.
- You start with 3 lives.
- Hitting a brick awards 10 points.
- Levels have `3 + level` rows of bricks.
- Level 3 and later make the top row armored; armored bricks take two hits.
- Clearing level 5 completes the game.
- Missing the ball costs one life; losing all lives ends the game.

## Snake

Classic snake with a two-input turn queue.

| Key                  | Action       |
| -------------------- | ------------ |
| Arrow keys or `WASD` | Queue a turn |

- The snake starts moving right.
- Food gives 10 points and grows the snake by one segment.
- Tick rate starts at 120 ms and drops by 2 ms per food, down to a 50 ms floor.
- Direct reverse turns are ignored.
- Hitting a wall or the snake body ends the game.

## Dino

A side-scrolling runner inspired by the browser dinosaur game.

| Key                            | Action                                      |
| ------------------------------ | ------------------------------------------- |
| `Up` / `W` / `Space` / `Enter` | Jump                                        |
| `Down` / `S`                   | Duck on the ground; fast-fall while jumping |

- Speed starts at 6.0 and slowly ramps to 13.0.
- Score increases continuously based on speed.
- Level is `score / 250 + 1`, capped at level 10.
- Large cacti can appear immediately.
- Cactus clusters unlock at 300 score.
- Low and high birds unlock at 500 score.
- The renderer switches into a night palette after 700 score and alternates by score cycle.
- Any hitbox overlap with an obstacle ends the run.

## Tetris

A 10x20 tetromino game with next preview, hold, ghost piece, scoring, and level
speedups.

| Key                            | Action                  |
| ------------------------------ | ----------------------- |
| `Left` / `A`                   | Move piece left         |
| `Right` / `D`                  | Move piece right        |
| `Down` / `S`                   | Soft drop one row       |
| `Up` / `W` / `Space` / `Enter` | Rotate clockwise        |
| `C`                            | Hold or swap held piece |

- The board is 10 columns by 20 rows.
- Hold can be used once per spawned piece.
- The ghost piece shows where the active piece will land.
- Line scoring is 100, 300, 500, or 800 points for 1, 2, 3, or 4 lines, multiplied by level.
- Level increases every 10 cleared lines, capped at level 10.
- Gravity starts at 700 ms and speeds up by 45 ms per level.
- The game ends when a newly spawned or held piece cannot fit.

## Pong

Paddle game against the CPU. First side to 11 points wins.

| Key                              | Action                 |
| -------------------------------- | ---------------------- |
| `Up` / `W`                       | Move left paddle up    |
| `Down` / `S`                     | Move left paddle down  |
| `2` during the opening moments   | Enable two-player mode |
| `Left` / `A` in two-player mode  | Move right paddle up   |
| `Right` / `D` in two-player mode | Move right paddle down |

- Scoring on the CPU side gives the player 100 arcade points.
- Player level increases every two player points, capped at level 10.
- CPU tracking gets stronger as the level rises.
- The ball speeds up slightly after paddle hits.
- Player score 11 completes the game; CPU score 11 is game over.

## Space Invaders

A wave shooter with 5 rows of 11 aliens, destructible shields, player bullets,
and alien bullets.

| Key               | Action     |
| ----------------- | ---------- |
| `Left` / `A`      | Move left  |
| `Right` / `D`     | Move right |
| `Space` / `Enter` | Fire       |

- Top-row aliens are worth 30 points.
- Middle rows are worth 20 points.
- Lower rows are worth 10 points.
- Level increases after each cleared wave, capped at level 10.
- Aliens march faster and shoot more often at higher levels.
- Player bullet capacity is 1 before level 3, 2 from level 3, and 3 from level 6.
- Shields start with 2 hit points per shield cell and are not regenerated between waves.
- Getting shot or letting aliens descend to the player area ends the game.

## Minesweeper

Cursor-driven Minesweeper. The first reveal generates the board and protects the
selected cell plus its neighboring cells.

| Key                     | Action                      |
| ----------------------- | --------------------------- |
| `1` before first reveal | Easy, 9x9 with 10 mines     |
| `2` before first reveal | Medium, 16x16 with 40 mines |
| `3` before first reveal | Hard, 30x16 with 99 mines   |
| Arrow keys or `WASD`    | Move cursor                 |
| `F`                     | Toggle flag                 |
| `Space` / `Enter`       | Reveal cell                 |

- Difficulty cannot be changed after the first reveal.
- Revealing a safe cell gives 1 point.
- Revealing an empty zero-adjacent cell flood-reveals neighbors.
- Clearing all safe cells awards `mine_count * 5` bonus points and completes the board.
- Revealing a mine shows all mines and ends the game.

## Flappy Bird

A pipe-dodging game with gravity and one-button flapping.

| Key                            | Action |
| ------------------------------ | ------ |
| `Up` / `W` / `Space` / `Enter` | Flap   |

- Gravity increases downward velocity until a cap.
- Flapping sets upward velocity immediately.
- Passing a pipe gives 1 point.
- Level increases every 10 points, capped at level 10.
- Pipe gaps shrink from 8 rows toward a 4-row minimum as level rises.
- Pipe speed increases with level.
- Hitting the top, ground, or a pipe ends the game.
