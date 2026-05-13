use ratatui::style::{Modifier, Style};

use crate::engine::renderer::Buffer;
use crate::games::minesweeper::state::{Difficulty, MineCell, MinesweeperState};
use crate::ui::theme;

pub fn render(state: &MinesweeperState, buffer: &mut Buffer) {
    draw_hud(state, buffer);
    draw_board(state, buffer);
    draw_overlay(state, buffer);
}

fn draw_hud(state: &MinesweeperState, buffer: &mut Buffer) {
    let remaining = state.mine_count.saturating_sub(state.flags_used());
    let elapsed = state.elapsed_ticks / 30;
    buffer.print(
        2,
        0,
        &format!("Mines: {remaining}  Time: {elapsed}s"),
        theme::style_hud(),
    );
    buffer.print_right(
        0,
        difficulty_label(state.difficulty),
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

const fn difficulty_label(difficulty: Difficulty) -> &'static str {
    match difficulty {
        Difficulty::Easy => "1 Easy",
        Difficulty::Medium => "2 Medium",
        Difficulty::Hard => "3 Hard",
    }
}

fn draw_board(state: &MinesweeperState, buffer: &mut Buffer) {
    let offset_x = state
        .bounds
        .width
        .saturating_sub(state.board_width.saturating_mul(2))
        / 2;
    let offset_y = 3;

    for y in 0..state.board_height {
        for x in 0..state.board_width {
            let cell = state.cells[state.index(x, y)];
            let screen_x = offset_x + x * 2;
            let screen_y = offset_y + y;
            let is_cursor = x == state.cursor_x && y == state.cursor_y;
            draw_cell(buffer, screen_x, screen_y, cell, is_cursor);
        }
    }
}

fn draw_cell(buffer: &mut Buffer, x: u16, y: u16, cell: MineCell, is_cursor: bool) {
    let cursor_style = if is_cursor {
        Style::new().bg(theme::HIGHLIGHT).fg(theme::BG_MAIN)
    } else {
        Style::new()
    };

    if cell.is_flagged && !cell.is_revealed {
        buffer.print(x, y, "F ", cursor_style.fg(theme::DANGER));
        return;
    }

    if !cell.is_revealed {
        buffer.print(x, y, "\u{25a0} ", cursor_style.fg(theme::MUTED));
        return;
    }

    if cell.has_mine {
        buffer.print(
            x,
            y,
            "* ",
            cursor_style.fg(theme::DANGER).add_modifier(Modifier::BOLD),
        );
        return;
    }

    if cell.adjacent == 0 {
        buffer.print(x, y, ". ", cursor_style.fg(theme::MUTED));
    } else {
        let idx = usize::from(cell.adjacent.saturating_sub(1).min(7));
        buffer.print(
            x,
            y,
            &format!("{} ", cell.adjacent),
            cursor_style.fg(theme::MINE_COLORS[idx]),
        );
    }
}

fn draw_overlay(state: &MinesweeperState, buffer: &mut Buffer) {
    if !state.is_game_over && !state.is_complete {
        return;
    }

    let msg = if state.is_complete { "CLEARED" } else { "BOOM" };
    let y = state.bounds.height / 2;
    buffer.print(
        state.bounds.width.saturating_sub(msg.len() as u16) / 2,
        y,
        msg,
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
