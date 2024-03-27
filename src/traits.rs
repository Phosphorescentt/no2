use crossterm::event::Event;
use ratatui::Frame;
use std::io;

use crate::app::Screen;

pub enum ScreenMessage {
    Exit,
    ChangeScreen(Screen),
    Noop,
}

pub trait FrameRenderer {
    fn render_frame(&self, frame: &mut Frame) -> color_eyre::Result<()> {
        todo!("render_frame not yet implemented!");
    }
}

pub trait EventHandler {
    fn handle_events(&mut self, event: Event) -> color_eyre::Result<ScreenMessage> {
        todo!("handle_events not yet implemented!");
    }
}
