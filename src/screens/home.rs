use crate::app::Screen;
use crate::components::Button;
use crate::traits::{EventHandler, FrameRenderer, ScreenMessage};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::Paragraph};
use std::collections::HashMap;

use super::settings::SettingsState;

enum ButtonAction {
    StartGame,
    Quit,
    Noop,
}

pub struct HomeState {
    selected_button: u8,
    menu_buttons: Vec<crate::components::Button>,
    // Keys are the button IDs.
    action_map: HashMap<String, ButtonAction>,
}

impl HomeState {
    fn move_prev_button(&mut self) -> std::io::Result<ScreenMessage> {
        self.selected_button = self.selected_button.saturating_sub(1);
        Ok(ScreenMessage::Noop)
    }

    fn move_next_button(&mut self) -> std::io::Result<ScreenMessage> {
        if self.selected_button == (self.menu_buttons.len() - 1) as u8 {
            return Ok(ScreenMessage::Noop);
        }
        self.selected_button = self.selected_button.saturating_add(1);

        Ok(ScreenMessage::Noop)
    }

    fn select_button(&mut self) -> std::io::Result<ScreenMessage> {
        // TODO: remove all these unwraps
        let action = self
            .action_map
            .get(
                &self
                    .menu_buttons
                    .get(self.selected_button as usize)
                    .unwrap()
                    .id,
            )
            .unwrap();

        match action {
            ButtonAction::StartGame => Ok(ScreenMessage::ChangeScreen(Screen::Settings(
                SettingsState::default(),
            ))),
            ButtonAction::Quit => Ok(ScreenMessage::Exit),
            ButtonAction::Noop => Ok(ScreenMessage::Noop),
        }
    }
}

impl Default for HomeState {
    fn default() -> Self {
        let play_button_id = String::from("play_button");
        let quit_button_id = String::from("quit_button");
        let play_button = Button::new(play_button_id.clone(), String::from("Play!"));
        let quit_button = Button::new(quit_button_id.clone(), String::from("Quit!"));

        HomeState {
            selected_button: 0,
            menu_buttons: vec![play_button, quit_button],
            action_map: HashMap::from([
                (play_button_id, ButtonAction::StartGame),
                (quit_button_id, ButtonAction::Quit),
            ]),
        }
    }
}

impl EventHandler for HomeState {
    fn handle_events(&mut self, event: Event) -> std::io::Result<ScreenMessage> {
        if let Event::Key(key) = event {
            if key.kind == event::KeyEventKind::Release {
                return Ok(ScreenMessage::Noop);
            }

            return match key.code {
                KeyCode::Up => self.move_prev_button(),
                KeyCode::Down => self.move_next_button(),
                KeyCode::Enter => self.select_button(),
                _ => Ok(ScreenMessage::Noop),
            };
        }

        Ok(ScreenMessage::Noop)
    }
}

impl FrameRenderer for HomeState {
    fn render_frame(&self, frame: &mut ratatui::prelude::Frame) -> std::io::Result<()> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3); self.menu_buttons.len()])
            .split(frame.size());

        for (i, menu_button) in self.menu_buttons.iter().enumerate() {
            let mut paragraph_widget = Paragraph::new(menu_button.text.clone());

            match i as u8 == self.selected_button {
                true => {
                    // TODO: replace this with styling from Button struct.
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
