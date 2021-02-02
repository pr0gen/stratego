use pyo3::prelude::pyclass;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::board::board_utils;
use crate::board::case::{ self, Case, Coordinate, State, };
use crate::board::piece::piece_utils::list_of_all_pieces;
use crate::board::piece::Color;
use crate::board::Board;
use crate::engine_utils::verify_board_integrity;
use crate::error::StrategoError;

#[pyclass]
#[derive(Serialize, Deserialize, Clone, Debug)]
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
            *case = case::create_full_case(Coordinate::new(i as i16, j as i16), piece.unwrap());
        }
    }

    let mut pieces = list_of_all_pieces(Color::Red);
    pieces.shuffle(&mut thread_rng());
    let max = 10;
    for (i, row) in cases.iter_mut().enumerate().take(max).skip(6) {
        for (j, case) in row.iter_mut().enumerate() {
            let piece = pieces.pop();
            *case = case::create_full_case(Coordinate::new(i as i16, j as i16), piece.unwrap());
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
            board[i].push(case::create_empty_case(Coordinate::new(i as i16, j as i16)));
        }
    }
    let mut board = StrategoBoard::new(board);

    board
        .place(case::create_unreachable_case(Coordinate::new(4, 2)))
        .unwrap();
    board
        .place(case::create_unreachable_case(Coordinate::new(4, 3)))
        .unwrap();
    board
        .place(case::create_unreachable_case(Coordinate::new(5, 2)))
        .unwrap();
    board
        .place(case::create_unreachable_case(Coordinate::new(5, 3)))
        .unwrap();
    board
        .place(case::create_unreachable_case(Coordinate::new(4, 6)))
        .unwrap();
    board
        .place(case::create_unreachable_case(Coordinate::new(4, 7)))
        .unwrap();
    board
        .place(case::create_unreachable_case(Coordinate::new(5, 6)))
        .unwrap();
    board
        .place(case::create_unreachable_case(Coordinate::new(5, 7)))
        .unwrap();

    board
}

impl StrategoBoard {
    pub fn new(cases: Vec<Vec<Case>>) -> Self {
        StrategoBoard { cases }
    }

    fn move_piece_when_empty(
        &mut self,
        to: Coordinate,
        case: Case,
    ) -> Result<Vec<Case>, StrategoError> {
        let piece = case.get_content();
        let new_case = case::create_full_case(to, piece.to_owned());
        self.place(new_case.clone())?;
        let case_coord = case.get_coordinate();
        self.place(case::create_empty_case(case_coord.to_owned()))?;

        Ok(vec![new_case])
    }

    fn move_piece_when_full(
        &mut self,
        case: Case,
        aim_case: &Case,
    ) -> Result<Vec<Case>, StrategoError> {
        let piece = case.get_content();
        let case_coord = case.get_coordinate();

        match board_utils::attack(
            case::create_full_case(case_coord.to_owned(), piece.to_owned()),
            aim_case.to_owned(),
        ) {
            Ok((from, to)) => {
                self.place(from.clone())?;
                self.place(to.clone())?;

                Ok(vec![from, to])
            }
            Err(e) => Err(e),
        }
    }
}

impl Board for StrategoBoard {
    fn moving(&mut self, from: Coordinate, to: Coordinate) -> Result<Vec<Case>, StrategoError> {
        let case = self.get_at(&from).to_owned();
        if !board_utils::check_move(self, &from, &to) {
            return Err(StrategoError::MoveError(
                String::from("Illegal move"),
                case,
                to,
            ));
        }

        let aim_case = self.get_at(&to).to_owned();

        match aim_case.get_state() {
            State::Empty => self.move_piece_when_empty(to, case),
            State::Full => self.move_piece_when_full(case, &aim_case),
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

    fn place(&mut self, case: Case) -> Result<Case, StrategoError> {
        let coordinate = case.get_coordinate();
        self.cases[coordinate.get_x() as usize][coordinate.get_y() as usize] = case.clone();
        Ok(case)
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
