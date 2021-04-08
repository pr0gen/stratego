use pyo3::prelude::pyclass;
use serde::{Deserialize, Serialize};
use crate::board::board_utils;
use crate::board::case::{self, Case, Coordinate, State};
use crate::board::piece::Color;
use crate::board::Board;
use crate::error::StrategoError;

#[pyclass]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StrategoBoard {
    cases: Vec<Vec<Case>>,
    last_coup: (Case, Case),
}

impl StrategoBoard {
    pub fn new(cases: Vec<Vec<Case>>) -> Self {
        let from = case::create_empty_case(Coordinate::new(-1, -1));
        let to = case::create_empty_case(Coordinate::new(-1, -1));
        StrategoBoard { cases, last_coup: (from, to) }
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

#[cfg(test)]
mod test_implementation {
    use crate::board::case::{self, State, Case, Coordinate};
    use crate::board::piece::{Piece, Color, PieceType};
    use crate::board::classic_board::StrategoBoard;
    use crate::board::Board;

    #[test] 
    fn should_get_cases_in_board_straight() {
        let board = StrategoBoard::new(create_3_x_3_stratego_board());
        let at = board.get_at(&Coordinate::new(0, 0));
        let piece = at.get_content();
        assert_eq!(&Color::Blue, piece.get_color());
        assert_eq!(&PieceType::Flag, piece.get_rank());

        let at = board.get_at(&Coordinate::new(1, 2));
        assert_eq!(&State::Empty, at.get_state());

        let at = board.get_at(&Coordinate::new(2, 1));
        let piece = at.get_content();
        assert_eq!(&Color::Red, piece.get_color());
        assert_eq!(&PieceType::Major, piece.get_rank());
    }

    fn create_3_x_3_stratego_board() -> Vec<Vec<Case>> {
        vec![
            vec![
                case::create_full_case(
                    Coordinate::new(0, 0),
                    Piece::new(PieceType::Flag, Color::Blue),
                ),
                case::create_full_case(
                    Coordinate::new(0, 1),
                    Piece::new(PieceType::Major, Color::Blue),
                ),
                case::create_full_case(
                    Coordinate::new(0, 2),
                    Piece::new(PieceType::Spy, Color::Blue),
                ),
            ],
            vec![
                case::create_empty_case(Coordinate::new(1, 0)),
                case::create_empty_case(Coordinate::new(1, 1)),
                case::create_empty_case(Coordinate::new(1, 2)),
            ],
            vec![
                case::create_full_case(
                    Coordinate::new(2, 0),
                    Piece::new(PieceType::Flag, Color::Red),
                ),
                case::create_full_case(
                    Coordinate::new(2, 1),
                    Piece::new(PieceType::Major, Color::Red),
                ),
                case::create_full_case(
                    Coordinate::new(2, 2),
                    Piece::new(PieceType::Spy, Color::Red),
                ),
            ],
        ]
    }
}
