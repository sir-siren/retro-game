use crate::engine::input::Key;
use crate::games::flappy::state::{FlappyState, Pipe};
use crate::games::rand::fast_rand;
use crate::types::geometry::Direction;

const GRAVITY: f32 = 0.17;
const FLAP_IMPULSE: f32 = -1.8;
const PIPE_WIDTH: u16 = 4;

pub fn tick(state: &mut FlappyState) {
    if state.is_game_over || state.is_complete {
        return;
    }

    state.tick = state.tick.wrapping_add(1);
    state.ground_scroll = state.ground_scroll.wrapping_add(1);
    state.velocity_y = (state.velocity_y + GRAVITY).min(2.4);
    state.bird_y += state.velocity_y;

    move_pipes(state);
    maybe_spawn_pipe(state);
    score_passed_pipes(state);
    check_collision(state);
}

pub const fn handle_input(state: &mut FlappyState, key: Key) {
    if matches!(key, Key::Action | Key::Dir(Direction::Up)) {
        state.velocity_y = FLAP_IMPULSE;
    }
}

fn move_pipes(state: &mut FlappyState) {
    let speed = state.pipe_speed();
    for pipe in &mut state.pipes {
        pipe.x -= speed;
    }
    state
        .pipes
        .retain(|pipe| pipe.x + f32::from(PIPE_WIDTH) > 0.0);
}

fn maybe_spawn_pipe(state: &mut FlappyState) {
    let spacing = f32::from(state.level.0).mul_add(-1.5, 38.0).max(24.0);
    if state
        .pipes
        .last()
        .is_some_and(|pipe| pipe.x > f32::from(state.bounds.width) - spacing)
    {
        return;
    }

    let min_y = 4u16;
    let max_y = state.ground_y().saturating_sub(6);
    let range = max_y.saturating_sub(min_y).max(1);
    let rand_value = fast_rand(state.tick ^ u64::from(state.score.0));
    let offset = u16::try_from(rand_value % u64::from(range)).unwrap_or(0);
    state.pipes.push(Pipe {
        x: f32::from(state.bounds.width.saturating_sub(1)),
        gap_y: min_y.saturating_add(offset),
        is_scored: false,
    });
}

fn score_passed_pipes(state: &mut FlappyState) {
    for pipe in &mut state.pipes {
        if !pipe.is_scored && pipe.x + f32::from(PIPE_WIDTH) < f32::from(FlappyState::BIRD_X) {
            pipe.is_scored = true;
            state.score.0 = state.score.0.saturating_add(1);
            state.level.0 = ((state.score.0 / 10) + 1).min(10) as u8;
        }
    }
}

fn check_collision(state: &mut FlappyState) {
    if state.bird_y < 2.0 || state.bird_y >= f32::from(state.ground_y()) {
        state.is_game_over = true;
        return;
    }

    let bird_x = f32::from(FlappyState::BIRD_X);
    for pipe in &state.pipes {
        let is_inside_x = bird_x + 1.0 >= pipe.x && bird_x <= pipe.x + f32::from(PIPE_WIDTH);
        if !is_inside_x {
            continue;
        }

        let gap_top = f32::from(pipe.gap_y);
        let gap_bottom = gap_top + f32::from(state.gap_height());
        if state.bird_y < gap_top || state.bird_y > gap_bottom {
            state.is_game_over = true;
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::geometry::TerminalSize;

    #[test]
    fn pipe_pass_increments_score_once() {
        let mut state = FlappyState::new(TerminalSize {
            width: 80,
            height: 24,
        });
        state.pipes.push(Pipe {
            x: f32::from(FlappyState::BIRD_X) - f32::from(PIPE_WIDTH) - 1.0,
            gap_y: 8,
            is_scored: false,
        });

        score_passed_pipes(&mut state);
        score_passed_pipes(&mut state);

        assert_eq!(state.score.0, 1);
    }
}
