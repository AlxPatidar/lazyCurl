use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};
pub fn get_title() -> String {
    return "Lazy Curl".to_string();
}
// create main
pub fn main_block() -> Block<'static> {
    let title = get_title();
    let instructions = Line::from(vec![
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
