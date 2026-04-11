use crate::engine::input::Key;
use crate::games::dino::state::{DinoObstacle, DinoObstacleKind, DinoState};
use crate::types::geometry::Direction;

const GRAVITY: i16 = 1;
const JUMP_FORCE: i16 = 6;
const DINO_COL: u16 = 8;
const DINO_WIDTH: u16 = 4;
const DUCK_WIDTH: u16 = 6;

fn rand(seed: u64) -> u64 {
    let mut x = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

pub fn tick(state: &mut DinoState) {
    if state.is_game_over {
        return;
    }

    state.tick = state.tick.wrapping_add(1);

    state.ground_scroll = state.ground_scroll.wrapping_add(state.speed);

    if state.tick % 5 == 0 {
        state.score.0 = state.score.0.saturating_add(1);
    }

    if state.tick % 200 == 0 {
        state.speed = state.speed.saturating_add(1).min(6);
        if state.level.0 < 5 {
            state.level.0 = state.level.0.saturating_add(1);
        }
    }

    apply_gravity(state);
    scroll_obstacles(state);
    check_collisions(state);
    maybe_spawn(state);
}

pub fn handle_input(state: &mut DinoState, key: Key) {
    if state.is_game_over {
        return;
    }

    match key {
        Key::Action | Key::Dir(Direction::Up) => {
            if !state.is_jumping && !state.is_ducking {
                state.is_jumping = true;
                state.jump_velocity = JUMP_FORCE;
            }
        }
        Key::Dir(Direction::Down) => {
            if !state.is_jumping {
                state.is_ducking = true;
            }
        }
        _ => {}
    }
}

pub fn release_duck(state: &mut DinoState) {
    state.is_ducking = false;
}

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

fn scroll_obstacles(state: &mut DinoState) {
    for obs in &mut state.obstacles {
        obs.col = obs.col.saturating_sub(state.speed);
    }
    state.obstacles.retain(|o| o.col > 2);
}

fn check_collisions(state: &mut DinoState) {
    let (dl, dr, dt, db) = dino_box(state);
    let ground = DinoState::ground_line(state.bounds);

    for obs in &state.obstacles {
        let (ol, or_, ot, ob_) = obstacle_box(obs, ground);
        if dl <= or_ && dr >= ol && dt <= ob_ && db >= ot {
            state.is_game_over = true;
            if state.score.0 > state.high_score {
                state.high_score = state.score.0;
            }
            return;
        }
    }
}

fn dino_box(state: &DinoState) -> (u16, u16, u16, u16) {
    if state.is_ducking {
        (
            DINO_COL,
            DINO_COL + DUCK_WIDTH - 1,
            state.dino_row + 1,
            state.dino_row + 1,
        )
    } else {
        (
            DINO_COL,
            DINO_COL + DINO_WIDTH - 1,
            state.dino_row,
            state.dino_row + 1,
        )
    }
}

fn obstacle_box(obs: &DinoObstacle, ground: u16) -> (u16, u16, u16, u16) {
    let stand = ground.saturating_sub(2);
    match obs.kind {
        DinoObstacleKind::SmallCactus => (obs.col, obs.col + 1, stand, stand + 1),
        DinoObstacleKind::LargeCactus => (obs.col, obs.col + 1, stand.saturating_sub(1), stand + 1),
        DinoObstacleKind::DoubleCactus => (obs.col, obs.col + 3, stand, stand + 1),
        DinoObstacleKind::LowBird => {
            let row = stand;
            (obs.col, obs.col + 2, row, row)
        }
        DinoObstacleKind::HighBird => {
            let row = stand.saturating_sub(4);
            (obs.col, obs.col + 2, row, row)
        }
    }
}

fn maybe_spawn(state: &mut DinoState) {
    let gap = 80u64.saturating_sub(u64::from(state.speed) * 8).max(25);
    if state.tick % gap != 0 {
        return;
    }

    let min_gap = state.bounds.width / 3;
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

fn pick_kind(r: u64, level: u8) -> DinoObstacleKind {
    let bucket = r % 10;
    match (bucket, level) {
        (0..=3, _) => DinoObstacleKind::SmallCactus,
        (4..=5, l) if l >= 2 => DinoObstacleKind::LargeCactus,
        (6, l) if l >= 2 => DinoObstacleKind::DoubleCactus,
        (7..=8, l) if l >= 3 => DinoObstacleKind::LowBird,
        (9, l) if l >= 2 => DinoObstacleKind::HighBird,
        _ => DinoObstacleKind::SmallCactus,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::geometry::TerminalSize;

    fn vp() -> TerminalSize {
        TerminalSize {
            width: 80,
            height: 24,
        }
    }

    #[test]
    fn jump_rises_then_falls() {
        let mut s = DinoState::new(vp());
        let start = s.dino_row;
        handle_input(&mut s, Key::Action);
        assert!(s.is_jumping);
        tick(&mut s);
        assert!(s.dino_row < start);
    }

    #[test]
    fn duck_hitbox_lower() {
        let mut s = DinoState::new(vp());
        s.is_ducking = true;
        let (_, _, top_duck, _) = dino_box(&s);
        s.is_ducking = false;
        let (_, _, top_stand, _) = dino_box(&s);
        assert!(top_duck > top_stand);
    }

    #[test]
    fn collision_triggers_game_over() {
        let mut s = DinoState::new(vp());
        s.obstacles.push(DinoObstacle {
            col: DINO_COL,
            kind: DinoObstacleKind::SmallCactus,
        });
        s.dino_row = DinoState::stand_row(s.bounds);
        check_collisions(&mut s);
        assert!(s.is_game_over);
    }
}
