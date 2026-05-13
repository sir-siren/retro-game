use ratatui::style::Style;

use crate::engine::renderer::Buffer;
use crate::games::invaders::state::InvadersState;
use crate::ui::theme;

pub fn render(state: &InvadersState, buffer: &mut Buffer) {
    draw_hud(state, buffer);
    draw_aliens(state, buffer);
    draw_shields(state, buffer);
    draw_bullets(state, buffer);
    draw_player(state, buffer);
    draw_overlay(state, buffer);
}

fn draw_hud(state: &InvadersState, buffer: &mut Buffer) {
    buffer.print(
        2,
        0,
        &format!("Score: {}", state.score.0),
        theme::style_hud(),
    );
    buffer.print_right(
        0,
        &format!("Level {}", state.level.0),
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

fn draw_aliens(state: &InvadersState, buffer: &mut Buffer) {
    for alien in &state.aliens {
        if !alien.is_alive || alien.x < 0 || alien.y < 0 {
            continue;
        }
        let color = match alien.row {
            0 => theme::TITLE_GOLD,
            1 | 2 => theme::HIGHLIGHT,
            _ => theme::SUCCESS,
        };
        let sprite = match alien.row {
            0 => "W",
            1 | 2 => "M",
            _ => "A",
        };
        buffer.print(
            alien.x as u16,
            alien.y as u16,
            sprite,
            Style::new().fg(color),
        );
    }
}

fn draw_shields(state: &InvadersState, buffer: &mut Buffer) {
    for shield in &state.shields {
        if shield.hp == 0 || shield.x < 0 || shield.y < 0 {
            continue;
        }
        let ch = if shield.hp == 2 {
            '\u{2588}'
        } else {
            '\u{2592}'
        };
        buffer.place(
            shield.x as u16,
            shield.y as u16,
            ch,
            Style::new().fg(theme::SUCCESS),
        );
    }
}

fn draw_bullets(state: &InvadersState, buffer: &mut Buffer) {
    for bullet in &state.player_bullets {
        if bullet.x >= 0 && bullet.y >= 0 {
            buffer.place(
                bullet.x as u16,
                bullet.y as u16,
                '|',
                Style::new().fg(theme::WHITE),
            );
        }
    }
    for bullet in &state.alien_bullets {
        if bullet.x >= 0 && bullet.y >= 0 {
            buffer.place(
                bullet.x as u16,
                bullet.y as u16,
                '!',
                Style::new().fg(theme::DANGER),
            );
        }
    }
}

fn draw_player(state: &InvadersState, buffer: &mut Buffer) {
    let y = state.bounds.height.saturating_sub(3);
    buffer.print(
        state.player_x.saturating_sub(1) as u16,
        y,
        "/A\\",
        Style::new().fg(theme::HIGHLIGHT),
    );
}

fn draw_overlay(state: &InvadersState, buffer: &mut Buffer) {
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
