use crate::board::case::{Case, Coordinate};
use crate::board::piece::Color;
use crate::board::Board;
use crate::engine_utils;
use crate::error::StrategoError;
use crate::player::Player;

pub trait Engine {
    fn status(&self) -> &Vec<Vec<Case>>;

    fn moving(&mut self) -> Result<(), StrategoError>;

    fn get_turn(&self) -> Color;

    fn display(&self) -> String;

    fn display_by_color(&self, color: &Color) -> String;
}

pub struct StrategoEngine {
    board: Box<dyn Board>,
    players: (Box<dyn Player>, Box<dyn Player>),
    turn: Color,
}

impl StrategoEngine {
    pub fn new(board: Box<dyn Board>, players: (Box<dyn Player>, Box<dyn Player>)) -> Self {
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

    fn execute_move(&mut self, from: Case, to: Coordinate) -> Result<Vec<Case>, StrategoError> {
        match self.board.moving(from, to) {
            Ok(cases) => {
                self.flip_color();
                Ok(cases)
            }
            Err(e) => Err(e),
        }
    }

    fn get_player_from_color(&self) -> &dyn Player {
        if Color::Red == self.turn {
            &*self.players.0
        } else {
            &*self.players.1
        }
    }
}

impl Engine for StrategoEngine {
    fn status(&self) -> &Vec<Vec<Case>> {
        self.board.state()
    }

    fn moving(&mut self) -> Result<(), StrategoError> {
        let player = self.get_player_from_color();
        let (c, to) = engine_utils::ask_next_move(player, self.status());
        if c.get_content().get_color() != player.get_color() {
            println!("You should move a piece of your color !");
            self.moving()
        } else {
            match self.execute_move(c, to) {
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
    use crate::player::HumanPlayer;

    use super::{Engine, StrategoEngine};

    #[test]
    fn should_create_and_get_status() {
        let engine: Box<dyn Engine> = Box::new(StrategoEngine::new(
            create_empty_stratego_board(),
            (
                Box::new(HumanPlayer::new(Color::Red, String::from("Tigran"))),
                Box::new(HumanPlayer::new(Color::Blue, String::from("Emma"))),
            ),
        ));

        let state = engine.status();

        assert_eq!(10, state.len());
    }
}
