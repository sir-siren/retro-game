use crate::engine::input::Key;
use crate::games::rand::fast_rand;
use crate::games::runner::state::{RunnerState, TrafficCar};
use crate::types::geometry::Direction;

const MIN_SPEED: u16 = 30;
const MAX_SPEED: u16 = 200;
const SPEED_INCREMENT: u16 = 5;
const PLAYER_WIDTH: u16 = 5;

pub fn tick(state: &mut RunnerState) {
    if state.is_game_over {
        return;
    }

    state.tick = state.tick.wrapping_add(1);

    if state.tick % 10 == 0 {
        state.score.0 = state.score.0.saturating_add(u32::from(state.speed) / 10);
        check_level_up(state);
    }

    move_obstacles(state);
    check_collision(state);
    spawn_obstacles(state);
}

pub fn handle_input(state: &mut RunnerState, key: Key) {
    if state.is_game_over {
        return;
    }

    match key {
        Key::Dir(Direction::Up) => {
            if state.player_lane > 0 {
                state.player_lane -= 1;
            }
        }
        Key::Dir(Direction::Down) => {
            if state.player_lane < RunnerState::lane_count() - 1 {
                state.player_lane += 1;
            }
        }
        Key::Dir(Direction::Right) | Key::Action => {
            state.speed = (state.speed + SPEED_INCREMENT).min(MAX_SPEED);
        }
        Key::Dir(Direction::Left) => {
            state.speed = state.speed.saturating_sub(SPEED_INCREMENT).max(MIN_SPEED);
        }
        _ => {}
    }
}

fn check_level_up(state: &mut RunnerState) {
    let threshold = u32::from(state.level.0) * 500;
    if state.score.0 >= threshold && state.level.0 < 5 {
        state.level.0 = state.level.0.saturating_add(1);
    }
}

fn move_obstacles(state: &mut RunnerState) {
    let scroll = (state.speed / 30).max(1);
    for car in &mut state.obstacles {
        car.col = car.col.saturating_sub(scroll);
    }
    state.obstacles.retain(|car| car.col > 0);
}

fn check_collision(state: &mut RunnerState) {
    let player_left = state.player_col();
    let player_right = player_left + PLAYER_WIDTH;
    let player_lane = state.player_lane;

    for car in &state.obstacles {
        if car.lane != player_lane {
            continue;
        }
        let car_right = car.col + car.width;
        if player_left < car_right && player_right > car.col {
            state.is_game_over = true;
            return;
        }
    }
}

fn spawn_obstacles(state: &mut RunnerState) {
    let base_gap = 40u64.saturating_sub(u64::from(state.level.0) * 5).max(15);
    if state.tick % base_gap != 0 {
        return;
    }

    let min_spacing = state.bounds.width / 3;
    if state
        .obstacles
        .iter()
        .any(|c| c.col > state.bounds.width.saturating_sub(min_spacing))
    {
        return;
    }

    let r = fast_rand(state.tick ^ u64::from(state.score.0));
    #[allow(clippy::cast_possible_truncation)]
    let lane = (r % u64::from(RunnerState::lane_count())) as u8;

    let width = if r % 5 == 0 { 7 } else { 5 };

    state.obstacles.push(TrafficCar {
        lane,
        col: state.bounds.width.saturating_sub(2),
        width,
    });

    if state.level.0 >= 3 && r % 3 == 0 {
        let r2 = fast_rand(state.tick.wrapping_mul(7));
        #[allow(clippy::cast_possible_truncation)]
        let lane2 = (r2 % u64::from(RunnerState::lane_count())) as u8;
        if lane2 != lane {
            state.obstacles.push(TrafficCar {
                lane: lane2,
                col: state.bounds.width.saturating_sub(2),
                width: 5,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::geometry::TerminalSize;

    #[test]
    fn lane_switching() {
        let mut state = RunnerState::new(TerminalSize {
            width: 80,
            height: 24,
        });
        assert_eq!(state.player_lane, 1);

        handle_input(&mut state, Key::Dir(Direction::Up));
        assert_eq!(state.player_lane, 0);

        handle_input(&mut state, Key::Dir(Direction::Up));
        assert_eq!(state.player_lane, 0);

        handle_input(&mut state, Key::Dir(Direction::Down));
        assert_eq!(state.player_lane, 1);
    }

    #[test]
    fn speed_bounds() {
        let mut state = RunnerState::new(TerminalSize {
            width: 80,
            height: 24,
        });
        for _ in 0..100 {
            handle_input(&mut state, Key::Dir(Direction::Right));
        }
        assert_eq!(state.speed, MAX_SPEED);

        for _ in 0..100 {
            handle_input(&mut state, Key::Dir(Direction::Left));
        }
        assert_eq!(state.speed, MIN_SPEED);
    }

    #[test]
    fn collision_detection() {
        let mut state = RunnerState::new(TerminalSize {
            width: 80,
            height: 24,
        });
        state.player_lane = 2;
        state.obstacles.push(TrafficCar {
            lane: 2,
            col: state.player_col(),
            width: 5,
        });
        check_collision(&mut state);
        assert!(state.is_game_over);
    }
}
