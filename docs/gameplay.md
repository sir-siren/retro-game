# Gameplay Guide

## General Controls

| Key               | Action                        |
| ----------------- | ----------------------------- |
| `Q`               | Quit current game / exit menu |
| `Ctrl+C`          | Force quit                    |
| Arrow keys / WASD | Directional input             |
| `Space` / `Enter` | Action / confirm              |
| `1-5`             | Menu selection                |

---

## Game 1: Runner (Highway Dodger)

### Concept

You drive a car on a 4-lane highway. Oncoming traffic approaches from the right. Dodge between lanes to survive. The faster you drive, the higher your score multiplier — but obstacles arrive faster too.

### HUD

- **Top Left**: `Score: <value>` — Your accumulated points
- **Top Right**: `<value>mph` — Your current speed

### Mechanics

- **Lanes**: 4 horizontal lanes separated by dashed lines
- **Movement**: Switch lanes with Up/Down
- **Speed**: Increase speed with Right/Space, decrease with Left
- **Collision**: Any contact with an oncoming car = instant game over
- **Scoring**: Points awarded every 10 ticks, proportional to your speed

### Strategy

- Start slow, learn the lane patterns
- Speed up gradually as you gain confidence
- Watch for double-spawns at higher levels — two cars in different lanes simultaneously

---

## Game 2: Bricks (Breakout)

### Concept

Classic breakout: destroy all bricks by bouncing a ball off your paddle. Clear all bricks to advance to the next level. 5 levels total.

### HUD

- **Top Left**: `Score: <value>`
- **Top Right**: Hearts representing lives (`♥♥♥`)

### Mechanics

- **Paddle**: Moves left/right at the bottom of the screen
- **Ball**: Bounces off walls, paddle, and bricks
- **Bricks**: Arranged in rows at the top. Hit a brick to damage it
    - Normal bricks (▓▓▓▓): 1 hit to destroy
    - Armored bricks (████): 2 hits to destroy (appear from level 3+)
- **Lives**: 3 lives. Lose a life when the ball falls below the paddle
- **Levels**: 5 levels with progressively more rows of bricks and faster ball speed

### Scoring

- 10 points per brick destroyed
- Armored bricks award points on each hit AND on destruction

### Strategy

- Aim for the edges of the paddle to angle the ball toward brick clusters
- Try to get the ball above the bricks for chain bounces

---

## Game 3: Snake

### Concept

Control a growing snake. Eat food to score points and grow longer. Hit a wall or your own body and it's game over.

### HUD

- **Top Left**: `Score: <value>`
- A solid horizontal line separates the HUD from the play area

### Mechanics

- **Snake**: Made of solid blocks (█), head marked with (▣)
- **Food**: Hollow square (□) spawning randomly
- **Growth**: Eating food adds one segment and awards 10 points
- **Speed**: The snake accelerates slightly with each food eaten
- **Death**: Hitting any wall boundary or your own body = game over
- **Direction**: Cannot reverse direction (no 180° turns)

### Scoring

- 10 points per food item
- Speed gradually increases as you eat — more risk, same reward

### Strategy

- Keep the snake looping in a predictable pattern
- Don't get trapped in corners
- As the snake gets long, plan your path several moves ahead

---

## Game 4: Dino (Chrome T-Rex)

### Concept

An infinite side-scrolling runner inspired by Chrome's offline dinosaur game. Jump over cacti, duck under birds, and survive as long as possible.

### HUD

- **Top Right**: `HI <high_score> <current_score>` in Chrome style

### Mechanics

- **Running**: The dino runs automatically. Speed increases over time
- **Jumping**: Press Up/Space to jump over ground obstacles
- **Ducking**: Hold Down to duck under low-flying birds
- **Obstacles**:
    - Small Cactus (│╥): Short, jump over
    - Large Cactus (│┤╥): Tall, jump over
    - Double Cactus: Two cacti side by side, jump over
    - Low Bird (╲══): At head height, duck under
    - High Bird: Above dino, no action needed (decoration)
- **Death**: Any collision = instant game over
- **Clouds**: Decorative clouds scroll in the background

### Scoring

- 1 point every 5 ticks
- Speed automatically increases every 200 ticks

### Strategy

- Time your jumps — don't jump too early or you'll land on the obstacle
- Duck early for birds — ducking has no cooldown
- At high speeds, rely on muscle memory and react to patterns
