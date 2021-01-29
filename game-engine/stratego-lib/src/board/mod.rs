use std::fmt::Debug;

use self::case::{Case, Coordinate};
use self::piece::Color;
use super::error::StrategoError;

pub mod board_utils;
pub mod case;
pub mod classic_board;
pub mod piece;

pub trait Board: Clone + Debug {
    fn moving(&mut self, case: Case, to: Coordinate) -> Result<Vec<Case>, StrategoError>;

    fn state(&self) -> &Vec<Vec<Case>>;

    fn display(&self) -> String;

    fn display_by_color(&self, color: &Color) -> String;

    fn get_at(&self, coordinate: &Coordinate) -> &Case;

    fn place(&mut self, case: Case) -> Result<Case, StrategoError>;
}
