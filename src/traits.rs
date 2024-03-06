use ratatui::Frame;
use std::io;

pub trait FrameRenderer {
    fn render_frame(frame: &mut Frame) -> io::Result<()> {
        todo!()
    }
}

pub trait EventHandler {
    fn handle_events() -> io::Result<()> {
        todo!()
    }
}
