use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::env;
use std::io;
mod app;
use reqwest;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = crate::app::App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

fn get_data(path: String) -> String {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    // Run the async code using `block_on`
    let response = runtime.block_on(async {
        reqwest::get(path)
            .await // Await the response inside the async block
            .unwrap() // Unwrap the result
    });
    // Print the response as text
    let body = runtime.block_on(response.text()).unwrap();
    return body;
}
impl Widget for &crate::app::App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let args: Vec<String> = env::args().collect();
        let method = &args[1];
        let path = &args[2];

        let title = Line::from(" Lazy Curl ".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let json_string = get_data(path.to_string());
        // Create the Ratatui Text widget to display the result
        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            method.to_string().yellow(),
            " Path: ".into(),
            json_string.into(), // Display the formatted JSON string
        ])]);
        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
