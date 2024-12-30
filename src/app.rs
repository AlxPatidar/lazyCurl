use crate::state::State;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{layout::Rect, widgets::Block};
use ratatui::{DefaultTerminal, Frame};
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
    let paragraph = crate::ui::container_block(json_string);
    frame.render_widget(paragraph, area);
}
/// updates the application's state based on user input
fn handle_events(state: &mut State) -> io::Result<()> {
    match event::read()? {
        // it's important to check that the event is a key press event as
        // crossterm also emits key release and repeat events on Windows.
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(key_event, state)
        }
        _ => {}
    };
    Ok(())
}

fn handle_key_event(key_event: KeyEvent, state: &mut State) {
    match key_event.code {
        KeyCode::Char('q') => state.exit(),
        KeyCode::Left => state.decrement_counter(),
        KeyCode::Right => state.increment_counter(),
        _ => {}
    }
}
