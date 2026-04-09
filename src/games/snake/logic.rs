use crate::engine::input::Key;
use crate::games::runner::logic::fast_rand; // Shared simple LCG
use crate::games::snake::state::SnakeState;
use crate::types::geometry::{Direction, Vec2};
use std::time::Duration;

/// Advances simulation state according to internal accumulator.
pub fn tick(state: &mut SnakeState) {
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

    // Since our outer GameLoop runs at fixed 33ms or so, we accumulate it here manually.
    // The spec states: Snake moving one cell per tick in current direction, but we have tick_rate.
    // We treat tick() as a 33ms update cycle in the engine, but Snake state needs to move at its tick_rate.
    state.tick_accumulator += Duration::from_millis(33);

    if state.tick_accumulator >= state.tick_rate {
        state.tick_accumulator -= state.tick_rate;
        step_snake(state);
    }
}

/// Binds next inputs.
pub fn handle_input(state: &mut SnakeState, key: Key) {
    if state.is_game_over || state.is_complete || state.transition_ticks > 0 {
        return;
    }

    if let Key::Dir(dir) = key {
        // Prevent 180 reverses, though this gets tricky if we allow multiple inputs in one tick.
        // Buffer them conceptually.
        if state.input_queue.len() < 2 {
            state.input_queue.push_back(dir);
        }
    }
}

/// Applies one raw step forward.
fn step_snake(state: &mut SnakeState) {
    if let Some(next_dir) = state.input_queue.pop_front() {
        if can_turn(state.direction, next_dir) {
            state.direction = next_dir;
        }
    }

    let head = state.segments[0];
    let mut next_head = head;

    match state.direction {
        Direction::Up => next_head.y -= 1,
        Direction::Down => next_head.y += 1,
        Direction::Left => next_head.x -= 1,
        Direction::Right => next_head.x += 1,
    }

    // Bounds checking (hit wall == death)
    #[allow(clippy::cast_possible_wrap)]
    let max_x = state.bounds.width as i32 - 1;
    #[allow(clippy::cast_possible_wrap)]
    let max_y = state.bounds.height as i32 - 1;

    // Row 0 is for HUB
    if next_head.x < 0 || next_head.x > max_x || next_head.y <= 0 || next_head.y > max_y {
        trigger_death(state);
        return;
    }

    // Body checking
    if state.segments.iter().any(|&s| s == next_head) {
        trigger_death(state);
        return;
    }

    // Interior walls
    if state.walls.iter().any(|&w| w == next_head) {
        trigger_death(state);
        return;
    }

    state.segments.push_front(next_head);

    // Food logic
    if next_head == state.food {
        state.score.0 = state.score.0.saturating_add(10);
        check_score_target(state);
        place_food(state);
    } else {
        state.segments.pop_back(); // Remove tail
    }
}

fn can_turn(current: Direction, n: Direction) -> bool {
    !matches!(
        (current, n),
        (Direction::Up, Direction::Down) |
        (Direction::Down, Direction::Up) |
        (Direction::Left, Direction::Right) |
        (Direction::Right, Direction::Left)
    )
}

fn trigger_death(state: &mut SnakeState) {
    state.lives.0 = state.lives.0.saturating_sub(1);
    if state.lives.0 == 0 {
        state.is_game_over = true;
    } else {
        state.reset_snake();
        place_food(state); // Move food
    }
}

fn check_score_target(state: &mut SnakeState) {
    // Target 50 for L1. Just make it progressive +50 per level.
    let target = 50 * u32::from(state.level.0);
    if state.score.0 >= target && state.level.0 < 5 {
        state.showing_clear = true;
        state.transition_ticks = 30; // 1s wait
    }
}

fn advance_level(state: &mut SnakeState) {
    state.level.0 = state.level.0.saturating_add(1);
    
    // Inject walls
    // For every level > 1, add 2 wall segments per level. Total walls = (level - 1) * 2
    state.walls.clear();
    let num_walls = (state.level.0.saturating_sub(1)) * 2;
    for i in 0..num_walls {
        // Just place them explicitly along an arc or use basic PRNG to scatter.
        #[allow(clippy::cast_possible_wrap)]
        let x = (fast_rand(u64::from(i) * 17) % state.bounds.width) as i32;
        #[allow(clippy::cast_possible_wrap)]
        let mut y = (fast_rand(u64::from(i) * 31) % state.bounds.height) as i32;
        if y == 0 { y = 1; }
        state.walls.push(Vec2::new(x, y));
    }

    state.tick_rate = state.tick_rate.saturating_sub(Duration::from_millis(15));
    state.reset_snake();
    place_food(state);
}

fn place_food(state: &mut SnakeState) {
    // Generate until empty cell
    let mut attempt = 0;
    loop {
        attempt += 1;
        #[allow(clippy::cast_possible_wrap)]
        let rx = (fast_rand(u64::from(state.score.0) ^ attempt) % state.bounds.width) as i32;
        #[allow(clippy::cast_possible_wrap)]
        let mut ry = (fast_rand((u64::from(state.score.0) * 11) ^ attempt) % state.bounds.height) as i32;
        if ry == 0 { ry = 1; }
        
        let candidate = Vec2::new(rx, ry);
        
        if !state.segments.contains(&candidate) && !state.walls.contains(&candidate) {
            state.food = candidate;
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::geometry::TerminalSize;

    #[test]
    fn test_snake_movement() {
        let mut state = SnakeState::new(TerminalSize { width: 80, height: 24 });
        state.tick_rate = Duration::from_millis(30);
        state.tick_accumulator = Duration::from_millis(30);
        
        let old_head = state.segments[0];
        assert_eq!(state.direction, Direction::Up);
        
        tick(&mut state);
        
        let new_head = state.segments[0];
        assert_eq!(new_head.y, old_head.y - 1);
        assert_eq!(new_head.x, old_head.x);
    }
}
