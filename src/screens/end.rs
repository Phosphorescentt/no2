use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::Paragraph;

use crate::screens::game::GameState;
use crate::traits::{EventHandler, FrameRenderer};

pub struct EndState {
    end_game_state: GameState,
}

#[derive(Debug)]
struct EndGameStats {
    total_squares: u16,
    black_squares: u16,
}

impl EndState {
    fn get_game_stats(&self) -> EndGameStats {
        let total_squares =
            (self.end_game_state.settings.size * self.end_game_state.settings.size) as u16;
        let black_squares = self
            .end_game_state
            .board_state
            .true_values
            .iter()
            .fold(0, |acc, e| {
                acc + e
                    .iter()
                    .map(|x| match x {
                        true => 1,
                        false => 0,
                    })
                    .sum::<u16>()
            });

        return EndGameStats {
            total_squares,
            black_squares,
        };
    }
}

impl From<GameState> for EndState {
    fn from(game_state: GameState) -> Self {
        return EndState {
            end_game_state: game_state,
        };
    }
}

impl EventHandler for EndState {}

impl FrameRenderer for EndState {
    fn render_frame(&self, frame: &mut ratatui::prelude::Frame) -> color_eyre::Result<()> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(20)])
            .split(frame.size());

        frame.render_widget(
            Paragraph::new(format!("{:?}", self.get_game_stats())),
            layout[0],
        );

        Ok(())
    }
}
