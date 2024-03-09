use crossterm::event::{Event, KeyEvent};
use ratatui::Frame;
use std::io;

use crate::app::Screen;

pub type Exit = bool;

pub enum ScreenMessage {
    Exit,
    ChangeScreen(Screen),
    Noop,
}

pub trait FrameRenderer {
    fn render_frame(&self, frame: &mut Frame) -> io::Result<()> {
        todo!()
    }
}

pub trait EventHandler {
    fn handle_events(&mut self, event: Event) -> io::Result<ScreenMessage> {
        todo!()
    }
}
