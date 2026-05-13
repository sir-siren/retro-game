use ratatui::style::Style;

use crate::engine::renderer::Buffer;
use crate::games::runner::state::RunnerState;
use crate::ui::theme;

const PLAYER_CAR: [&str; 3] = [
    "\u{2552}\u{2550}\u{2550}\u{2550}\u{2555}",
    "\u{2502}\u{25c8}\u{2588}\u{25c8}\u{2502}",
    "\u{2558}\u{2550}\u{2550}\u{2550}\u{255b}",
];
const ENEMY_CAR: [&str; 3] = [
    "\u{2554}\u{2550}\u{2550}\u{2550}\u{2557}",
    "\u{2551}\u{25c8}\u{2593}\u{25c8}\u{2551}",
    "\u{255a}\u{2550}\u{2550}\u{2550}\u{255d}",
];
const ENEMY_TRUCK: [&str; 3] = [
    "\u{2554}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2557}",
    "\u{2551}\u{25c8}\u{2593}\u{2593}\u{2593}\u{25c8}\u{2551}",
    "\u{255a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{255d}",
];

pub fn render(state: &RunnerState, buffer: &mut Buffer) {
    draw_hud(state, buffer);
    draw_road(state, buffer);
    draw_obstacles(state, buffer);
    draw_player(state, buffer);
    draw_overlays(state, buffer);
}

fn draw_hud(state: &RunnerState, buffer: &mut Buffer) {
    let score_text = format!("Score: {}", state.score.0);
    buffer.print(2, 0, &score_text, theme::style_hud());

    let speed_fraction = f32::from(state.speed.saturating_sub(30)) / 170.0;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let filled = (speed_fraction * 10.0) as u16;
    let bar: String = (0..10u16)
        .map(|i| if i < filled { '\u{2588}' } else { '\u{2591}' })
        .collect();
    let speed_text = format!("{}mph [{}]", state.speed, bar);
    buffer.print_right(0, &speed_text, 2, theme::style_hud());

    buffer.horizontal_line(
        1,
        0,
        state.bounds.width,
        '\u{2500}',
        Style::new().fg(theme::BORDER),
    );
}

fn draw_road(state: &RunnerState, buffer: &mut Buffer) {
    let top = RunnerState::hud_height();
    let bot = state.bounds.height.saturating_sub(1);
    let w = state.bounds.width;
    let road_style = Style::new().fg(theme::MUTED);

    for y in top..bot {
        buffer.place(0, y, '\u{2590}', road_style);
        buffer.place(1, y, '\u{2591}', road_style);
    }

    for y in top..bot {
        buffer.place(w.saturating_sub(2), y, '\u{2591}', road_style);
        buffer.place(w.saturating_sub(1), y, '\u{258c}', road_style);
    }

    let scroll = state.road_scroll % 8;
    let lane_style = Style::new().fg(theme::BORDER);
    for i in 0..RunnerState::lane_count().saturating_sub(1) {
        let y = state.lane_divider_y(i);
        for x in 2..w.saturating_sub(2) {
            if (x + scroll) % 8 < 3 {
                buffer.place(x, y, '\u{2500}', lane_style);
            }
        }
    }

    buffer.horizontal_line(bot, 0, w, '\u{2550}', road_style);
}

fn draw_obstacles(state: &RunnerState, buffer: &mut Buffer) {
    let enemy_style = Style::new().fg(theme::RUNNER_ENEMY);

    for car in &state.obstacles {
        if state.speed >= 80 {
            let trail_ch = if state.speed >= 140 {
                '\u{2592}'
            } else {
                '\u{2591}'
            };
            let trail_len = (state.speed / 50).min(4);
            let cy = state.lane_y(car.lane);
            let trail_style = Style::new().fg(theme::MUTED);
            for t in 1..=trail_len {
                let tx = car.col + car.width + t;
                if tx < state.bounds.width.saturating_sub(2) {
                    if cy > 0 {
                        buffer.place(tx, cy - 1, trail_ch, trail_style);
                    }
                    buffer.place(tx, cy, trail_ch, trail_style);
                    if cy + 1 < state.bounds.height {
                        buffer.place(tx, cy + 1, trail_ch, trail_style);
                    }
                }
            }
        }

        let sprites: &[&str; 3] = if car.width > 5 {
            &ENEMY_TRUCK
        } else {
            &ENEMY_CAR
        };
        let cy = state.lane_y(car.lane);
        for (offset, line) in sprites.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            let y = cy.saturating_sub(1) + offset as u16;
            if car.col < state.bounds.width {
                buffer.print(car.col, y, line, enemy_style);
            }
        }
    }
}

fn draw_player(state: &RunnerState, buffer: &mut Buffer) {
    let cy = state.lane_y(state.player_lane);
    let col = RunnerState::player_col();
    let player_style = Style::new().fg(theme::RUNNER_PLAYER);

    for (offset, line) in PLAYER_CAR.iter().enumerate() {
        #[allow(clippy::cast_possible_truncation)]
        let y = cy.saturating_sub(1) + offset as u16;
        buffer.print(col, y, line, player_style);
    }
}

fn draw_overlays(state: &RunnerState, buffer: &mut Buffer) {
    if state.is_game_over {
        let cx = state.bounds.width / 2;
        let cy = state.bounds.height / 2;
        let border_style = Style::new().fg(theme::DANGER);
        let text_style = theme::style_title();

        buffer.print(cx.saturating_sub(6), cy.saturating_sub(1), "\u{2554}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2557}", border_style);
        buffer.print(
            cx.saturating_sub(6),
            cy,
            "\u{2551} GAME  OVER \u{2551}",
            border_style,
        );
        buffer.print(cx.saturating_sub(6), cy + 1, "\u{255a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{255d}", border_style);

        let sub = format!("Final Score: {}", state.score.0);
        #[allow(clippy::cast_possible_truncation)]
        let sub_col = cx.saturating_sub(sub.len() as u16 / 2);
        buffer.print(sub_col, cy + 3, &sub, text_style);

        let retry_style = Style::new().fg(theme::SUCCESS);
        let quit_style = theme::style_muted();
        buffer.print(cx.saturating_sub(8), cy + 5, "[R] Retry", retry_style);
        buffer.print(cx.saturating_sub(8), cy + 6, "[Q] Quit to Menu", quit_style);
    }
}
