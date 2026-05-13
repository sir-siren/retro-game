use crate::engine::input::Key;
use crate::games::pong::state::PongState;
use crate::types::geometry::Direction;

const PADDLE_SPEED: f32 = 1.4;

pub fn tick(state: &mut PongState) {
    if state.is_game_over || state.is_complete {
        return;
    }

    state.tick = state.tick.wrapping_add(1);
    move_cpu(state);
    move_ball(state);
    check_wall_bounce(state);
    check_paddle_bounce(state);
    check_score(state);
}

pub fn handle_input(state: &mut PongState, key: Key) {
    if state.is_game_over || state.is_complete {
        return;
    }

    match key {
        Key::Number(2) if state.tick < 30 => state.is_two_player = true,
        Key::Dir(Direction::Up) => move_player(state, -PADDLE_SPEED),
        Key::Dir(Direction::Down) => move_player(state, PADDLE_SPEED),
        Key::Dir(Direction::Left) if state.is_two_player => move_cpu_manual(state, -PADDLE_SPEED),
        Key::Dir(Direction::Right) if state.is_two_player => move_cpu_manual(state, PADDLE_SPEED),
        _ => {}
    }
}

fn move_player(state: &mut PongState, delta: f32) {
    state.player_y = clamp_paddle(state.player_y + delta, state.bounds.height);
}

fn move_cpu_manual(state: &mut PongState, delta: f32) {
    state.cpu_y = clamp_paddle(state.cpu_y + delta, state.bounds.height);
}

fn move_cpu(state: &mut PongState) {
    if state.is_two_player {
        return;
    }

    let reaction = f32::from(state.level.0).mul_add(0.04, 0.25).min(0.75);
    let target = state.ball.y;
    if (target - state.cpu_y).abs() > 0.5 {
        state.cpu_y += (target - state.cpu_y).signum() * reaction;
        state.cpu_y = clamp_paddle(state.cpu_y, state.bounds.height);
    }
}

fn move_ball(state: &mut PongState) {
    state.ball.x += state.ball.dx;
    state.ball.y += state.ball.dy;
}

fn check_wall_bounce(state: &mut PongState) {
    if state.ball.y <= 2.0 {
        state.ball.y = 2.0;
        state.ball.dy = state.ball.dy.abs();
    }

    let bottom = f32::from(state.bounds.height.saturating_sub(2));
    if state.ball.y >= bottom {
        state.ball.y = bottom;
        state.ball.dy = -state.ball.dy.abs();
    }
}

fn check_paddle_bounce(state: &mut PongState) {
    let left_x = f32::from(PongState::left_paddle_x());
    if state.ball.dx < 0.0 && state.ball.x <= left_x + 1.0 {
        bounce_from_paddle(state, state.player_y, left_x + 1.0, 1.0);
    }

    let right_x = f32::from(state.right_paddle_x());
    if state.ball.dx > 0.0 && state.ball.x >= right_x - 1.0 {
        bounce_from_paddle(state, state.cpu_y, right_x - 1.0, -1.0);
    }
}

fn bounce_from_paddle(state: &mut PongState, paddle_y: f32, x: f32, direction: f32) {
    let half = f32::from(PongState::PADDLE_HEIGHT) / 2.0;
    if state.ball.y < paddle_y - half || state.ball.y > paddle_y + half {
        return;
    }

    let offset = (state.ball.y - paddle_y) / half;
    state.ball.x = x;
    state.ball.dx = (state.ball.dx.abs() + 0.03).min(1.8) * direction;
    state.ball.dy = offset * 0.9;
}

fn check_score(state: &mut PongState) {
    if state.ball.x < 1.0 {
        state.cpu_score = state.cpu_score.saturating_add(1);
        state.reset_ball(1.0);
    } else if state.ball.x > f32::from(state.bounds.width.saturating_sub(1)) {
        state.player_score = state.player_score.saturating_add(1);
        state.score.0 = state.score.0.saturating_add(100);
        state.level.0 = ((state.player_score / 2) + 1).min(10);
        state.reset_ball(-1.0);
    }

    if state.player_score >= PongState::WINNING_SCORE {
        state.is_complete = true;
    } else if state.cpu_score >= PongState::WINNING_SCORE {
        state.is_game_over = true;
    }
}

fn clamp_paddle(y: f32, height: u16) -> f32 {
    let half = f32::from(PongState::PADDLE_HEIGHT) / 2.0;
    y.clamp(2.0 + half, f32::from(height.saturating_sub(2)) - half)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::geometry::TerminalSize;

    #[test]
    fn scoring_on_right_side_increments_player_score() {
        let mut state = PongState::new(TerminalSize {
            width: 80,
            height: 24,
        });
        state.ball.x = 81.0;

        check_score(&mut state);

        assert_eq!(state.player_score, 1);
        assert_eq!(state.score.0, 100);
    }
}
