use crate::board::case::Case;
use crate::board::case::Coordinate;
use crate::board::classic_board::*;
use crate::board::piece::Color;
use crate::board::Board;
use crate::player::Player;

pub trait Engine {
    fn status(&self) -> Box<dyn Board>;

    fn ask_next_move(&self, player: Box<dyn Player>) -> (Case, Coordinate);
}

pub struct StrategoEngine {
    board: Box<dyn Board>,
    players: (Box<dyn Player>, Box<dyn Player>),
    turn: Color,
}
