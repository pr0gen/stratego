use crate::board::case::{Case, Coordinate};
use crate::board::classic_board::StrategoBoard;
use crate::board::piece::Color;
use crate::board::Board;
use crate::engine_utils;
use crate::error::StrategoError;
use crate::player::ai_player::AIPlayer;
use crate::player::{HumanPlayer, Player};

pub trait Engine<B: Board> {
    fn status(&self) -> &B;

    fn moving(&mut self) -> Result<(), StrategoError>;

    fn perform_move(
        &mut self,
        from: Coordinate,
        to: Coordinate,
    ) -> Result<Vec<Case>, StrategoError>;

    fn get_turn(&self) -> Color;

    fn display(&self) -> String;

    fn display_by_color(&self, color: &Color) -> String;
}

#[derive(Debug)]
pub struct StrategoEngine {
    board: StrategoBoard,
    players: (HumanPlayer, AIPlayer),
    turn: Color,
}

impl StrategoEngine {
    pub fn new(board: StrategoBoard, players: (HumanPlayer, AIPlayer)) -> Self {
        StrategoEngine {
            board,
            players,
            turn: Color::Red,
        }
    }

    fn flip_color(&mut self) {
        if Color::Blue == self.turn {
            self.turn = Color::Red;
        } else {
            self.turn = Color::Blue;
        }
    }


    fn get_player_from_color(&self) -> &dyn Player {
        if Color::Red == self.turn {
            &self.players.0
        } else {
            &self.players.1
        }
    }
}

impl Engine<StrategoBoard> for StrategoEngine {
    fn status(&self) -> &StrategoBoard {
        &self.board
    }

    fn moving(&mut self) -> Result<(), StrategoError> {
        let player = self.get_player_from_color();
        let color = *player.get_color();
        let (from, to) = engine_utils::ask_next_move(player, self.status());
        let c = self.board.get_at(&from);
        if c.get_content().get_color() != &color {
            println!("You should move a piece of your color !");
            self.moving()
        } else {
            match self.perform_move(from, to) {
                Ok(_) => {
                    println!("{}", self.display_by_color(&self.turn));
                    Ok(())
                }
                Err(StrategoError::MoveError(message, _, _)) => {
                    println!("{}", message);
                    self.moving()
                }
                Err(e) => {
                    panic!("Something went wrong when moving piece, {}", e.message())
                }
            }
        }
    }

    fn perform_move(&mut self, from: Coordinate, to: Coordinate) -> Result<Vec<Case>, StrategoError> {
        match self.board.moving(from, to) {
            Ok(cases) => {
                self.flip_color();
                Ok(cases)
            }
            Err(e) => Err(e),
        }
    }

    fn get_turn(&self) -> Color {
        self.turn
    }

    fn display(&self) -> String {
        self.board.display()
    }

    fn display_by_color(&self, color: &Color) -> String {
        self.board.display_by_color(color)
    }
}

#[cfg(test)]
mod test {
    use crate::board::classic_board::create_empty_stratego_board;
    use crate::board::piece::Color;
    use crate::board::Board;
    use crate::player::ai_player::AIPlayer;
    use crate::player::HumanPlayer;

    use super::{Engine, StrategoEngine};

    #[test]
    fn should_create_and_get_status() {
        let engine = StrategoEngine::new(
            create_empty_stratego_board(),
            (
                HumanPlayer::new(Color::Red, String::from("Tigran")),
                AIPlayer::new(Color::Blue, String::from("Emma")),
            ),
        );

        let state = engine.status();

        assert_eq!(10, state.state().len());
    }
}
