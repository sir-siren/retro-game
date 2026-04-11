use crate::engine::renderer::Buffer;
use crate::games::runner::state::RunnerState;

const PLAYER_CAR: [&str; 3] = ["┌═══┐", "│ █ │", "└═══┘"];
const ENEMY_CAR: [&str; 3] = ["╔═══╗", "║ ▓ ║", "╚═══╝"];
const ENEMY_TRUCK: [&str; 3] = ["╔═════╗", "║ ▓▓▓ ║", "╚═════╝"];

pub fn render(state: &RunnerState, buffer: &mut Buffer) {
    draw_hud(state, buffer);
    draw_lanes(state, buffer);
    draw_obstacles(state, buffer);
    draw_player(state, buffer);
    draw_overlays(state, buffer);
}

fn draw_hud(state: &RunnerState, buffer: &mut Buffer) {
    let score_text: String = format!("Score: {}", state.score.0);
    buffer.print(2, 0, &score_text);

    let speed_text = format!("{}mph", state.speed);
    buffer.print_right(0, &speed_text, 2);

    buffer.horizontal_line(1, 0, state.bounds.width, '─');
}

fn draw_lanes(state: &RunnerState, buffer: &mut Buffer) {
    for i in 0..RunnerState::lane_count().saturating_sub(1) {
        let y: u16 = state.lane_divider_y(i);
        buffer.dashed_line(y, 0, state.bounds.width);
    }

    buffer.horizontal_line(
        state.bounds.height.saturating_sub(1),
        0,
        state.bounds.width,
        '═',
    );
}

fn draw_obstacles(state: &RunnerState, buffer: &mut Buffer) {
    for car in &state.obstacles {
        let center_y: u16 = state.lane_y(car.lane);
        let sprites: &[&str; 3] = if car.width > 5 {
            &ENEMY_TRUCK
        } else {
            &ENEMY_CAR
        };

        for (offset, line) in sprites.iter().enumerate() {
            let y: u16 = center_y.saturating_sub(1) + offset as u16;
            if car.col < state.bounds.width {
                buffer.print(car.col, y, line);
            }
        }
    }
}

fn draw_player(state: &RunnerState, buffer: &mut Buffer) {
    let center_y: u16 = state.lane_y(state.player_lane);
    let col: u16 = state.player_col();

    for (offset, line) in PLAYER_CAR.iter().enumerate() {
        let y: u16 = center_y.saturating_sub(1) + offset as u16;
        buffer.print(col, y, line);
    }
}

fn draw_overlays(state: &RunnerState, buffer: &mut Buffer) {
    if state.is_game_over {
        let cx: u16 = state.bounds.width / 2;
        let cy: u16 = state.bounds.height / 2;

        buffer.print(cx.saturating_sub(6), cy.saturating_sub(1), "╔════════════╗");
        buffer.print(cx.saturating_sub(6), cy, "║ GAME  OVER ║");
        buffer.print(cx.saturating_sub(6), cy + 1, "╚════════════╝");

        let sub = format!("Final Score: {}", state.score.0);
        #[allow(clippy::cast_possible_truncation)]
        let sx = cx.saturating_sub(sub.len() as u16 / 2);
        buffer.print(sx, cy + 3, &sub);

        buffer.print(cx.saturating_sub(10), cy + 5, "Press any key to exit");
    }
}
