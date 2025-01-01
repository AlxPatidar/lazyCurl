use crate::state::State;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::prelude::*;
use ratatui::widgets::{Borders, Paragraph};
use ratatui::{layout::Rect, widgets::Block};
use ratatui::{DefaultTerminal, Frame};
use serde_json::Value;
use std::env;
use std::io;

/// runs the application's main loop until the user quits
pub fn run(terminal: &mut DefaultTerminal, state: &mut State) -> io::Result<()> {
    while !state.exit {
        terminal.draw(|frame| draw_ui(frame, state))?;
        handle_events(state)?;
    }
    Ok(())
}

fn draw_ui(frame: &mut Frame, state: &mut State) {
    let area = frame.area();
    // fetch components from ui module
    let main_block: Block = crate::ui::main_block();
    app_widget(frame, main_block.inner(area), state);
    frame.render_widget(main_block, area);
}

fn app_widget(frame: &mut Frame, area: Rect, state: &mut State) {
    let args: Vec<String> = env::args().collect();
    #[allow(unused_variables)]
    let method = &args[1];
    let path = &args[2];
    state.set_method(method.to_string());
    state.set_path(path.to_string());
    // let user: String = state.get_data(path.to_string());
    // let method: &str = "GET";
    let json_string = state.get_data(path.to_string());
    // let paragraph = crate::ui::container_block(json_string);
    // frame.render_widget(paragraph, area);
    let layout_vartical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(8), Constraint::Percentage(92)])
        .split(area);
    let layout_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(8),
                Constraint::Percentage(84),
                Constraint::Percentage(8),
            ]
            .as_ref(),
        )
        .split(layout_vartical[0]);

    // Header
    frame.render_widget(
        Paragraph::new("Top").block(Block::new().borders(Borders::ALL)),
        layout_horizontal[0],
    );

    // Create a Paragraph widget to display the input text
    let input_paragraph = Paragraph::new(state.path.clone())
        .style(match state.input_mode {
            crate::state::InputMode::Normal => Style::default(),
            crate::state::InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::bordered().title("Input"));
    frame.render_widget(input_paragraph, layout_horizontal[1]);

    frame.render_widget(
        Paragraph::new("Send")
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL)),
        layout_horizontal[2],
    );
    // Body
    // Parse and pretty-print the JSON string
    let parsed_json: Value = serde_json::from_str(&json_string).unwrap();
    let pretty_json = serde_json::to_string_pretty(&parsed_json).unwrap();
    // Create a Text widget that will render the formatted JSON
    let text = Text::from(pretty_json);
    let block = Block::bordered();
    let para = Paragraph::new(text).block(block);
    // let paragraph = crate::ui::container_block(json_string);
    frame.render_widget(para, layout_vartical[1]);
}
/// updates the application's state based on user input
fn handle_events(state: &mut State) -> io::Result<()> {
    match event::read()? {
        // it's important to check that the event is a key press event as
        // crossterm also emits key release and repeat events on Windows.
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(key_event, state)
        }
        Event::Mouse(click_event) => handle_mouse_event(click_event, state),
        _ => {}
    };
    Ok(())
}

fn handle_key_event(key_event: KeyEvent, state: &mut State) {
    let mut input_text = String::from(state.path.clone());
    match key_event.code {
        KeyCode::Char('q') => state.exit(),
        KeyCode::Char('i') => {
            state.set_mode(crate::state::InputMode::Editing);
        }
        KeyCode::Esc => {
            state.set_mode(crate::state::InputMode::Normal);
        }
        KeyCode::Left => state.decrement_counter(),
        KeyCode::Right => state.increment_counter(),
        KeyCode::Backspace => {
            input_text.pop(); // Remove the last character
            state.set_path(input_text); // Update the input_text
        }
        KeyCode::Char(c) => {
            input_text.push(c);
            state.set_path(input_text); // Append the character to input_text
        }
        _ => {}
    }
}
fn handle_mouse_event(click_event: MouseEvent, _state: &mut State) {
    match click_event.kind {
        _ => {}
    }
}
