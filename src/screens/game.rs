use std::ops::Deref;

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

#[derive(Clone)]
pub struct GameState {
    pub settings: GameSettings,
    pub board_state: BoardState,
}

#[derive(Clone)]
pub struct BoardState {
    size: u8,
    // Values generated at the start
    pub true_values: Vec<Vec<bool>>,
    row_counts: Vec<Vec<u8>>,
    column_counts: Vec<Vec<u8>>,
    // Values assigned by the player
    assigned_values: Vec<Vec<Option<bool>>>,
    selected_square: (u8, u8),
    invalid_tile: Option<(u8, u8)>,
}

impl BoardState {
    fn render(self) -> Paragraph<'static> {
        let max_len_column_counts = self.column_counts.iter().map(|x| x.len()).max().unwrap();
        let max_len_row_counts = self.row_counts.iter().map(|x| x.len()).max().unwrap();

        let mut lines: Vec<Line> = Vec::new();

        // Render column counts
        let column_paddings: Vec<usize> = self
            .column_counts
            .iter()
            .map(|x| max_len_column_counts - x.len())
            .collect();
        for j in 0..max_len_column_counts {
            let mut column_counts_line: String =
                String::from(" ".repeat((max_len_row_counts * 2) as usize));
            for n in 0..self.size {
                if j < column_paddings[n as usize] {
                    column_counts_line.push(' ');
                } else {
                    let current_count =
                        self.column_counts[n as usize].get(j - column_paddings[n as usize]);
                    if let Some(v) = current_count {
                        column_counts_line.push_str(v.to_string().as_str());
                    } else {
                        column_counts_line.push(' ');
                    }
                }
            }
            lines.push(Line::from(column_counts_line.white()))
        }

        for m in 0..self.size {
            let mut line_characters: Vec<Span<'static>> = Vec::new();

            // Render row counts
            let row_padding = max_len_row_counts - self.row_counts[m as usize].len();
            for i in 0..max_len_row_counts {
                if i < row_padding {
                    line_characters.push(" ".white());
                } else {
                    let current_count = self.row_counts[m as usize].get(i - row_padding);
                    if let Some(v) = current_count {
                        line_characters.push(v.to_string().white());
                    }
                }
                line_characters.push(" ".white());
            }

            // Render board
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

        let mut row_counts: Vec<Vec<u8>> = vec![Vec::new(); settings.size as usize];
        let mut column_counts: Vec<Vec<u8>> = vec![Vec::new(); settings.size as usize];
        let mut reset_row_counts: bool = true;
        let mut reset_column_counts: Vec<bool> = vec![true; settings.size as usize];

        let mut assigned_values: Vec<Vec<Option<bool>>> = Vec::new();
        for m in 0..settings.size {
            reset_row_counts = true;
            let mut true_current_row: Vec<bool> = Vec::new();
            let mut assigned_current_row: Vec<Option<bool>> = Vec::new();
            for n in 0..settings.size {
                let b: bool = rng.gen();
                true_current_row.push(b);

                // Swap comments here for debugging purposes.
                // assigned_current_row.push(Some(b));
                assigned_current_row.push(None);

                if b {
                    if reset_row_counts {
                        row_counts[m as usize].push(0);
                    }
                    *row_counts[m as usize].last_mut().unwrap() += 1;

                    if reset_column_counts[n as usize] {
                        column_counts[n as usize].push(0);
                    }
                    *column_counts[n as usize].last_mut().unwrap() += 1;

                    reset_row_counts = false;
                    reset_column_counts[n as usize] = false;
                } else {
                    reset_row_counts = true;
                    reset_column_counts[n as usize] = true;
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
    fn toggle_selected_square(&mut self) -> color_eyre::Result<ScreenMessage> {
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

    fn move_selected_up(&mut self) -> color_eyre::Result<ScreenMessage> {
        let selected_square = self.board_state.selected_square;
        self.board_state.selected_square = (selected_square.0.saturating_sub(1), selected_square.1);
        Ok(ScreenMessage::Noop)
    }

    fn move_selected_down(&mut self) -> color_eyre::Result<ScreenMessage> {
        let selected_square = self.board_state.selected_square;
        if selected_square.0 == (self.board_state.size - 1) as u8 {
            return Ok(ScreenMessage::Noop);
        }
        self.board_state.selected_square = (selected_square.0.saturating_add(1), selected_square.1);
        Ok(ScreenMessage::Noop)
    }

    fn move_selected_left(&mut self) -> color_eyre::Result<ScreenMessage> {
        let selected_square = self.board_state.selected_square;
        self.board_state.selected_square = (selected_square.0, selected_square.1.saturating_sub(1));
        Ok(ScreenMessage::Noop)
    }

    fn move_selected_right(&mut self) -> color_eyre::Result<ScreenMessage> {
        let selected_square = self.board_state.selected_square;
        if selected_square.1 == (self.board_state.size - 1) as u8 {
            return Ok(ScreenMessage::Noop);
        }
        self.board_state.selected_square = (selected_square.0, selected_square.1.saturating_add(1));
        Ok(ScreenMessage::Noop)
    }

    fn check_assigned(&mut self) -> color_eyre::Result<ScreenMessage> {
        let solved = self.board_state.check_assigned();

        match solved {
            BoardValidity::Valid => Ok(ScreenMessage::ChangeScreen(Screen::End(EndState::from(
                self.clone(),
            )))),
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
    ) -> color_eyre::Result<crate::traits::ScreenMessage> {
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
    fn render_frame(&self, frame: &mut ratatui::prelude::Frame) -> color_eyre::Result<()> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(20), Constraint::Length(2)])
            .split(frame.size());

        frame.render_widget(self.board_state.clone().render(), layout[0]);
        frame.render_widget(
            Paragraph::new(format!("{:?}", self.board_state.invalid_tile).to_string()),
            layout[1],
        );

        Ok(())
    }
}
