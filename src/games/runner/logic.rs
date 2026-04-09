use crate::engine::input::Key;
use crate::games::runner::state::{Obstacle, ObstacleType, RunnerState};
use crate::types::geometry::Direction;

/// Basic linear congruential generator for spawning logic lacking heavy dependencies.
#[must_use]
pub fn fast_rand(seed: u64) -> u16 {
    let mut x = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    #[allow(clippy::cast_possible_truncation)]
    (x as u16)
}

/// Advances simulation tick.
pub fn tick(state: &mut RunnerState) {
    if state.is_game_over {
        return;
    }

    if state.hurt_ticks > 0 {
        state.hurt_ticks = state.hurt_ticks.saturating_sub(1);
    }

    state.tick = state.tick.wrapping_add(1);

    if state.tick % 30 == 0 {
        state.score.0 = state.score.0.saturating_add(10);
        check_level_up(state);
    }

    apply_gravity(state);
    move_obstacles(state);
    check_collision(state);
    spawn_obstacles(state);
}

/// Progresses the level if the score threshold is passed.
fn check_level_up(state: &mut RunnerState) {
    let required_score = u32::from(state.level.0) * 200;
    if state.score.0 >= required_score && state.level.0 < 5 {
        state.level.0 = state.level.0.saturating_add(1);
        state.speed = u16::from(state.level.0);
    }
}

/// Calculates and enforces jump parabola.
fn apply_gravity(state: &mut RunnerState) {
    if !state.is_jumping {
        return;
    }

    let ground = state.ground_row().saturating_sub(1);

    if state.jump_velocity > 0 {
        state.player_row = state.player_row.saturating_sub(1);
    } else {
        state.player_row = state.player_row.saturating_add(1);
    }

    state.jump_velocity = state.jump_velocity.saturating_sub(1);

    if state.player_row >= ground {
        state.player_row = ground;
        state.is_jumping = false;
        state.jump_velocity = 0;
    }
}

/// Shifts obstacles leftwards based on the speed factor.
fn move_obstacles(state: &mut RunnerState) {
    let mut speed_rem = state.speed;
    while speed_rem > 0 {
        for obs in &mut state.obstacles {
            obs.col = obs.col.saturating_sub(1);
        }
        speed_rem = speed_rem.saturating_sub(1);
    }
    state.obstacles.retain(|obs| obs.col > 0);
}

/// Implements hurt mechanic via axis-aligned bounds.
fn check_collision(state: &mut RunnerState) {
    if state.hurt_ticks > 0 {
        return;
    }

    let p_col = 4;
    let p_width = 3;
    let ground = state.ground_row().saturating_sub(1);

    let mut hit = false;
    for obs in &state.obstacles {
        let obs_width = match obs.kind {
            ObstacleType::Single | ObstacleType::Ceiling => 1,
            ObstacleType::Double => 2,
        };

        let obs_left = obs.col;
        let obs_right = obs.col + (obs_width - 1);

        let p_left = p_col;
        let p_right = p_col + p_width - 1;

        let horizontal = p_left <= obs_right && p_right >= obs_left;

        if horizontal {
            let p_top = state.player_row;
            match obs.kind {
                ObstacleType::Ceiling => {
                    let ceil_limit = state.bounds.height.saturating_sub(state.bounds.height / 2);
                    if p_top <= ceil_limit {
                        hit = true;
                    }
                }
                _ => {
                    if p_top >= ground.saturating_sub(1) {
                        hit = true;
                    }
                }
            }
        }
    }

    if hit {
        state.lives.0 = state.lives.0.saturating_sub(1);
        if state.lives.0 == 0 {
            state.is_game_over = true;
        } else {
            state.hurt_ticks = 15; // 500ms
        }
    }
}

/// Periodically generates new obstacles.
fn spawn_obstacles(state: &mut RunnerState) {
    let mut spawn_rate = 60u64.saturating_sub(u64::from(state.level.0) * 8);
    if spawn_rate < 20 {
        spawn_rate = 20;
    }

    if state.tick % spawn_rate == 0 {
        let r = fast_rand(state.tick ^ u64::from(state.score.0));
        
        let mut kind = ObstacleType::Single;
        if state.level.0 >= 2 && r % 3 == 0 {
            kind = ObstacleType::Double;
        }
        if state.level.0 >= 3 && r % 4 == 0 {
            kind = ObstacleType::Ceiling;
        }

        state.obstacles.push(Obstacle {
            col: state.bounds.width.saturating_sub(2),
            kind,
        });
    }
}

/// Accepts upstream inputs.
pub fn handle_input(state: &mut RunnerState, key: Key) {
    if state.is_game_over {
        return;
    }

    if !state.is_jumping && (key == Key::Action || key == Key::Dir(Direction::Up)) {
        state.is_jumping = true;
        state.jump_velocity = 5;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::geometry::TerminalSize;

    #[test]
    fn test_jump_mechanics() {
        let mut state = RunnerState::new(TerminalSize { width: 80, height: 24 });
        assert!(!state.is_jumping);
        
        // Initial ground
        let ground = state.player_row;
        
        handle_input(&mut state, Key::Action);
        assert!(state.is_jumping);
        assert_eq!(state.jump_velocity, 5);
        
        tick(&mut state);
        assert_eq!(state.player_row, ground - 1);
        assert_eq!(state.jump_velocity, 4);
    }
}
