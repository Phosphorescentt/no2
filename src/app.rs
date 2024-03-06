use crate::screens;
use crate::tui;
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
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        match self.state.screen {
            Screen::Home(home_state) => home_state.render_frame(frame),
            Screen::Settings(settings_state) => settings_date.render_frame(frame),
            Screen::Game(game_state) => game_state.render_frame(frame),
            Screen::End(end_state) => end_state.render_frame(frame),
        }
        tui::render_tui(frame, &self.state);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match self.state.screen {
            Screen::Home(home_state) => home_state.handle_events(),
            Screen::Settings(settings_state) => settings_state.handle_events(),
            Screen::Game(game_state) => game_state.handle_events(),
            Screen::End(end_state) => end_state.handle_events(),
        }
        todo!()
    }
}
