use crate::board::case::{Case, Coordinate};
use crate::board::piece::Color;
use crate::board::Board;
use crate::error::StrategoError;
use crate::player::Player;

pub trait Engine {
    fn status(&self) -> &Vec<Vec<Case>>;

    fn execute_move(&mut self, from: Case, to: Coordinate)-> Result<Vec<Case>, StrategoError>;

    fn get_turn(&self) -> Color;

    fn get_player_from_color(&self) -> &Box<dyn Player>;

    fn display(&self) -> String;
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
}

impl Engine for StrategoEngine {
    fn status(&self) -> &Vec<Vec<Case>> {
        self.board.state()
    }

    fn execute_move(&mut self, from: Case, to: Coordinate) -> Result<Vec<Case>, StrategoError> {
        //FIXME There will be bugs for sure
        if let Color::Blue = self.turn {
            self.turn = Color::Red;
        } else {
            self.turn = Color::Blue;
        }

        self.board.moving(from, to)
    }

    fn get_turn(&self) -> Color {
        self.turn
    }

    fn get_player_from_color(&self) -> &Box<dyn Player> {
        if Color::Red == self.turn {
            &self.players.0
        } else {
            &self.players.1
        }
    }

    fn display(&self) -> String {
        self.board.display()
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
