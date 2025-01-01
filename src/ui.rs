use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders},
};

use crate::state::InputMode;
pub fn get_title() -> String {
    return "Lazy Curl".to_string();
}
// create main
pub fn main_block(input_mode: &InputMode) -> Block<'static> {
    let title = get_title();
    let instructions = Line::from(vec![
        Span::raw("Mode "),
        match input_mode {
            InputMode::Normal => Span::styled("Normal", Style::default().fg(Color::Yellow)),
            InputMode::Editing => Span::styled("Editing", Style::default().fg(Color::Yellow)),
        },
        Span::raw(" Quit ").into(),
        Span::raw("<Q> ".to_string()),
    ]);
    Block::default()
        .title(title)
        .title_bottom(instructions.centered())
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Thick)
        .border_style(Style::default().fg(Color::White))
}
