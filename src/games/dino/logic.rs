use crate::engine::input::Key;
use crate::games::dino::state::{DinoObstacle, DinoObstacleKind, DinoState, DinoStatus};
use crate::games::rand::fast_rand;
use crate::types::geometry::Direction;

const GRAVITY: f32 = 0.6;
const FAST_FALL_GRAVITY: f32 = GRAVITY * 2.5;
const JUMP_FORCE: f32 = 10.0;
const VERTICAL_SCALE: f32 = 0.055;
const DINO_COL: f32 = 8.0;
const DINO_WIDTH: f32 = 4.0;
const DUCK_WIDTH: f32 = 6.0;
const MAX_SPEED: f32 = 13.0;

/// Axis-aligned bounding box used for collision detection.
#[derive(Debug, Clone, Copy)]
struct HitBox {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

pub fn tick(state: &mut DinoState) {
    if state.status.is_game_over() {
        return;
    }

    state.tick = state.tick.wrapping_add(1);
    state.speed = (state.speed + 0.001).min(MAX_SPEED);
    state.ground_scroll += state.speed * 0.16;
    state.level.0 = ((state.score.0 / 250) + 1).min(10) as u8;

    state.score_progress += state.speed * 0.08;
    if state.score_progress >= 1.0 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let gained = state.score_progress.floor() as u32;
        state.score.0 = state.score.0.saturating_add(gained);
        #[allow(clippy::cast_precision_loss)]
        {
            state.score_progress -= gained as f32;
        }
    }

    apply_gravity(state);
    scroll_obstacles(state);
    check_collisions(state);
    maybe_spawn(state);
}

pub fn handle_input(state: &mut DinoState, key: Key) {
    match key {
        Key::Action | Key::Dir(Direction::Up)
            if !state.status.is_jumping() && !state.status.is_ducking() =>
        {
            state.status = DinoStatus::Jumping;
            state.velocity_y = JUMP_FORCE;
        }
        Key::Dir(Direction::Down) if state.status.is_jumping() => {
            state.velocity_y -= FAST_FALL_GRAVITY;
        }
        Key::Dir(Direction::Down) => {
            state.status = DinoStatus::Ducking;
        }
        _ => {}
    }
}

pub const fn release_duck(state: &mut DinoState) {
    if state.status.is_ducking() {
        state.status = DinoStatus::Running;
    }
}

fn apply_gravity(state: &mut DinoState) {
    if !state.status.is_jumping() {
        return;
    }

    state.dino_y -= state.velocity_y * VERTICAL_SCALE;
    state.velocity_y -= GRAVITY;

    let stand = DinoState::stand_y(state.bounds);
    if state.dino_y >= stand {
        state.dino_y = stand;
        state.status = DinoStatus::Running;
        state.velocity_y = 0.0;
    }
}

fn scroll_obstacles(state: &mut DinoState) {
    for obs in &mut state.obstacles {
        obs.col -= state.speed * 0.16;
    }
    state.obstacles.retain(|o| o.col > 2.0);
}

pub fn check_collisions(state: &mut DinoState) {
    let dino = dino_hitbox(state);
    let ground = DinoState::ground_line(state.bounds);

    for obs in &state.obstacles {
        let obstacle = obstacle_hitbox(*obs, ground);
        let overlaps = dino.left <= obstacle.right
            && dino.right >= obstacle.left
            && dino.top <= obstacle.bottom
            && dino.bottom >= obstacle.top;
        if overlaps {
            if state.score.0 > state.high_score {
                state.high_score = state.score.0;
            }
            state.status = DinoStatus::GameOver;
            return;
        }
    }
}

const fn dino_hitbox(state: &DinoState) -> HitBox {
    if state.status.is_ducking() {
        HitBox {
            left: DINO_COL,
            right: DINO_COL + DUCK_WIDTH - 1.0,
            top: state.dino_y + 1.0,
            bottom: state.dino_y + 1.8,
        }
    } else {
        HitBox {
            left: DINO_COL,
            right: DINO_COL + DINO_WIDTH - 1.0,
            top: state.dino_y,
            bottom: state.dino_y + 2.0,
        }
    }
}

fn obstacle_hitbox(obs: DinoObstacle, ground: u16) -> HitBox {
    let stand = f32::from(ground.saturating_sub(2));
    match obs.kind {
        DinoObstacleKind::SmallCactus => HitBox {
            left: obs.col,
            right: obs.col + 1.0,
            top: stand,
            bottom: stand + 1.0,
        },
        DinoObstacleKind::LargeCactus => HitBox {
            left: obs.col,
            right: obs.col + 2.0,
            top: stand - 1.0,
            bottom: stand + 1.0,
        },
        DinoObstacleKind::CactusCluster => HitBox {
            left: obs.col,
            right: obs.col + 5.0,
            top: stand,
            bottom: stand + 1.0,
        },
        DinoObstacleKind::LowBird => HitBox {
            left: obs.col,
            right: obs.col + 2.0,
            top: stand,
            bottom: stand,
        },
        DinoObstacleKind::HighBird => HitBox {
            left: obs.col,
            right: obs.col + 2.0,
            top: stand - 4.0,
            bottom: stand - 4.0,
        },
    }
}

fn maybe_spawn(state: &mut DinoState) {
    if state.spawn_cooldown > 0 {
        state.spawn_cooldown = state.spawn_cooldown.saturating_sub(1);
        return;
    }

    let min_gap = f32::from(state.bounds.width) * min_gap_ratio(state.speed);
    if let Some(last) = state.obstacles.last() {
        if last.col > f32::from(state.bounds.width) - min_gap {
            return;
        }
    }

    let rand_value = fast_rand(state.tick ^ u64::from(state.score.0));
    let kind = pick_kind(rand_value, state.score.0);
    state.obstacles.push(DinoObstacle {
        col: f32::from(state.bounds.width.saturating_sub(3)),
        kind,
    });
    state.spawn_cooldown = next_spawn_cooldown(state.speed, rand_value);
}

fn min_gap_ratio(speed: f32) -> f32 {
    speed.mul_add(-0.015, 0.55).clamp(0.32, 0.55)
}

fn next_spawn_cooldown(speed: f32, rand_value: u64) -> u16 {
    let base = speed.mul_add(-4.0, 80.0).clamp(28.0, 70.0);
    let rand_bucket = u16::try_from(rand_value % 40).unwrap_or(0);
    let variance = 0.8 + f32::from(rand_bucket) / 100.0;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let cooldown = (base * variance).round() as u16;
    cooldown.max(18)
}

const fn pick_kind(rand_value: u64, score: u32) -> DinoObstacleKind {
    let rand_bucket = rand_value % 10;
    match rand_bucket {
        4 | 5 => DinoObstacleKind::LargeCactus,
        6 if score >= 300 => DinoObstacleKind::CactusCluster,
        7 | 8 if score >= 500 => DinoObstacleKind::LowBird,
        9 if score >= 500 => DinoObstacleKind::HighBird,
        _ => DinoObstacleKind::SmallCactus,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::geometry::TerminalSize;

    fn default_viewport() -> TerminalSize {
        TerminalSize {
            width: 80,
            height: 24,
        }
    }

    #[test]
    fn jump_input_raises_dino_above_starting_row() {
        let mut state = DinoState::new(default_viewport());
        let start_y = state.dino_y;
        handle_input(&mut state, Key::Action);
        assert!(state.status.is_jumping());
        tick(&mut state);
        assert!(state.dino_y < start_y);
    }

    #[test]
    fn duck_hitbox_top_is_lower_than_standing_top() {
        let mut state = DinoState::new(default_viewport());
        state.status = DinoStatus::Ducking;
        let duck_top = dino_hitbox(&state).top;
        state.status = DinoStatus::Running;
        let stand_top = dino_hitbox(&state).top;
        assert!(duck_top > stand_top);
    }

    #[test]
    fn obstacle_overlap_with_dino_triggers_game_over() {
        let mut state = DinoState::new(default_viewport());
        state.obstacles.push(DinoObstacle {
            col: DINO_COL,
            kind: DinoObstacleKind::SmallCactus,
        });
        state.dino_y = DinoState::stand_y(state.bounds);
        check_collisions(&mut state);
        assert!(state.status.is_game_over());
    }
}
