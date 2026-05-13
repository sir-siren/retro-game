use ratatui::style::{Modifier, Style};

use crate::engine::renderer::Buffer;
use crate::games::tetris::logic::{ghost_y, piece_cells};
use crate::games::tetris::state::{ActivePiece, TetrisState, Tetromino};
use crate::ui::theme;

const PIECE_COLORS: [ratatui::style::Color; 7] = [
    ratatui::style::Color::Cyan,
    theme::TITLE_GOLD,
    ratatui::style::Color::Magenta,
    theme::SUCCESS,
    theme::DANGER,
    ratatui::style::Color::Blue,
    ratatui::style::Color::Rgb(255, 140, 0),
];

pub fn render(state: &TetrisState, buffer: &mut Buffer) {
    draw_hud(state, buffer);
    draw_board_frame(state, buffer);
    draw_locked_blocks(state, buffer);
    draw_ghost(state, buffer);
    draw_piece(state, buffer);
    draw_side_panel(state, buffer);
    draw_overlay(state, buffer);
}

const fn origin(state: &TetrisState) -> (u16, u16) {
    let x = state
        .bounds
        .width
        .saturating_sub(TetrisState::BOARD_WIDTH.saturating_mul(2))
        / 2;
    (x, 3)
}

fn draw_hud(state: &TetrisState, buffer: &mut Buffer) {
    buffer.print(
        2,
        0,
        &format!("Score: {}", state.score.0),
        theme::style_hud(),
    );
    buffer.print_right(
        0,
        &format!("Level {}  Lines {}", state.level.0, state.lines_cleared),
        2,
        theme::style_muted(),
    );
    buffer.horizontal_line(
        1,
        0,
        state.bounds.width,
        '\u{2500}',
        Style::new().fg(theme::BORDER),
    );
}

fn draw_board_frame(state: &TetrisState, buffer: &mut Buffer) {
    let (ox, oy) = origin(state);
    for y in 0..=TetrisState::BOARD_HEIGHT {
        buffer.place(
            ox.saturating_sub(1),
            oy + y,
            '\u{2502}',
            Style::new().fg(theme::BORDER),
        );
        buffer.place(
            ox + TetrisState::BOARD_WIDTH * 2,
            oy + y,
            '\u{2502}',
            Style::new().fg(theme::BORDER),
        );
    }
    buffer.horizontal_line(
        oy + TetrisState::BOARD_HEIGHT,
        ox,
        ox + TetrisState::BOARD_WIDTH * 2,
        '\u{2500}',
        Style::new().fg(theme::BORDER),
    );
}

fn draw_locked_blocks(state: &TetrisState, buffer: &mut Buffer) {
    let (ox, oy) = origin(state);
    for y in 0..20usize {
        for x in 0..10usize {
            if let Some(color_idx) = state.board[y][x] {
                draw_block(buffer, ox + x as u16 * 2, oy + y as u16, color_idx, false);
            }
        }
    }
}

fn draw_ghost(state: &TetrisState, buffer: &mut Buffer) {
    let mut ghost = state.current;
    ghost.y = ghost_y(state);
    let (ox, oy) = origin(state);
    for (x, y) in piece_cells(ghost) {
        if y >= 0 {
            draw_ghost_block(buffer, ox + x as u16 * 2, oy + y as u16);
        }
    }
}

fn draw_piece(state: &TetrisState, buffer: &mut Buffer) {
    let (ox, oy) = origin(state);
    for (x, y) in piece_cells(state.current) {
        if y >= 0 {
            draw_block(
                buffer,
                ox + x as u16 * 2,
                oy + y as u16,
                state.current.kind.color_index(),
                true,
            );
        }
    }
}

fn draw_block(buffer: &mut Buffer, x: u16, y: u16, color_idx: u8, is_active: bool) {
    let color = PIECE_COLORS[usize::from(color_idx)];
    let style = if is_active {
        Style::new().fg(color).add_modifier(Modifier::BOLD)
    } else {
        Style::new().fg(color)
    };
    buffer.print(x, y, "\u{2588}\u{2588}", style);
}

fn draw_ghost_block(buffer: &mut Buffer, x: u16, y: u16) {
    buffer.print(x, y, "\u{2591}\u{2591}", Style::new().fg(theme::MUTED));
}

fn draw_side_panel(state: &TetrisState, buffer: &mut Buffer) {
    let (ox, oy) = origin(state);
    let panel_x = ox + TetrisState::BOARD_WIDTH * 2 + 4;
    buffer.print(panel_x, oy, "NEXT", theme::style_title());
    draw_preview(buffer, panel_x, oy + 2, state.next);
    buffer.print(panel_x, oy + 7, "HOLD", theme::style_title());
    if let Some(held) = state.hold {
        draw_preview(buffer, panel_x, oy + 9, held);
    }
}

fn draw_preview(buffer: &mut Buffer, x: u16, y: u16, kind: Tetromino) {
    let piece = ActivePiece {
        kind,
        rotation: 0,
        x: 0,
        y: 0,
    };
    for (cell_x, cell_y) in piece_cells(piece) {
        draw_block(
            buffer,
            x + cell_x as u16 * 2,
            y + cell_y as u16,
            kind.color_index(),
            false,
        );
    }
}

fn draw_overlay(state: &TetrisState, buffer: &mut Buffer) {
    if !state.is_game_over {
        return;
    }

    let y = state.bounds.height / 2;
    buffer.print(
        state.bounds.width.saturating_sub(9) / 2,
        y,
        "GAME OVER",
        theme::style_title(),
    );
    buffer.print(
        state.bounds.width.saturating_sub(9) / 2,
        y + 2,
        "[R] Retry",
        Style::new().fg(theme::SUCCESS),
    );
    buffer.print(
        state.bounds.width.saturating_sub(16) / 2,
        y + 3,
        "[Q] Quit to Menu",
        theme::style_muted(),
    );
}
