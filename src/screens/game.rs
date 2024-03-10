use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::Paragraph,
};

use crate::traits::{EventHandler, FrameRenderer, ScreenMessage};

#[derive(Clone, Copy)]
pub struct GameSettings {
    pub size: u8,
}

pub struct GameState {
    pub settings: GameSettings,
    board_state: BoardState,
}

#[derive(Clone)]
struct BoardState {
    size: u8,
    // Values generated at the start
    true_values: Vec<Vec<bool>>,
    // Values assigned by the player
    assigned_values: Vec<Vec<Option<bool>>>,
    selected_square: (u8, u8),
}

impl BoardState {
    fn render(self) -> Paragraph<'static> {
        let mut lines: Vec<Line> = Vec::new();
        for m in 0..self.size {
            let mut line_characters: Vec<Span<'static>> = Vec::new();
            for n in 0..self.size {
                let value = self
                    .assigned_values
                    .get(m as usize)
                    .unwrap()
                    .get(n as usize)
                    .unwrap();
                let ch: char = if let Some(v) = value {
                    match v {
                        true => '#',
                        false => 'X',
                    }
                } else {
                    '.'
                };

                if (m, n) == self.selected_square {
                    line_characters.push(ch.to_string().red())
                } else {
                    line_characters.push(ch.to_string().white())
                }
            }
            lines.push(Line::from(line_characters));
        }

        Paragraph::new(Text::from(lines))
    }
}

impl From<GameSettings> for BoardState {
    fn from(settings: GameSettings) -> Self {
        let mut rng = rand::thread_rng();
        let mut true_values: Vec<Vec<bool>> = Vec::new();
        let mut assigned_values: Vec<Vec<Option<bool>>> = Vec::new();
        for _m in 0..settings.size {
            let mut true_current_row: Vec<bool> = Vec::new();
            let mut assigned_current_row: Vec<Option<bool>> = Vec::new();
            for _n in 0..settings.size {
                let b: bool = rng.gen();
                true_current_row.push(b);
                assigned_current_row.push(None);
            }
            true_values.push(true_current_row);
            assigned_values.push(assigned_current_row);
        }

        BoardState {
            size: settings.size,
            true_values,
            assigned_values,
            selected_square: (0, 0),
        }
    }
}

impl GameState {
    fn toggle_selected_square(&mut self) -> std::io::Result<ScreenMessage> {
        let selected_square = self.board_state.selected_square;

        let current_state = self
            .board_state
            .assigned_values
            .get(selected_square.0 as usize)
            .unwrap()
            .get(selected_square.1 as usize)
            .unwrap();

        let new_state = match current_state {
            None => Some(true),
            Some(true) => Some(false),
            Some(false) => None,
        };

        let mut current_row = self
            .board_state
            .assigned_values
            .get(selected_square.0 as usize)
            .unwrap()
            .clone();

        current_row[selected_square.1 as usize] = new_state;

        self.board_state.assigned_values[selected_square.0 as usize] = current_row;
        Ok(ScreenMessage::Noop)
    }

    fn move_selected_up(&mut self) -> std::io::Result<ScreenMessage> {
        let selected_square = self.board_state.selected_square;
        self.board_state.selected_square = (selected_square.0.saturating_sub(1), selected_square.1);
        Ok(ScreenMessage::Noop)
    }

    fn move_selected_down(&mut self) -> std::io::Result<ScreenMessage> {
        let selected_square = self.board_state.selected_square;
        if selected_square.0 == (self.board_state.size - 1) as u8 {
            return Ok(ScreenMessage::Noop);
        }
        self.board_state.selected_square = (selected_square.0.saturating_add(1), selected_square.1);
        Ok(ScreenMessage::Noop)
    }

    fn move_selected_left(&mut self) -> std::io::Result<ScreenMessage> {
        let selected_square = self.board_state.selected_square;
        self.board_state.selected_square = (selected_square.0, selected_square.1.saturating_sub(1));
        Ok(ScreenMessage::Noop)
    }

    fn move_selected_right(&mut self) -> std::io::Result<ScreenMessage> {
        let selected_square = self.board_state.selected_square;
        if selected_square.1 == (self.board_state.size - 1) as u8 {
            return Ok(ScreenMessage::Noop);
        }
        self.board_state.selected_square = (selected_square.0, selected_square.1.saturating_add(1));
        Ok(ScreenMessage::Noop)
    }
}

impl Default for GameState {
    fn default() -> Self {
        let default_settings = GameSettings { size: 10 };
        GameState {
            settings: default_settings,
            board_state: BoardState::from(default_settings),
        }
    }
}

impl From<GameSettings> for GameState {
    fn from(settings: GameSettings) -> Self {
        GameState {
            settings,
            board_state: BoardState::from(settings),
        }
    }
}

impl EventHandler for GameState {
    fn handle_events(
        &mut self,
        event: crossterm::event::Event,
    ) -> std::io::Result<crate::traits::ScreenMessage> {
        if let Event::Key(key) = event {
            if key.kind == event::KeyEventKind::Release {
                return Ok(ScreenMessage::Noop);
            }

            return match key.code {
                KeyCode::Char(' ') => self.toggle_selected_square(),
                KeyCode::Up => self.move_selected_up(),
                KeyCode::Down => self.move_selected_down(),
                KeyCode::Left => self.move_selected_left(),
                KeyCode::Right => self.move_selected_right(),
                _ => Ok(ScreenMessage::Noop),
            };
        }
        Ok(ScreenMessage::Noop)
    }
}

impl FrameRenderer for GameState {
    fn render_frame(&self, frame: &mut ratatui::prelude::Frame) -> std::io::Result<()> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Percentage(50)])
            .split(frame.size());

        frame.render_widget(Paragraph::new(self.settings.size.to_string()), layout[0]);
        frame.render_widget(self.board_state.clone().render(), layout[1]);

        Ok(())
    }
}
