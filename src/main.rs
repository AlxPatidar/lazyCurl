mod app;
mod state;
use state::State;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut state: State = state::State::default();
    let app_result = app::run(&mut terminal, &mut state);
    ratatui::restore();
    app_result
}
