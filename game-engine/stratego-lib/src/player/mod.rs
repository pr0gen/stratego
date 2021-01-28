use std::hash::Hash;

use crate::board::case::Coordinate;
use crate::board::classic_board::StrategoBoard;
use crate::board::piece::Color;
use crate::parse;

pub mod ai_player;

pub trait Player {
    fn ask_next_move(&self, board: StrategoBoard) -> (Coordinate, Coordinate);

    fn get_color(&self) -> &Color;

    fn get_name(&self) -> &str;
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct HumanPlayer {
    color: Color,
    name: String,
}

impl Player for HumanPlayer {
    fn ask_next_move(&self, _board: StrategoBoard) -> (Coordinate, Coordinate) {
        println!("{:?} is playing", self.color);
        println!("from ? (e.g 0A)");
        let from: (i16, i16) = parse::parse_input(parse::read_from_input().unwrap().as_str());

        println!("to ? (e.g 0A)");

        let to: (i16, i16) = parse::parse_input(parse::read_from_input().unwrap().as_str());
        //println!("{:?}|{:?}", from, to);

        (Coordinate::new(from.0, from.1), Coordinate::new(to.0, to.1))
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

impl HumanPlayer {
    pub fn new(color: Color, name: String) -> Self {
        HumanPlayer { color, name }
    }
}

#[cfg(test)]
mod test {
    use crate::board::piece::Color;

    use super::{HumanPlayer, Player};

    #[test]
    fn should_create_player() {
        let hp = HumanPlayer::new(Color::Red, String::from("Tigran"));
        assert_eq!(hp.get_name(), "Tigran");
        assert_eq!(hp.get_color(), &Color::Red);
    }
}
