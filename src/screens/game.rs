use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::Paragraph,
};

use crate::{
    app::Screen,
    traits::{EventHandler, FrameRenderer, ScreenMessage},
};

use super::end::EndState;

#[derive(Clone, Copy)]
pub struct GameSettings {
    pub size: u8,
}

enum BoardValidity {
    Valid,
    Invalid(u8, u8),
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
    row_counts: Vec<u8>,
    column_counts: Vec<u8>,
    // Values assigned by the player
    assigned_values: Vec<Vec<Option<bool>>>,
    selected_square: (u8, u8),
    invalid_tile: Option<(u8, u8)>,
}

impl BoardState {
    fn render(self) -> Paragraph<'static> {
        let mut lines: Vec<Line> = Vec::new();
        let mut column_counts: String = String::from("  ");

        for n in 0..self.size {
            column_counts.push_str(self.column_counts[n as usize].to_string().as_str());
        }

        lines.push(Line::from(column_counts.white()));
        lines.push(Line::from(" ".white()));

        for m in 0..self.size {
            let mut line_characters: Vec<Span<'static>> = Vec::new();
            line_characters.push(self.row_counts[m as usize].to_string().white());
            line_characters.push(" ".white());

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

    fn check_assigned(&self) -> BoardValidity {
        for m in 0..self.size {
            for n in 0..self.size {
                let assigned_value = self
                    .assigned_values
                    .get(m as usize)
                    .unwrap()
                    .get(n as usize)
                    .unwrap();

                let true_value = self
                    .true_values
                    .get(m as usize)
                    .unwrap()
                    .get(n as usize)
                    .unwrap();

                match (assigned_value, true_value) {
                    (Some(v1), v2) => {
                        if v1 != v2 {
                            return BoardValidity::Invalid(m, n);
                        }
                    }
                    (None, _) => return BoardValidity::Invalid(m, n),
                }
            }
        }

        return BoardValidity::Valid;
    }
}

impl From<GameSettings> for BoardState {
    fn from(settings: GameSettings) -> Self {
        let mut rng = rand::thread_rng();
        let mut true_values: Vec<Vec<bool>> = Vec::new();
        let mut row_counts: Vec<u8> = vec![0; settings.size as usize];
        let mut column_counts: Vec<u8> = vec![0; settings.size as usize];
        let mut assigned_values: Vec<Vec<Option<bool>>> = Vec::new();
        for m in 0..settings.size {
            let mut true_current_row: Vec<bool> = Vec::new();
            let mut assigned_current_row: Vec<Option<bool>> = Vec::new();
            for n in 0..settings.size {
                let b: bool = rng.gen();
                true_current_row.push(b);
                assigned_current_row.push(None);

                if b {
                    row_counts[m as usize] += 1;
                    column_counts[n as usize] += 1;
                }
            }
            true_values.push(true_current_row);
            assigned_values.push(assigned_current_row);
        }

        BoardState {
            size: settings.size,
            true_values,
            row_counts,
            column_counts,
            assigned_values,
            selected_square: (0, 0),
            invalid_tile: None,
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

    fn check_assigned(&mut self) -> std::io::Result<ScreenMessage> {
        let solved = self.board_state.check_assigned();

        match solved {
            BoardValidity::Valid => Ok(ScreenMessage::ChangeScreen(Screen::End(EndState))),
            BoardValidity::Invalid(m, n) => {
                self.board_state.invalid_tile = Some((m, n));
                Ok(ScreenMessage::Noop)
            }
        }
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
                KeyCode::Char('c') => self.check_assigned(),
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
            .constraints(vec![Constraint::Percentage(50), Constraint::Length(2)])
            .split(frame.size());

        frame.render_widget(self.board_state.clone().render(), layout[0]);
        frame.render_widget(
            Paragraph::new(format!("{:?}", self.board_state.invalid_tile).to_string()),
            layout[1],
        );

        Ok(())
    }
}
