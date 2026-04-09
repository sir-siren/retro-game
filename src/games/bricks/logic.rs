use crate::engine::input::Key;
use crate::games::bricks::state::BricksState;
use crate::types::geometry::Direction;

/// Advances physical engine matching Bricks physics logic.
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

/// Applies raw sub-pixel velocity delta to tracking.
fn apply_ball_motion(state: &mut BricksState) {
    state.ball.x += state.ball.dx;
    state.ball.y += state.ball.dy;
}

/// Checks bounds overlapping left, right, and top edges.
fn check_wall_collisions(state: &mut BricksState) {
    if state.ball.x <= 0.0 {
        state.ball.x = 0.0;
        state.ball.dx = state.ball.dx.abs(); // force positive
    } else if state.ball.x >= f32::from(state.bounds.width.saturating_sub(1)) {
        state.ball.x = f32::from(state.bounds.width.saturating_sub(1));
        state.ball.dx = -state.ball.dx.abs(); // force negative
    }

    // Top bound
    if state.ball.y <= 1.0 {
        // reserve row 0 for HUD
        state.ball.y = 1.0;
        state.ball.dy = state.ball.dy.abs();
    }
}

/// Checks overlaps with paddle bounding box.
fn check_paddle_collision(state: &mut BricksState) {
    let paddle_y: f32 = f32::from(state.bounds.height.saturating_sub(2)); // Row just above bottom

    // Check if falling down and crossing the row threshold
    if state.ball.dy > 0.0 && state.ball.y >= paddle_y && state.ball.y <= paddle_y + 1.0 {
        let bx_idx: u16 = state.ball.x.round() as u16;
        if bx_idx >= state.paddle_col && bx_idx < state.paddle_col + 5 {
            // 5 chars wide
            state.ball.y = paddle_y - 0.5;
            state.ball.dy = -state.ball.dy.abs();

            // Reflect angle based on intersection distance from center
            let intersect_offset: f32 = (state.ball.x - (f32::from(state.paddle_col) + 2.0)) / 2.0;
            state.ball.dx = intersect_offset * 1.5;
        }
    }
}

/// AABB collision mapping against grid structure.
fn check_brick_collisions(state: &mut BricksState) {
    let ball_c: u16 = state.ball.x.round() as u16;
    let ball_r: u16 = state.ball.y.round() as u16;

    let mut hit_idx: Option<usize> = None;

    for (i, brick) in state.bricks.iter().enumerate() {
        if !brick.is_alive {
            continue;
        }

        // Brick is 4 wide [##]
        if ball_c >= brick.col && ball_c < brick.col + 4 && ball_r == brick.row {
            hit_idx = Some(i);
            break;
        }
    }

    if let Some(idx) = hit_idx {
        let brick: &mut super::state::Brick = &mut state.bricks[idx];
        brick.hp = brick.hp.saturating_sub(1);
        if brick.hp == 0 {
            brick.is_alive = false;
        }

        // Deflect back
        state.ball.dy = -state.ball.dy;
        state.score.0 = state.score.0.saturating_add(10);
    }
}

/// Flags life loss and repositions state.
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

/// Verifies total clear.
fn check_level_clear(state: &mut BricksState) {
    if state.level.0 > 5 {
        return; // Finished
    }
    let any_alive: bool = state
        .bricks
        .iter()
        .any(|b: &super::state::Brick| b.is_alive);
    if !any_alive {
        state.showing_clear = true;
        state.transition_ticks = 30; // 1s at ~30fps
    }
}

/// Transitions constraints to next matrix configuration.
fn advance_level(state: &mut BricksState) {
    if state.level.0 >= 5 {
        state.is_complete = true;
        return;
    }
    state.level.0 = state.level.0.saturating_add(1);
    state.spawn_level_bricks();
    state.reset_ball();
}

/// Captures user displacement events.
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
            let limit: u16 = state.bounds.width.saturating_sub(5);
            if state.paddle_col > limit {
                state.paddle_col = limit;
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::geometry::TerminalSize;

    #[test]
    fn test_brick_collision() {
        let mut state: BricksState = BricksState::new(TerminalSize {
            width: 80,
            height: 24,
        });

        let target: crate::games::bricks::state::Brick = state.bricks[0];
        // Overlap exactly
        state.ball.x = f32::from(target.col);
        state.ball.y = f32::from(target.row);
        state.ball.dy = -1.0;

        tick(&mut state);

        let checked: crate::games::bricks::state::Brick = state.bricks[0];
        assert_ne!(checked.hp, target.hp);
        assert_eq!(state.ball.dy, 1.0); // Deflected
    }
}
