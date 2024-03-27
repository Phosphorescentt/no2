use std::collections::HashMap;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::Paragraph,
};

use crate::{
    app::Screen,
    components::Button,
    traits::{EventHandler, FrameRenderer, ScreenMessage},
};

use super::game::{GameSettings, GameState};
use super::home::HomeState;

enum ButtonAction {
    StartGame(GameSettings),
    Back,
    Noop,
}

pub struct SettingsState {
    selected_button: u8,
    menu_buttons: Vec<crate::components::Button>,
    // Keys are the button IDs.
    action_map: HashMap<String, ButtonAction>,
    // TODO: move to system where buttons will mutate this variable and then when we hit start we
    // use this variable to initialise the game state.
    // settings: GameSettings,
}

impl SettingsState {
    fn move_prev_button(&mut self) -> color_eyre::Result<ScreenMessage> {
        self.selected_button = self.selected_button.saturating_sub(1);
        Ok(ScreenMessage::Noop)
    }

    fn move_next_button(&mut self) -> color_eyre::Result<ScreenMessage> {
        if self.selected_button == (self.menu_buttons.len() - 1) as u8 {
            return Ok(ScreenMessage::Noop);
        }
        self.selected_button = self.selected_button.saturating_add(1);

        Ok(ScreenMessage::Noop)
    }

    fn select_button(&mut self) -> color_eyre::Result<ScreenMessage> {
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
            ButtonAction::StartGame(settings) => Ok(ScreenMessage::ChangeScreen(Screen::Game(
                GameState::from(settings.clone()),
            ))),
            ButtonAction::Back => Ok(ScreenMessage::ChangeScreen(Screen::Home(
                HomeState::default(),
            ))),
            ButtonAction::Noop => Ok(ScreenMessage::Noop),
        }
    }
}

impl Default for SettingsState {
    fn default() -> Self {
        let five_button_id = String::from("5by5");
        let ten_button_id = String::from("10by10");
        let back_button_id = String::from("back");
        let five_button = Button::new(five_button_id.clone(), String::from("5x5"));
        let ten_button = Button::new(ten_button_id.clone(), String::from("10x10"));
        let back_button = Button::new(back_button_id.clone(), String::from("Back"));

        SettingsState {
            selected_button: 0,
            menu_buttons: vec![five_button, ten_button, back_button],
            action_map: HashMap::from([
                (
                    five_button_id,
                    ButtonAction::StartGame(GameSettings { size: 5 }),
                ),
                (
                    ten_button_id,
                    ButtonAction::StartGame(GameSettings { size: 10 }),
                ),
                (back_button_id, ButtonAction::Back),
            ]),
        }
    }
}

impl EventHandler for SettingsState {
    fn handle_events(&mut self, event: Event) -> color_eyre::Result<ScreenMessage> {
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

impl FrameRenderer for SettingsState {
    fn render_frame(&self, frame: &mut ratatui::prelude::Frame) -> color_eyre::Result<()> {
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
