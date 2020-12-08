use self::case::{Case, Coordinate};
use super::error::StrategoError;

pub mod board_utils;
pub mod case;
pub mod classic_board;
pub mod piece;

pub trait Board {
    fn moving(&mut self, case: Case, to: Coordinate) -> Result<Vec<Case>, StrategoError>;

    fn state(&self) -> &Vec<Vec<Case>>;

    fn display(&self) -> String;
}
