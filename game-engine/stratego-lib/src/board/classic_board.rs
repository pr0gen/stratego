use pyo3::prelude::pyclass;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::board::board_utils::{attack, check_piece_move};
use crate::board::case::{
    create_empty_case, create_full_case, create_unreachable_case, Case, Coordinate, State,
};
use crate::board::piece::piece_utils::list_of_all_pieces;
use crate::board::piece::Color;
use crate::board::Board;
use crate::engine_utils::verify_board_integrity;
use crate::error::StrategoError;

#[pyclass]
#[derive(Serialize, Deserialize, Hash, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct StrategoBoard {
    cases: Vec<Vec<Case>>,
}

pub fn create_stratego_board() -> StrategoBoard {
    let board = create_empty_stratego_board();

    let mut cases = board.state().clone();

    let mut pieces = list_of_all_pieces(Color::Blue);
    pieces.shuffle(&mut thread_rng());

    let max = 4;
    for (i, row) in cases.iter_mut().enumerate().take(max) {
        for (j, case) in row.iter_mut().enumerate() {
            let piece = pieces.pop();
            *case = create_full_case(Coordinate::new(i as i16, j as i16), piece.unwrap());
        }
    }

    let mut pieces = list_of_all_pieces(Color::Red);
    pieces.shuffle(&mut thread_rng());
    let max = 10;
    for (i, row) in cases.iter_mut().enumerate().take(max).skip(6) {
        for (j, case) in row.iter_mut().enumerate() {
            let piece = pieces.pop();
            *case = create_full_case(Coordinate::new(i as i16, j as i16), piece.unwrap());
        }
    }

    verify_board_integrity(StrategoBoard::new(cases))
        .unwrap_or_else(|e| panic!("failed to check engine integrity: {:?}", e))
}

pub fn create_empty_stratego_board() -> StrategoBoard {
    let size = 10;
    let mut board: Vec<Vec<Case>> = Vec::with_capacity(size);
    for i in 0..size {
        board.push(Vec::with_capacity(size));
        for j in 0..size {
            board[i].push(create_empty_case(Coordinate::new(i as i16, j as i16)));
        }
    }

    board[4][2] = create_unreachable_case(Coordinate::new(4, 2));
    board[4][3] = create_unreachable_case(Coordinate::new(4, 3));
    board[5][2] = create_unreachable_case(Coordinate::new(5, 2));
    board[5][3] = create_unreachable_case(Coordinate::new(5, 3));
    board[4][6] = create_unreachable_case(Coordinate::new(4, 6));
    board[4][7] = create_unreachable_case(Coordinate::new(4, 7));
    board[5][6] = create_unreachable_case(Coordinate::new(5, 6));
    board[5][7] = create_unreachable_case(Coordinate::new(7, 5));

    StrategoBoard::new(board)
}

impl StrategoBoard {
    pub fn new(cases: Vec<Vec<Case>>) -> Self {
        StrategoBoard { cases }
    }

    pub fn place(&mut self, case: Case) -> Result<(), StrategoError> {
        let coordinate = case.get_coordinate();
        self.cases[coordinate.get_x() as usize][coordinate.get_y() as usize] = case.clone();
        Ok(())
    }
}

impl Board for StrategoBoard {
    fn moving(&mut self, case: Case, to: Coordinate) -> Result<Vec<Case>, StrategoError> {
        if !check_piece_move(&case, &to) {
            return Err(StrategoError::MoveError(
                String::from("Illegal move"),
                case,
                to,
            ));
        }

        let piece = case.get_content();
        let to_x = to.get_x();
        let to_y = to.get_y();
        let aim_case = self
            .cases
            .get(to_x as usize)
            .unwrap_or_else(|| panic!("Failed to get from column"))
            .get(to_y as usize)
            .unwrap_or_else(|| panic!("Failed to get from row"));
        match aim_case.get_state() {
            State::Empty => {
                let new_case = create_full_case(to, piece.to_owned());
                self.cases[to_x as usize][to_y as usize] = new_case.clone();
                let case_coord = case.get_coordinate();
                self.cases[case_coord.get_x() as usize][case_coord.get_y() as usize] =
                    create_empty_case(case_coord.to_owned());
                Ok(vec![new_case])
            }
            State::Full => {
                let case_coord = case.get_coordinate();
                match attack(
                    create_full_case(case_coord.to_owned(), piece.to_owned()),
                    aim_case.to_owned(),
                ) {
                    Ok((from, to)) => {
                        self.cases[case_coord.get_x() as usize][case_coord.get_y() as usize] =
                            from.clone();
                        self.cases[to_x as usize][to_y as usize] = to.clone();
                        Ok(vec![from, to])
                    }
                    Err(e) => Err(e),
                }
            }
            State::Unreachable => Err(StrategoError::MoveError(
                String::from("Unreachable"),
                case,
                to,
            )),
        }
    }

    fn state(&self) -> &Vec<Vec<Case>> {
        &self.cases
    }

    fn display(&self) -> String {
        let mut display = String::from("   |");
        let size = self.cases.len();
        display.push_str(get_header(size).as_str());
        display.push('\n');
        for i in 0..size {
            let row = self.cases.get(i).unwrap();
            display.push_str(format!(" {} | {}\n", i, parse_row(row)).as_str());
        }
        display
    }

    fn display_by_color(&self, color: &Color) -> String {
        let mut display = String::from("   |");
        let size = self.cases.len();
        display.push_str(get_header(size).as_str());
        display.push('\n');
        for i in 0..size {
            let row = self.cases.get(i).unwrap();
            display.push_str(format!(" {} | {}\n", i, parse_row_by_color(row, color)).as_str());
        }
        display
    }

    fn get_at(&self, coordinate: &Coordinate) -> &Case {
        &self.cases[coordinate.get_x() as usize][coordinate.get_y() as usize]
    }

}

fn get_header(length: usize) -> String {
    let columns = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    columns[0..length]
        .chars()
        .map(|c| format!("   {}   |", c))
        .collect()
}

fn parse_row(row: &[Case]) -> String {
    row.iter()
        .map(|case| format!("{} | ", case.display()))
        .collect()
}

fn parse_row_by_color(row: &[Case], color: &Color) -> String {
    row.iter()
        .map(|case| format!("{} | ", case.display_by_color(color)))
        .collect()
}
