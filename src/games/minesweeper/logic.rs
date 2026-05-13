use std::collections::VecDeque;

use crate::engine::input::Key;
use crate::games::minesweeper::state::{Difficulty, MinesweeperState};
use crate::games::rand::fast_rand;
use crate::types::geometry::Direction;

pub const fn tick(state: &mut MinesweeperState) {
    if !state.is_game_over && !state.is_complete && state.is_generated {
        state.elapsed_ticks = state.elapsed_ticks.saturating_add(1);
    }
}

pub fn handle_input(state: &mut MinesweeperState, key: Key) {
    if state.is_game_over || state.is_complete {
        return;
    }

    match key {
        Key::Number(1) => state.set_difficulty(Difficulty::Easy),
        Key::Number(2) => state.set_difficulty(Difficulty::Medium),
        Key::Number(3) => state.set_difficulty(Difficulty::Hard),
        Key::Dir(Direction::Left) => state.cursor_x = state.cursor_x.saturating_sub(1),
        Key::Dir(Direction::Right) => {
            state.cursor_x = state.cursor_x.saturating_add(1).min(state.board_width - 1);
        }
        Key::Dir(Direction::Up) => state.cursor_y = state.cursor_y.saturating_sub(1),
        Key::Dir(Direction::Down) => {
            state.cursor_y = state.cursor_y.saturating_add(1).min(state.board_height - 1);
        }
        Key::Flag => toggle_flag(state),
        Key::Action => reveal_current(state),
        _ => {}
    }
}

fn toggle_flag(state: &mut MinesweeperState) {
    let idx = state.index(state.cursor_x, state.cursor_y);
    let cell = &mut state.cells[idx];
    if !cell.is_revealed {
        cell.is_flagged = !cell.is_flagged;
    }
}

fn reveal_current(state: &mut MinesweeperState) {
    if !state.is_generated {
        generate_board(state, state.cursor_x, state.cursor_y);
    }

    let idx = state.index(state.cursor_x, state.cursor_y);
    if state.cells[idx].is_flagged {
        return;
    }

    if state.cells[idx].has_mine {
        state.is_game_over = true;
        reveal_all_mines(state);
        return;
    }

    flood_reveal(state, state.cursor_x, state.cursor_y);
    check_complete(state);
}

fn generate_board(state: &mut MinesweeperState, safe_x: u16, safe_y: u16) {
    let mut placed = 0u16;
    let mut attempt = 0u64;

    while placed < state.mine_count {
        attempt = attempt.wrapping_add(1);
        let rand_value = fast_rand(attempt ^ u64::from(state.mine_count));
        let x = u16::try_from(rand_value % u64::from(state.board_width)).unwrap_or(0);
        let y = u16::try_from(fast_rand(rand_value) % u64::from(state.board_height)).unwrap_or(0);

        if is_safe_zone(safe_x, safe_y, x, y) {
            continue;
        }

        let idx = state.index(x, y);
        if state.cells[idx].has_mine {
            continue;
        }
        state.cells[idx].has_mine = true;
        placed = placed.saturating_add(1);
    }

    calculate_adjacency(state);
    state.is_generated = true;
}

fn is_safe_zone(safe_x: u16, safe_y: u16, x: u16, y: u16) -> bool {
    (i32::from(safe_x) - i32::from(x)).abs() <= 1 && (i32::from(safe_y) - i32::from(y)).abs() <= 1
}

fn calculate_adjacency(state: &mut MinesweeperState) {
    for y in 0..state.board_height {
        for x in 0..state.board_width {
            let idx = state.index(x, y);
            if state.cells[idx].has_mine {
                continue;
            }
            state.cells[idx].adjacent = neighbors(state, x, y)
                .iter()
                .filter(|(nx, ny)| state.cells[state.index(*nx, *ny)].has_mine)
                .count() as u8;
        }
    }
}

fn flood_reveal(state: &mut MinesweeperState, x: u16, y: u16) {
    let mut queue = VecDeque::from([(x, y)]);
    while let Some((cx, cy)) = queue.pop_front() {
        let idx = state.index(cx, cy);
        if state.cells[idx].is_revealed || state.cells[idx].is_flagged {
            continue;
        }

        state.cells[idx].is_revealed = true;
        state.score.0 = state.score.0.saturating_add(1);
        if state.cells[idx].adjacent != 0 {
            continue;
        }

        for (nx, ny) in neighbors(state, cx, cy) {
            let neighbor_idx = state.index(nx, ny);
            if !state.cells[neighbor_idx].is_revealed {
                queue.push_back((nx, ny));
            }
        }
    }
}

fn neighbors(state: &MinesweeperState, x: u16, y: u16) -> Vec<(u16, u16)> {
    let mut result = Vec::with_capacity(8);
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = i32::from(x) + dx;
            let ny = i32::from(y) + dy;
            if state.contains(nx, ny) {
                result.push((nx as u16, ny as u16));
            }
        }
    }
    result
}

fn reveal_all_mines(state: &mut MinesweeperState) {
    for cell in &mut state.cells {
        if cell.has_mine {
            cell.is_revealed = true;
        }
    }
}

fn check_complete(state: &mut MinesweeperState) {
    let hidden_safe = state
        .cells
        .iter()
        .any(|cell| !cell.has_mine && !cell.is_revealed);
    if !hidden_safe {
        state.is_complete = true;
        state.score.0 = state
            .score
            .0
            .saturating_add(u32::from(state.mine_count) * 5);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::geometry::TerminalSize;

    #[test]
    fn first_reveal_is_safe_and_generates_board() {
        let mut state = MinesweeperState::new(TerminalSize {
            width: 80,
            height: 24,
        });
        state.cursor_x = 4;
        state.cursor_y = 4;

        handle_input(&mut state, Key::Action);

        let idx = state.index(4, 4);
        assert!(state.is_generated);
        assert!(!state.cells[idx].has_mine);
        assert!(state.cells[idx].is_revealed);
    }
}
