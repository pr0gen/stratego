use super::case::{create_empty_case, create_full_case, Case, Coordinate};
use crate::error::StrategoError;
//use crate::board::piece::PieceType;
//use crate::engine_utils::

pub fn attack(from: Case, to: Case) -> Result<(Case, Case), StrategoError> {
    let attacker = from.get_content();
    let defenser = to.get_content();

    //if &PieceType::Scout == attacker.get_rank() {
    //check_scout_move()
    //}

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

pub fn check_piece_move(case: &Case, to: &Coordinate) -> bool {
    let m = case.get_content().get_move();
    let c_coord = case.get_coordinate();
    let delta_x = (to.get_x() - c_coord.get_x()).abs();
    let delta_y = (to.get_y() - c_coord.get_y()).abs();
    (delta_x <= m.get_max() && delta_y == 0) || (delta_x == 0 && delta_y <= m.get_max())
}
