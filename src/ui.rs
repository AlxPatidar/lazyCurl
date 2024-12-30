use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
pub fn get_title() -> String {
    return "Lazy Curl".to_string();
}
// create main
pub fn main_block() -> Block<'static> {
    let title = get_title();
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Thick)
        .border_style(Style::default().fg(Color::White))
}

pub fn container_block(json: String) -> Paragraph<'static> {
    let title = Line::from(" Lazy Curl ");
    let instructions = Line::from(vec![
        Span::raw(" Quit ").into(),
        Span::raw("<Q> ".to_string()),
    ]);
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered());

    // Create the Ratatui Text widget to display the result
    let counter_text = vec![Line::from(vec![
        Span::raw("Value: "),
        Span::raw(json.to_string()),
    ])];
    Paragraph::new(counter_text).centered().block(block)
}
