use std::io;

mod app;
mod components;
mod screens;
mod terminal;
mod traits;
mod tui;

fn main() -> io::Result<()> {
    let mut terminal = terminal::init_terminal()?;
    let app_result = app::App::default().run(&mut terminal);
    terminal::restore_terminal()?;
    app_result
}
