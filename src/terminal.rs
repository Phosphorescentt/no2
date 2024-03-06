use crossterm::{execute, terminal::*};
use ratatui::prelude::*;
use std::io::{self, stdout, Stdout};

pub type TerminalInterface = Terminal<CrosstermBackend<Stdout>>;

pub fn init_terminal() -> io::Result<TerminalInterface> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore_terminal() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
