use std::io;

mod app;
mod components;
mod error_handling;
mod screens;
mod terminal;
mod traits;
mod tui;

fn main() -> color_eyre::Result<()> {
    error_handling::install_hooks()?;
    let mut terminal = terminal::init_terminal()?;
    let app_result = app::App::default().run(&mut terminal);
    terminal::restore_terminal()?;
    app_result
}
