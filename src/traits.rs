use crossterm::event::{Event, KeyEvent};
use ratatui::Frame;
use std::io;

pub type Exit = bool;

pub trait FrameRenderer {
    fn render_frame(&self, frame: &mut Frame) -> io::Result<()> {
        todo!()
    }
}

pub trait EventHandler {
    fn handle_events(&mut self, event: Event) -> io::Result<Exit> {
        todo!()
    }
}
