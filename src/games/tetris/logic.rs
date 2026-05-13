use std::time::Duration;

use crate::engine::input::Key;
use crate::games::rand::fast_rand;
use crate::games::tetris::state::{ActivePiece, TetrisState, Tetromino};
use crate::types::geometry::Direction;

pub fn tick(state: &mut TetrisState) {
    if state.is_game_over || state.is_complete {
        return;
    }

    state.tick = state.tick.wrapping_add(1);
    state.tick_accumulator += Duration::from_millis(33);
    if state.tick_accumulator < state.tick_rate {
        return;
    }
    state.tick_accumulator -= state.tick_rate;
    soft_drop(state);
}

pub fn handle_input(state: &mut TetrisState, key: Key) {
    if state.is_game_over || state.is_complete {
        return;
    }

    match key {
        Key::Dir(Direction::Left) => try_move(state, -1, 0),
        Key::Dir(Direction::Right) => try_move(state, 1, 0),
        Key::Dir(Direction::Down) => soft_drop(state),
        Key::Dir(Direction::Up) | Key::Action => rotate(state),
        Key::Hold => hold_piece(state),
        _ => {}
    }
}

#[must_use]
pub fn ghost_y(state: &TetrisState) -> i32 {
    let mut piece = state.current;
    while is_valid(state, piece.x, piece.y + 1, piece.rotation, piece.kind) {
        piece.y += 1;
    }
    piece.y
}

#[must_use]
pub fn piece_cells(piece: ActivePiece) -> [(i32, i32); 4] {
    let raw = base_cells(piece.kind);
    raw.map(|(x, y)| rotate_cell(x, y, piece.rotation))
        .map(|(x, y)| (x + piece.x, y + piece.y))
}

fn try_move(state: &mut TetrisState, dx: i32, dy: i32) {
    let piece = state.current;
    if is_valid(
        state,
        piece.x + dx,
        piece.y + dy,
        piece.rotation,
        piece.kind,
    ) {
        state.current.x += dx;
        state.current.y += dy;
    }
}

fn soft_drop(state: &mut TetrisState) {
    let piece = state.current;
    if is_valid(state, piece.x, piece.y + 1, piece.rotation, piece.kind) {
        state.current.y += 1;
    } else {
        lock_piece(state);
    }
}

fn rotate(state: &mut TetrisState) {
    if state.current.kind == Tetromino::O {
        return;
    }
    let next_rotation = (state.current.rotation + 1) % 4;
    for kick in [0, -1, 1, -2, 2] {
        if is_valid(
            state,
            state.current.x + kick,
            state.current.y,
            next_rotation,
            state.current.kind,
        ) {
            state.current.x += kick;
            state.current.rotation = next_rotation;
            return;
        }
    }
}

fn hold_piece(state: &mut TetrisState) {
    if !state.can_hold {
        return;
    }
    state.can_hold = false;
    let held = state.hold.replace(state.current.kind);
    match held {
        Some(kind) => {
            state.current = spawn_piece(kind);
            if !is_valid_current(state) {
                state.is_game_over = true;
            }
        }
        None => spawn_next(state),
    }
}

fn lock_piece(state: &mut TetrisState) {
    for (x, y) in piece_cells(state.current) {
        if y < 0 {
            state.is_game_over = true;
            return;
        }
        if (0..10).contains(&x) && (0..20).contains(&y) {
            state.board[y as usize][x as usize] = Some(state.current.kind.color_index());
        }
    }
    let cleared = clear_lines(state);
    award_score(state, cleared);
    spawn_next(state);
}

fn spawn_next(state: &mut TetrisState) {
    state.current = spawn_piece(state.next);
    state.seed = fast_rand(state.seed ^ state.tick);
    state.next = random_piece(state.seed);
    state.can_hold = true;
    if !is_valid_current(state) {
        state.is_game_over = true;
    }
}

fn clear_lines(state: &mut TetrisState) -> u8 {
    let mut write_row = 19usize;
    let mut cleared = 0u8;
    for read_row in (0..20usize).rev() {
        if state.board[read_row].iter().all(Option::is_some) {
            cleared = cleared.saturating_add(1);
            continue;
        }
        state.board[write_row] = state.board[read_row];
        write_row = write_row.saturating_sub(1);
    }
    for row in 0..=write_row {
        state.board[row] = [None; 10];
    }
    state.lines_cleared = state.lines_cleared.saturating_add(u16::from(cleared));
    state.level.0 = (state.lines_cleared / 10 + 1).min(10) as u8;
    state.tick_rate = Duration::from_millis(700u64.saturating_sub(u64::from(state.level.0) * 45));
    cleared
}

fn award_score(state: &mut TetrisState, cleared: u8) {
    let base = match cleared {
        1 => 100,
        2 => 300,
        3 => 500,
        4 => 800,
        _ => 0,
    };
    state.score.0 = state
        .score
        .0
        .saturating_add(base * u32::from(state.level.0));
}

fn is_valid_current(state: &TetrisState) -> bool {
    is_valid(
        state,
        state.current.x,
        state.current.y,
        state.current.rotation,
        state.current.kind,
    )
}

fn is_valid(state: &TetrisState, x: i32, y: i32, rotation: u8, kind: Tetromino) -> bool {
    let piece = ActivePiece {
        kind,
        rotation,
        x,
        y,
    };
    piece_cells(piece).iter().all(|(cell_x, cell_y)| {
        if *cell_x < 0 || *cell_x >= 10 || *cell_y >= 20 {
            return false;
        }
        *cell_y < 0 || state.board[*cell_y as usize][*cell_x as usize].is_none()
    })
}

const fn spawn_piece(kind: Tetromino) -> ActivePiece {
    ActivePiece {
        kind,
        rotation: 0,
        x: 3,
        y: -1,
    }
}

const fn random_piece(seed: u64) -> Tetromino {
    match seed % 7 {
        0 => Tetromino::I,
        1 => Tetromino::O,
        2 => Tetromino::T,
        3 => Tetromino::S,
        4 => Tetromino::Z,
        5 => Tetromino::J,
        _ => Tetromino::L,
    }
}

const fn base_cells(kind: Tetromino) -> [(i32, i32); 4] {
    match kind {
        Tetromino::I => [(0, 1), (1, 1), (2, 1), (3, 1)],
        Tetromino::O => [(1, 0), (2, 0), (1, 1), (2, 1)],
        Tetromino::T => [(1, 0), (0, 1), (1, 1), (2, 1)],
        Tetromino::S => [(1, 0), (2, 0), (0, 1), (1, 1)],
        Tetromino::Z => [(0, 0), (1, 0), (1, 1), (2, 1)],
        Tetromino::J => [(0, 0), (0, 1), (1, 1), (2, 1)],
        Tetromino::L => [(2, 0), (0, 1), (1, 1), (2, 1)],
    }
}

const fn rotate_cell(x: i32, y: i32, rotation: u8) -> (i32, i32) {
    match rotation % 4 {
        0 => (x, y),
        1 => (3 - y, x),
        2 => (3 - x, 3 - y),
        _ => (y, 3 - x),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::geometry::TerminalSize;

    #[test]
    fn ghost_piece_drops_to_bottom_on_empty_board() {
        let state = TetrisState::new(TerminalSize {
            width: 80,
            height: 24,
        });

        assert!(ghost_y(&state) > state.current.y);
    }

    #[test]
    fn line_clear_awards_single_line_points() {
        let mut state = TetrisState::new(TerminalSize {
            width: 80,
            height: 24,
        });
        state.board[19] = [Some(0); 10];

        let cleared = clear_lines(&mut state);
        award_score(&mut state, cleared);

        assert_eq!(cleared, 1);
        assert_eq!(state.score.0, 100);
    }
}
