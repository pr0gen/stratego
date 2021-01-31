use pyo3::prelude::pyclass;
use std::fmt;
use serde::{Deserialize, Serialize};

use self::deplacement::{AvailableMove, Move};

pub mod deplacement;
pub mod piece_utils;

pub type PyColor = String;
pub type PyPieceType = i8;

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Piece {
    m: Move,
    rank: PieceType,
    color: Color,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub enum Color {
    None,
    Red,
    Blue,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, PartialOrd, Clone, Hash)]
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

impl Into<Color> for PyColor {
    fn into(self) -> Color {
        match self.as_str() {
            "Blue" => Color::Blue,
            "Red" => Color::Red,
            _ => Color::None,
        }
    }
}

impl Into<PieceType> for PyPieceType {
    fn into(self) -> PieceType {
        match self {
            -1 => PieceType::Flag,
            -2 => PieceType::Bomb,
            10 => PieceType::Marshal,
            9 => PieceType::General,
            8 => PieceType::Colonel,
            7 => PieceType::Major,
            6 => PieceType::Captain,
            5 => PieceType::Lieutenant,
            4 => PieceType::Sergeant,
            3 => PieceType::Miner,
            2 => PieceType::Scout,
            1 => PieceType::Spy,
            _ => PieceType::Null,
        }
    }
}

impl From<&str> for Color {
    fn from(color: &str) -> Self {
        match color {
            "Red" => Color::Red,
            "Blue" => Color::Blue,
            _ => Color::None,
        }
    }
}

impl Color {
    pub fn as_str(&self) -> &str {
        match self {
            Color::Red => "Red",
            Color::Blue => "Blue",
            _ => "None",
        }
    }
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
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
        if let Color::Blue = self.color {
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


impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{}", self.rank, self.color.as_str())
    }
}

#[cfg(test)]
mod test {

    use super::{Color, Piece, PieceType};

    #[test]
    fn should_display() {
        let piece = Piece::new(PieceType::Captain, Color::Blue);
        assert_eq!("Cap B", piece.display());

        let piece = Piece::new(PieceType::Sergeant, Color::Red);
        assert_eq!("Ser R", piece.display());

        let piece = Piece::new(PieceType::Null, Color::None);
        assert_eq!("     ", piece.display());
    }
}
