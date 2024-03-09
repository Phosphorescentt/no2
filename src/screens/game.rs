use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::Paragraph,
};

use crate::traits::{EventHandler, FrameRenderer, ScreenMessage};

#[derive(Clone, Copy)]
pub struct GameSettings {
    pub size: u8,
}

pub struct GameState {
    pub settings: GameSettings,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            settings: GameSettings { size: 10 },
        }
    }
}

impl From<GameSettings> for GameState {
    fn from(settings: GameSettings) -> Self {
        GameState { settings }
    }
}

impl EventHandler for GameState {
    fn handle_events(
        &mut self,
        event: crossterm::event::Event,
    ) -> std::io::Result<crate::traits::ScreenMessage> {
        Ok(ScreenMessage::Noop)
    }
}

impl FrameRenderer for GameState {
    fn render_frame(&self, frame: &mut ratatui::prelude::Frame) -> std::io::Result<()> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3)])
            .split(frame.size());

        frame.render_widget(Paragraph::new(self.settings.size.to_string()), layout[0]);

        Ok(())
    }
}
