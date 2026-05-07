# Gameplay

## Controls (all games)

| Key | Action |
|-----|--------|
| `Q` | Quit to menu |
| `Ctrl+C` | Force quit |
| `1–5` | Menu selection |

---

## Runner

You're a car on a 4-lane highway. Traffic comes from the right. You control lane and speed.

**HUD:** score top-left, speed (mph) top-right.

**How it works:**
- Switch lanes with `↑`/`↓`
- Increase speed with `→`/`Space`, decrease with `←`
- Score accumulates every 10 ticks proportional to speed
- Level increases every 500 points — obstacles spawn faster and sometimes two at once

**Death:** any contact with a traffic car.

**Tips:** don't floor it immediately. At level 3+ double-spawns start appearing — two cars in different lanes at the same time. Moderate speed gives you more reaction time than it costs in score.

---

## Bricks

Standard Breakout. Clear all bricks to advance. 5 levels.

**HUD:** score top-left, remaining lives (`♥♥♥`) top-right.

**How it works:**
- Ball bounces off walls, ceiling, paddle, and bricks
- Normal bricks (`▓▓▓▓`) take 1 hit
- Armored bricks (`████`) take 2 hits, appear from level 3 onward
- Ball speed scales with level
- Lose a life when the ball falls below the paddle
- 3 lives total

**Scoring:** 10 points per brick hit (armored bricks award points on each hit, not just on destruction).

**Tips:** angle your shots by hitting the ball with the edge of the paddle. Getting the ball above the brick rows causes chain bounces that clear multiple bricks per trip.

---

## Snake

Eat food, grow longer, don't hit a wall or yourself.

**HUD:** score top-left, separator line below.

**How it works:**
- Head is `▣`, body is `█`, food is `□`
- Eating food: +10 points, +1 segment, slight speed increase
- Tick rate starts at 120ms and decreases by 2ms per food eaten (floor at 50ms)
- Input is queued up to 2 moves deep so you can buffer turns

**Death:** hitting any wall or any segment of your own body.

**Tips:** the input queue means you can pre-buffer a turn before the snake reaches a corner. At high lengths, plan 3–4 moves ahead and keep the snake in a predictable coil pattern.

---

## Dino

Side-scrolling infinite runner. Obstacles come from the right. Survive as long as possible.

**HUD:** `HI XXXXX XXXXX` top-right (high score, current score). High score persists within the session.

**How it works:**
- Speed starts at 1 and increments every 200 ticks (cap: 6)
- Score increments every 5 ticks
- Level increases every 200 ticks (1–5), unlocking harder obstacle types at higher levels

**Obstacles by level:**

| Level | Obstacle types |
|-------|---------------|
| 1 | Small cactus (`│╥`) |
| 2+ | Large cactus (`│┤╥`), double cactus, high bird |
| 3+ | Low bird (`╲══`) — requires duck |

**Controls:**
- `↑` / `W` / `Space` — jump (can't jump while ducking)
- `↓` / `S` — duck (hold to stay ducked; releasing stands back up)

**Death:** any hitbox overlap with an obstacle.

**Tips:**
- Low birds are at head height — duck early, the hitbox is unforgiving
- At speed 5+ you're reacting more than planning. Learn the visual rhythm
- High birds are decoration, you don't need to do anything
- Double cacti have a wider hitbox than they look — jump early