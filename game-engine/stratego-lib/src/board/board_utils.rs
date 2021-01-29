use super::case::{create_empty_case, create_full_case, Case, Coordinate, State};
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

pub fn check_piece_move(case: &Case, to: &Coordinate) -> bool {
    let m = case.get_content().get_move();
    let c_coord = case.get_coordinate();
    let delta_x = (to.get_x() - c_coord.get_x()).abs();
    let delta_y = (to.get_y() - c_coord.get_y()).abs();
    (delta_x <= m.get_max() && delta_y == 0) || (delta_x == 0 && delta_y <= m.get_max())
}

pub fn check_scout_move(case: &Case, to: &Coordinate, cases: &[Vec<Case>]) -> bool {
    let from = case.get_coordinate();
    if from.get_y() == to.get_y() {
        let vertical = find_vertical_cases(cases, to.get_y());
        match find_direction_vertical(from, to) {
            Direction::Up => check_part_of_row(&vertical),
            Direction::Down => check_part_of_row(&vertical),
            _ => panic!(),
        }
    } else {
        let horizontal = cases.get(to.get_x() as usize).unwrap_or_else(|| {
            panic!(
                "Failed to get case from board at {:?}, to check scout move",
                to
            )
        });
        match find_direction_horizontal(from, to) {
            Direction::Left => check_part_of_row(&horizontal),
            Direction::Right => check_part_of_row(&horizontal),
            _ => panic!(),
        }
    }
}

pub fn find_vertical_cases(cases: &[Vec<Case>], y: i16) -> Vec<Case> {
    let mut vec = Vec::new();
    for row in cases {
        for case in row {
            let coordinate = case.get_coordinate();
            if y == coordinate.get_y() {
                vec.push(case.to_owned());
            }
        }
    }
    vec
}

fn check_part_of_row(cases: &[Case]) -> bool {
    for case in cases {
        let state = case.get_state();
        if &State::Unreachable == state || &State::Full == state {
            return false;
        }
    }
    true
}

fn find_direction_vertical(from: &Coordinate, to: &Coordinate) -> Direction {
    if from.get_x() - to.get_x() > 0 {
        Direction::Down
    } else {
        Direction::Up
    }
}

fn find_direction_horizontal(from: &Coordinate, to: &Coordinate) -> Direction {
    if from.get_y() - to.get_y() > 0 {
        Direction::Right
    } else {
        Direction::Left
    }
}
