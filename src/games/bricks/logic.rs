use crate::engine::input::Key;
use crate::games::bricks::state::BricksState;
use crate::types::geometry::Direction;

pub fn tick(state: &mut BricksState) {
    if state.is_game_over || state.is_complete {
        return;
    }

    if state.transition_ticks > 0 {
        state.transition_ticks = state.transition_ticks.saturating_sub(1);
        if state.transition_ticks == 0 && state.showing_clear {
            state.showing_clear = false;
            advance_level(state);
        }
        return;
    }

    apply_ball_motion(state);
    check_wall_collisions(state);
    check_paddle_collision(state);
    check_brick_collisions(state);
    check_dead_ball(state);
    check_level_clear(state);
}

pub fn handle_input(state: &mut BricksState, key: Key) {
    if state.is_game_over || state.is_complete || state.transition_ticks > 0 {
        return;
    }

    match key {
        Key::Dir(Direction::Left) => {
            state.paddle_col = state.paddle_col.saturating_sub(2);
        }
        Key::Dir(Direction::Right) => {
            state.paddle_col = state.paddle_col.saturating_add(2);
            let limit = state.bounds.width.saturating_sub(state.paddle_width);
            if state.paddle_col > limit {
                state.paddle_col = limit;
            }
        }
        _ => {}
    }
}

fn apply_ball_motion(state: &mut BricksState) {
    state.ball.x += state.ball.dx;
    state.ball.y += state.ball.dy;
}

fn check_wall_collisions(state: &mut BricksState) {
    if state.ball.x <= 0.0 {
        state.ball.x = 0.5;
        state.ball.dx = state.ball.dx.abs();
    } else if state.ball.x >= f32::from(state.bounds.width.saturating_sub(1)) {
        state.ball.x = f32::from(state.bounds.width.saturating_sub(1)) - 0.5;
        state.ball.dx = -state.ball.dx.abs();
    }

    if state.ball.y <= f32::from(BricksState::HUD_HEIGHT) {
        state.ball.y = f32::from(BricksState::HUD_HEIGHT) + 0.5;
        state.ball.dy = state.ball.dy.abs();
    }
}

fn check_paddle_collision(state: &mut BricksState) {
    let paddle_y = f32::from(state.paddle_row());

    if state.ball.dy > 0.0 && state.ball.y >= paddle_y && state.ball.y <= paddle_y + 1.0 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let bx = state.ball.x.round() as u16;
        if bx >= state.paddle_col && bx < state.paddle_col + state.paddle_width {
            state.ball.y = paddle_y - 0.5;
            state.ball.dy = -state.ball.dy.abs();

            let center = f32::from(state.paddle_col) + f32::from(state.paddle_width) / 2.0;
            let offset = (state.ball.x - center) / (f32::from(state.paddle_width) / 2.0);
            state.ball.dx = offset * 1.2;
        }
    }
}

fn check_brick_collisions(state: &mut BricksState) {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let ball_c = state.ball.x.round() as u16;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let ball_r = state.ball.y.round() as u16;

    let mut hit_idx: Option<usize> = None;

    for (i, brick) in state.bricks.iter().enumerate() {
        if !brick.is_alive {
            continue;
        }
        if ball_c >= brick.col
            && ball_c < brick.col + BricksState::BRICK_WIDTH
            && ball_r == brick.row
        {
            hit_idx = Some(i);
            break;
        }
    }

    if let Some(idx) = hit_idx {
        let brick = &mut state.bricks[idx];
        brick.hp = brick.hp.saturating_sub(1);
        if brick.hp == 0 {
            brick.is_alive = false;
        }
        state.ball.dy = -state.ball.dy;
        state.score.0 = state.score.0.saturating_add(10);
    }
}

fn check_dead_ball(state: &mut BricksState) {
    if state.ball.y > f32::from(state.bounds.height) {
        state.lives.0 = state.lives.0.saturating_sub(1);
        if state.lives.0 == 0 {
            state.is_game_over = true;
        } else {
            state.reset_ball();
        }
    }
}

fn check_level_clear(state: &mut BricksState) {
    if state.level.0 > 5 {
        return;
    }
    let any_alive = state.bricks.iter().any(|b| b.is_alive);
    if !any_alive {
        state.showing_clear = true;
        state.transition_ticks = 30;
    }
}

fn advance_level(state: &mut BricksState) {
    if state.level.0 >= 5 {
        state.is_complete = true;
        return;
    }
    state.level.0 = state.level.0.saturating_add(1);
    state.spawn_level_bricks();
    state.reset_ball();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::geometry::TerminalSize;

    #[test]
    fn brick_collision_deflects() {
        let mut state = BricksState::new(TerminalSize {
            width: 80,
            height: 24,
        });

        let target = state.bricks[0];
        state.ball.x = f32::from(target.col);
        state.ball.y = f32::from(target.row);
        state.ball.dy = -1.0;

        tick(&mut state);

        let checked = state.bricks[0];
        assert_ne!(checked.hp, target.hp);
    }
}
