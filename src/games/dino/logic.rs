//! Pure update logic for the Dino game — no I/O.

use crate::engine::input::Key;
use crate::games::dino::state::{DinoObstacle, DinoObstacleKind, DinoState};
use crate::types::geometry::Direction;

/// Gravity applied each tick while jumping (rows per tick).
const GRAVITY: i16 = 1;
/// Initial upward velocity when a jump starts.
const JUMP_FORCE: i16 = 7;
/// Invincibility ticks granted after a hit (~500 ms at 30 fps).
const HURT_DURATION: u16 = 15;
/// Dino left edge column.
const DINO_COL: u16 = 5;
/// Dino width in characters.
const DINO_WIDTH: u16 = 3;
/// Ducking dino width in characters.
const DUCK_WIDTH: u16 = 5;

/// Minimal LCG for obstacle variety without external crates.
#[must_use]
fn rand(seed: u64) -> u64 {
    let mut x = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

/// Advances one simulation tick.
pub fn tick(state: &mut DinoState) {
    if state.is_game_over {
        return;
    }

    if state.hurt_ticks > 0 {
        state.hurt_ticks = state.hurt_ticks.saturating_sub(1);
    }

    state.tick = state.tick.wrapping_add(1);

    if state.tick % 20 == 0 {
        state.score.0 = state.score.0.saturating_add(1);
        check_level_up(state);
    }

    apply_gravity(state);
    scroll_obstacles(state);
    check_collisions(state);
    maybe_spawn(state);
}

/// Handles directional and action keys.
pub fn handle_input(state: &mut DinoState, key: Key) {
    if state.is_game_over {
        return;
    }

    match key {
        Key::Action | Key::Dir(Direction::Up) => {
            if !state.is_jumping && !state.is_ducking {
                state.is_jumping = true;
                state.jump_velocity = JUMP_FORCE;
                state.is_ducking = false;
            }
        }
        Key::Dir(Direction::Down) => {
            if !state.is_jumping {
                state.is_ducking = true;
            }
        }
        _ => {
            // Release duck when down key is not held — handled per frame by checking
            // absence of down input; approximated here by resetting on any other key.
            state.is_ducking = false;
        }
    }
}

/// Releases the duck posture.
pub fn release_duck(state: &mut DinoState) {
    state.is_ducking = false;
}

/// Levels up when score thresholds are crossed.
fn check_level_up(state: &mut DinoState) {
    let threshold = u32::from(state.level.0) * 150;
    if state.score.0 >= threshold && state.level.0 < 5 {
        state.level.0 = state.level.0.saturating_add(1);
        state.speed = state.level.0.into();
    }
}

/// Applies jump physics and landing detection.
fn apply_gravity(state: &mut DinoState) {
    if !state.is_jumping {
        return;
    }

    if state.jump_velocity > 0 {
        let rise = u16::try_from(state.jump_velocity).unwrap_or(1);
        state.dino_row = state.dino_row.saturating_sub(rise.min(state.dino_row));
    } else {
        let fall = u16::try_from(state.jump_velocity.unsigned_abs()).unwrap_or(1);
        state.dino_row = state.dino_row.saturating_add(fall);
    }

    state.jump_velocity = state.jump_velocity.saturating_sub(GRAVITY);

    let stand = DinoState::stand_row(state.bounds);
    if state.dino_row >= stand {
        state.dino_row = stand;
        state.is_jumping = false;
        state.jump_velocity = 0;
    }
}

/// Moves all obstacles left and removes off-screen ones.
fn scroll_obstacles(state: &mut DinoState) {
    for obs in &mut state.obstacles {
        obs.col = obs.col.saturating_sub(state.speed);
    }
    state.obstacles.retain(|o| o.col > 2);
}

/// AABB collision detection between dino and all obstacles.
fn check_collisions(state: &mut DinoState) {
    if state.hurt_ticks > 0 {
        return;
    }

    let (dl, dr, dt, db) = dino_box(state);
    let ground = DinoState::ground_line(state.bounds);
    let mut hit = false;

    for obs in &state.obstacles {
        let (ol, or_, ot, ob) = obstacle_box(obs, ground);
        if dl <= or_ && dr >= ol && dt <= ob && db >= ot {
            hit = true;
            break;
        }
    }

    if hit {
        state.lives.0 = state.lives.0.saturating_sub(1);
        if state.lives.0 == 0 {
            state.is_game_over = true;
        } else {
            state.hurt_ticks = HURT_DURATION;
            // Reset position on hit
            state.dino_row = DinoState::stand_row(state.bounds);
            state.is_jumping = false;
            state.is_ducking = false;
            state.jump_velocity = 0;
        }
    }
}

/// Returns (left, right, top, bottom) hitbox for the dino.
fn dino_box(state: &DinoState) -> (u16, u16, u16, u16) {
    if state.is_ducking {
        (DINO_COL, DINO_COL + DUCK_WIDTH - 1, state.dino_row, state.dino_row)
    } else {
        (DINO_COL, DINO_COL + DINO_WIDTH - 1, state.dino_row.saturating_sub(1), state.dino_row)
    }
}

/// Returns (left, right, top, bottom) hitbox for an obstacle.
fn obstacle_box(obs: &DinoObstacle, ground: u16) -> (u16, u16, u16, u16) {
    let stand = ground.saturating_sub(1);
    match obs.kind {
        DinoObstacleKind::SmallCactus => {
            (obs.col, obs.col, stand.saturating_sub(1), stand)
        }
        DinoObstacleKind::LargeCactus => {
            (obs.col, obs.col, stand.saturating_sub(2), stand)
        }
        DinoObstacleKind::DoubleCactus => {
            (obs.col, obs.col.saturating_add(2), stand.saturating_sub(1), stand)
        }
        DinoObstacleKind::LowBird => {
            // At head height — duck removes the head row, so duck avoids this.
            let row = stand.saturating_sub(1);
            (obs.col, obs.col.saturating_add(2), row, row)
        }
        DinoObstacleKind::HighBird => {
            // Well above dino — visual only, never collides.
            let row = stand.saturating_sub(5);
            (obs.col, obs.col.saturating_add(2), row, row)
        }
    }
}

/// Spawns a new obstacle based on tick timing and level.
fn maybe_spawn(state: &mut DinoState) {
    let gap = spawn_gap(state.level.0);
    if state.tick % gap != 0 {
        return;
    }

    // Ensure minimum spacing from the last obstacle.
    let min_gap = u16::from(state.bounds.width) / 3;
    if let Some(last) = state.obstacles.last() {
        if last.col > state.bounds.width.saturating_sub(min_gap) {
            return;
        }
    }

    let r = rand(state.tick ^ u64::from(state.score.0));
    let kind = pick_kind(r, state.level.0);
    state.obstacles.push(DinoObstacle {
        col: state.bounds.width.saturating_sub(3),
        kind,
    });
}

/// Ticks between spawns — decreases with level.
fn spawn_gap(level: u8) -> u64 {
    let base: u64 = 80;
    base.saturating_sub(u64::from(level) * 10).max(30)
}

/// Picks an obstacle kind based on randomness and current level.
fn pick_kind(r: u64, level: u8) -> DinoObstacleKind {
    let bucket = r % 10;
    match (bucket, level) {
        (0..=4, _) => DinoObstacleKind::SmallCactus,
        (5..=6, l) if l >= 2 => DinoObstacleKind::LargeCactus,
        (7, l) if l >= 2 => DinoObstacleKind::DoubleCactus,
        (8, l) if l >= 3 => DinoObstacleKind::LowBird,
        (9, l) if l >= 2 => DinoObstacleKind::HighBird,
        _ => DinoObstacleKind::SmallCactus,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::geometry::TerminalSize;

    fn vp() -> TerminalSize {
        TerminalSize { width: 80, height: 24 }
    }

    #[test]
    fn test_jump_rises_then_falls() {
        let mut s = DinoState::new(vp());
        let start = s.dino_row;
        handle_input(&mut s, Key::Action);
        assert!(s.is_jumping);
        tick(&mut s);
        assert!(s.dino_row < start, "dino should rise after jump");
    }

    #[test]
    fn test_duck_hitbox_is_lower() {
        let mut s = DinoState::new(vp());
        s.is_ducking = true;
        let (_, _, top, _) = dino_box(&s);
        s.is_ducking = false;
        let (_, _, stand_top, _) = dino_box(&s);
        assert!(top > stand_top, "duck hitbox top should be lower than standing");
    }

    #[test]
    fn test_lives_decrease_on_collision() {
        let mut s = DinoState::new(vp());
        let ground = DinoState::ground_line(s.bounds);
        let stand = DinoState::stand_row(s.bounds);
        s.obstacles.push(DinoObstacle {
            col: DINO_COL,
            kind: DinoObstacleKind::SmallCactus,
        });
        // Force cactus into collision position by placing it at dino column
        // and ensuring dino is at stand row with no hurt ticks.
        s.dino_row = stand;
        s.hurt_ticks = 0;
        let _ = ground; // used in obstacle_box
        check_collisions(&mut s);
        assert_eq!(s.lives.0, 2);
    }
}
