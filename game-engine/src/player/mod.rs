use crate::board::case::Coordinate;
use crate::board::piece::Color;

pub trait Player {
    
    fn ask_next_move(&self) -> (Coordinate, Coordinate);

    fn get_color(&self) -> &Color;
}

pub struct HumanPlayer {
    color: Color,
}

impl Player for HumanPlayer {
    fn ask_next_move(&self) -> (Coordinate, Coordinate) {
        todo!()
    }

    fn get_color(&self) -> &Color {
        &self.color
    }
}
