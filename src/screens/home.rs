use crate::components::Button;
use crate::traits::{EventHandler, Exit, FrameRenderer};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::Paragraph};

pub struct HomeState {
    pub selected_button: u8,
    pub menu_buttons: Vec<crate::components::Button>,
}

impl HomeState {
    fn move_select_prev(&mut self) {
        self.selected_button = self.selected_button.saturating_sub(1);
    }

    fn move_select_next(&mut self) {
        if self.selected_button == (self.menu_buttons.len() - 1) as u8 {
            return;
        }
        self.selected_button = self.selected_button.saturating_add(1);
    }

    fn select_button(&mut self) {}
}

impl Default for HomeState {
    fn default() -> Self {
        HomeState {
            selected_button: 0,
            menu_buttons: vec![
                Button::from(String::from("Play!")),
                Button::from(String::from("Quit!")),
            ],
        }
    }
}

impl EventHandler for HomeState {
    fn handle_events(&mut self, event: Event) -> std::io::Result<Exit> {
        if let Event::Key(key) = event {
            if key.kind == event::KeyEventKind::Release {
                return Ok(false);
            }

            match key.code {
                KeyCode::Up => self.move_select_prev(),
                KeyCode::Down => self.move_select_next(),
                KeyCode::Enter => self.select_button(),
                _ => {}
            }
        }

        Ok(false)
    }
}

impl FrameRenderer for HomeState {
    fn render_frame(&self, frame: &mut ratatui::prelude::Frame) -> std::io::Result<()> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Length(3)])
            .split(frame.size());

        for (i, menu_button) in self.menu_buttons.iter().enumerate() {
            let mut paragraph_widget = Paragraph::new(menu_button.text.to_string());

            match i as u8 == self.selected_button {
                true => {
                    paragraph_widget = paragraph_widget.yellow().on_dark_gray();
                }
                _ => {
                    paragraph_widget = paragraph_widget.white();
                }
            }
            frame.render_widget(paragraph_widget, layout[i]);
        }

        Ok(())
    }
}
