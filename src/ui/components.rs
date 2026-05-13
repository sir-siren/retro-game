use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Gauge, Paragraph};

use crate::ui::theme;

#[must_use]
pub fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical_margin = area.height.saturating_sub(height) / 2;
    let horizontal_margin = area.width.saturating_sub(width) / 2;

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(vertical_margin),
            Constraint::Length(height.min(area.height)),
            Constraint::Min(0),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(horizontal_margin),
            Constraint::Length(width.min(area.width)),
            Constraint::Min(0),
        ])
        .split(vertical[1]);

    horizontal[1]
}

pub fn render_game_over_popup(
    frame: &mut Frame<'_>,
    area: Rect,
    score: u32,
    best: u32,
    is_new_best: bool,
) {
    let title = if is_new_best {
        " New Best "
    } else {
        " Game Over "
    };
    let popup_area = centered_rect(32, 9, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::style_danger())
        .title(title)
        .title_style(theme::style_title());

    let text = vec![
        Line::from(Span::styled(
            format!("Score: {score}"),
            Style::new().fg(theme::WHITE),
        )),
        Line::from(Span::styled(
            format!("Best:  {best}"),
            Style::new().fg(theme::HIGHLIGHT),
        )),
        Line::from(""),
        Line::from(Span::styled("[R] Retry", Style::new().fg(theme::SUCCESS))),
        Line::from(Span::styled(
            "[Q] Quit to Menu",
            Style::new().fg(theme::MUTED),
        )),
    ];

    frame.render_widget(Clear, popup_area);
    frame.render_widget(
        Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(block),
        popup_area,
    );
}

pub fn render_countdown(frame: &mut Frame<'_>, area: Rect, count: u8) {
    let popup_area = centered_rect(13, 5, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::style_border())
        .style(Style::new().bg(theme::BG_PANEL));
    let text = Paragraph::new(Line::from(Span::styled(
        count.to_string(),
        Style::new()
            .fg(theme::TITLE_GOLD)
            .bg(theme::BG_PANEL)
            .add_modifier(Modifier::BOLD),
    )))
    .alignment(Alignment::Center)
    .block(block);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(text, popup_area);
}

pub fn render_pause_overlay(frame: &mut Frame<'_>, area: Rect) {
    let popup_area = centered_rect(22, 5, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::style_border())
        .style(Style::new().bg(theme::BG_PANEL));
    let text = Paragraph::new(Line::from(Span::styled("PAUSED", theme::style_title())))
        .alignment(Alignment::Center)
        .block(block);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(text, popup_area);
}

pub fn render_level_up(frame: &mut Frame<'_>, area: Rect, level: u8) {
    let popup_area = centered_rect(24, 5, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(theme::SUCCESS))
        .style(Style::new().bg(theme::BG_PANEL));
    let text = Paragraph::new(Line::from(Span::styled(
        format!("LEVEL {level}"),
        Style::new()
            .fg(theme::SUCCESS)
            .bg(theme::BG_PANEL)
            .add_modifier(Modifier::BOLD),
    )))
    .alignment(Alignment::Center)
    .block(block);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(text, popup_area);
}

pub fn render_hud(
    frame: &mut Frame<'_>,
    area: Rect,
    score: u32,
    level: u8,
    progress: f64,
    extra_spans: &[Span<'_>],
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20),
            Constraint::Min(20),
            Constraint::Length(24),
        ])
        .split(area);

    let score_text = Paragraph::new(Line::from(Span::styled(
        format!("Score {score}"),
        theme::style_hud(),
    )));
    frame.render_widget(score_text, chunks[0]);

    let gauge = Gauge::default()
        .gauge_style(Style::new().fg(theme::HIGHLIGHT).bg(theme::BG_PANEL))
        .ratio(progress.clamp(0.0, 1.0))
        .label(Span::styled(
            format!("LVL {level}"),
            Style::new().fg(theme::WHITE),
        ));
    frame.render_widget(gauge, chunks[1]);

    let extras = Paragraph::new(Line::from(extra_spans.to_vec()))
        .alignment(Alignment::Right)
        .style(theme::style_muted());
    frame.render_widget(extras, chunks[2]);
}
