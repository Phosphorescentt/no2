use crate::traits::{EventHandler, FrameRenderer, ScreenMessage};

pub struct SettingsState;

impl Default for SettingsState {
    fn default() -> Self {
        SettingsState
    }
}

impl EventHandler for SettingsState {
    fn handle_events(&mut self, event: crossterm::event::Event) -> std::io::Result<ScreenMessage> {
        Ok(ScreenMessage::Noop)
    }
}

impl FrameRenderer for SettingsState {
    fn render_frame(&self, frame: &mut ratatui::prelude::Frame) -> std::io::Result<()> {
        Ok(())
    }
}
