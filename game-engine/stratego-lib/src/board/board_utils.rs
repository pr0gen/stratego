use super::case::{create_empty_case, create_full_case, Case, Coordinate};
use crate::board::Board;
use crate::engine_utils;
use crate::error::StrategoError;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn attack(from: Case, to: Case) -> Result<(Case, Case), StrategoError> {
    let attacker = from.get_content();
    let defenser = to.get_content();

    if attacker.get_color() == defenser.get_color() {
        Err(StrategoError::MoveError(
            String::from("Don't attack your self"),
            from,
            to.get_coordinate().to_owned(),
        ))
    } else if attacker.get_rank() > defenser.get_rank() {
        Ok((
            create_empty_case(*from.get_coordinate()),
            create_full_case(*to.get_coordinate(), from.get_content().clone()),
        ))
    } else if attacker.get_rank() == defenser.get_rank() {
        Ok((
            create_empty_case(*from.get_coordinate()),
            create_empty_case(*to.get_coordinate()),
        ))
    } else {
        Ok((
            create_empty_case(*from.get_coordinate()),
            create_full_case(*to.get_coordinate(), to.get_content().clone()),
        ))
    }
}

pub fn check_move(board: &impl Board, from: &Coordinate, to: &Coordinate) -> bool {
    let response: Vec<_> = engine_utils::get_availables_moves(board)
        .into_iter()
        .filter(|(f, t, _, _)| f == from && t == to)
        .collect();
    let size = response.len();
    size == 1
}


