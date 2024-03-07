use crate::screens;
use crate::traits::{EventHandler, FrameRenderer};
use crossterm::event::{self, Event, KeyCode};
use ratatui::Frame;
use std::io;

pub struct App {
    pub state: GlobalState,
    pub exit: bool,
}

pub struct GlobalState {
    pub screen: Screen,
}

pub enum Screen {
    // Home => Settings => Playing => End => Home,
    Home(screens::home::HomeState),
    Settings(screens::settings::SettingsState),
    Game(screens::game::GameState),
    End(screens::end::EndState),
}

impl Default for App {
    fn default() -> Self {
        App {
            state: GlobalState {
                screen: Screen::Home(screens::home::HomeState::default()),
            },
            exit: false,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut crate::terminal::TerminalInterface) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame).unwrap())?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) -> io::Result<()> {
        match &self.state.screen {
            Screen::Home(home_state) => home_state.render_frame(frame),
            Screen::Settings(settings_state) => settings_state.render_frame(frame),
            Screen::Game(game_state) => game_state.render_frame(frame),
            Screen::End(end_state) => end_state.render_frame(frame),
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        // Read any events that come from the terminal and pass them off
        // to the relevant handlers.
        let event = event::read()?;

        // Always catch 'q' as quit regardless of screen.
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('q') {
                self.exit = true;
                return Ok(());
            };
        }

        let exit = match &mut self.state.screen {
            Screen::Home(home_state) => home_state.handle_events(event),
            Screen::Settings(settings_state) => settings_state.handle_events(event),
            Screen::Game(game_state) => game_state.handle_events(event),
            Screen::End(end_state) => end_state.handle_events(event),
        }?;

        if exit {
            self.exit = true
        }

        Ok(())
    }
}
