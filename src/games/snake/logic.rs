use crate::engine::input::Key;
use crate::games::rand::fast_rand;
use crate::games::snake::state::SnakeState;
use crate::types::geometry::{Direction, Vec2};
use std::time::Duration;

pub fn tick(state: &mut SnakeState) {
    if state.is_game_over || state.is_complete {
        return;
    }

    state.tick_accumulator += Duration::from_millis(33);

    if state.tick_accumulator >= state.tick_rate {
        state.tick_accumulator -= state.tick_rate;
        step_snake(state);
    }
}

pub fn handle_input(state: &mut SnakeState, key: Key) {
    if state.is_game_over || state.is_complete {
        return;
    }

    if let Key::Dir(dir) = key {
        if state.input_queue.len() < 2 {
            state.input_queue.push_back(dir);
        }
    }
}

fn step_snake(state: &mut SnakeState) {
    if let Some(next_dir) = state.input_queue.pop_front() {
        if can_turn(state.direction, next_dir) {
            state.direction = next_dir;
        }
    }

    let head = state.segments[0];
    let next_head = match state.direction {
        Direction::Up => Vec2::new(head.x, head.y - 1),
        Direction::Down => Vec2::new(head.x, head.y + 1),
        Direction::Left => Vec2::new(head.x - 1, head.y),
        Direction::Right => Vec2::new(head.x + 1, head.y),
    };

    if next_head.x < 0
        || next_head.x > state.play_area_right()
        || next_head.y < state.play_area_top()
        || next_head.y > state.play_area_bottom()
    {
        state.is_game_over = true;
        return;
    }

    if state.segments.iter().any(|&s| s == next_head) {
        state.is_game_over = true;
        return;
    }

    state.segments.push_front(next_head);

    if next_head == state.food {
        state.score.0 = state.score.0.saturating_add(10);
        place_food(state);

        if state.tick_rate > Duration::from_millis(50) {
            state.tick_rate = state.tick_rate.saturating_sub(Duration::from_millis(2));
        }
    } else {
        state.segments.pop_back();
    }
}

fn can_turn(current: Direction, next: Direction) -> bool {
    !matches!(
        (current, next),
        (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left)
    )
}

fn place_food(state: &mut SnakeState) {
    let mut attempt: u64 = 0;
    loop {
        attempt += 1;
        #[allow(clippy::cast_possible_wrap)]
        let rx =
            (fast_rand(u64::from(state.score.0) ^ attempt) % u64::from(state.bounds.width)) as i32;
        #[allow(clippy::cast_possible_wrap)]
        let ry_raw = (fast_rand(u64::from(state.score.0).wrapping_mul(11) ^ attempt)
            % u64::from(state.bounds.height.saturating_sub(SnakeState::HUD_HEIGHT)))
            as i32;
        let ry = ry_raw + i32::from(SnakeState::HUD_HEIGHT);

        let candidate = Vec2::new(rx, ry);

        if !state.segments.contains(&candidate) {
            state.food = candidate;
            break;
        }

        if attempt > 1000 {
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
    fn snake_moves_right_by_default() {
        let mut state = SnakeState::new(TerminalSize {
            width: 80,
            height: 24,
        });
        state.tick_rate = Duration::from_millis(30);
        state.tick_accumulator = Duration::from_millis(30);

        let old_head = state.segments[0];
        assert_eq!(state.direction, Direction::Right);

        tick(&mut state);

        let new_head = state.segments[0];
        assert_eq!(new_head.x, old_head.x + 1);
        assert_eq!(new_head.y, old_head.y);
    }

    #[test]
    fn cannot_reverse_direction() {
        assert!(!can_turn(Direction::Up, Direction::Down));
        assert!(!can_turn(Direction::Left, Direction::Right));
        assert!(can_turn(Direction::Up, Direction::Left));
    }
}
