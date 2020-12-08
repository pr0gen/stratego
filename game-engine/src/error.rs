use crate::board::case::{Case, Coordinate};
use crate::board::piece::Piece;

#[derive(Debug)]
pub enum StrategoError {
    MoveError(String, Case, Coordinate),
    PlacementError(i16, i16, Piece),
    InitGameError(String),
}

impl StrategoError {
    pub fn message(&self) -> String {
        match self {
            StrategoError::MoveError(string, case, coordinate) => {
                let coord = case.get_coordinate();
                format!(
                    "{} | Can not move piece {} ({}, {}) to ({}, {})",
                    string,
                    case.display(),
                    coord.get_x(),
                    coord.get_y(),
                    coordinate.get_x(),
                    coordinate.get_y()
                )
            }
            StrategoError::PlacementError(x, y, piece) => format!(
                "Can not place piece {:?} to ({}, {})",
                piece.get_rank(),
                x,
                y
            ),
            StrategoError::InitGameError(s) => s.to_owned(),
        }
    }
}
