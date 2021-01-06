use pyo3::prelude::pyclass;
use serde::{Serialize, Deserialize};

pub mod deplacement;
pub mod piece_utils;

use self::deplacement::{AvailableMove, Move};

#[pyclass]
#[derive(Serialize, Deserialize, Hash, Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Piece {
    m: Move,
    rank: PieceType,
    color: Box<Color>,
}

#[derive(Serialize, Deserialize, Hash, Debug, Eq, Ord, PartialEq, PartialOrd, Copy, Clone)]
pub enum Color {
    None,
    Red,
    Blue,
}

#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Hash)]
pub enum PieceType {
    Null = -10,
    Bomb = -2,
    Marshal = 10,
    General = 9,
    Colonel = 8,
    Major = 7,
    Captain = 6,
    Lieutenant = 5,
    Sergeant = 4,
    Miner = 3,
    Scout = 2,
    Spy = 1,
    Flag = -1,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Box<Color>) -> Self {
        match piece_type {
            PieceType::Null => Piece {
                m: Move::new(AvailableMove::Immovable),
                rank: PieceType::Null,
                color,
            },
            PieceType::Bomb => Piece {
                m: Move::new(AvailableMove::Immovable),
                rank: PieceType::Bomb,
                color,
            },
            PieceType::Marshal => Piece {
                m: Move::new(AvailableMove::Normal),
                rank: PieceType::Marshal,
                color,
            },
            PieceType::General => Piece {
                m: Move::new(AvailableMove::Normal),
                rank: PieceType::General,
                color,
            },
            PieceType::Colonel => Piece {
                m: Move::new(AvailableMove::Normal),
                rank: PieceType::Colonel,
                color,
            },
            PieceType::Major => Piece {
                m: Move::new(AvailableMove::Normal),
                rank: PieceType::Major,
                color,
            },
            PieceType::Captain => Piece {
                m: Move::new(AvailableMove::Normal),
                rank: PieceType::Captain,
                color,
            },
            PieceType::Lieutenant => Piece {
                m: Move::new(AvailableMove::Normal),
                rank: PieceType::Lieutenant,
                color,
            },
            PieceType::Sergeant => Piece {
                m: Move::new(AvailableMove::Normal),
                rank: PieceType::Sergeant,
                color,
            },
            PieceType::Miner => Piece {
                m: Move::new(AvailableMove::Normal),
                rank: PieceType::Miner,
                color,
            },
            PieceType::Scout => Piece {
                m: Move::new(AvailableMove::Scout),
                rank: PieceType::Scout,
                color,
            },
            PieceType::Spy => Piece {
                m: Move::new(AvailableMove::Normal),
                rank: PieceType::Spy,
                color,
            },
            PieceType::Flag => Piece {
                m: Move::new(AvailableMove::Immovable),
                rank: PieceType::Flag,
                color,
            },
        }
    }

    pub fn get_rank(&self) -> &PieceType {
        &self.rank
    }

    pub fn get_move(&self) -> &Move {
        &self.m
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn display(&self) -> String {
        let mut msg = " R";
        if let Color::Blue = *self.color {
            msg = " B";
        }

        match self.rank {
            PieceType::Null => "     ".to_string(),
            PieceType::Bomb => format!(" B {}", msg),
            PieceType::Marshal => format!("Mar{}", msg),
            PieceType::General => format!(" G {}", msg),
            PieceType::Colonel => format!("Col{}", msg),
            PieceType::Major => format!("Maj{}", msg),
            PieceType::Captain => format!("Cap{}", msg),
            PieceType::Lieutenant => format!(" L {}", msg),
            PieceType::Sergeant => format!("Ser{}", msg),
            PieceType::Miner => format!("Min{}", msg),
            PieceType::Scout => format!("Sco{}", msg),
            PieceType::Spy => format!("Spy{}", msg),
            PieceType::Flag => format!(" F {}", msg),
        }
    }
}

#[cfg(test)]
mod test {

    use super::{Color, Piece, PieceType};

    #[test]
    fn should_display() {
        let piece = Piece::new(PieceType::Captain, Box::new(Color::Blue));
        assert_eq!("Cap B", piece.display());

        let piece = Piece::new(PieceType::Sergeant, Box::new(Color::Red));
        assert_eq!("Ser R", piece.display());

        let piece = Piece::new(PieceType::Null, Box::new(Color::None));
        assert_eq!("     ", piece.display());
    }
}
