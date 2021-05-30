use crate::board::board_utils;
use crate::board::case::{self, Case, Coordinate, State};
use crate::board::piece::Color;
use crate::board::Board;
use crate::error::StrategoError;
use pyo3::prelude::pyclass;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StrategoBoard {
    cases: Vec<Vec<Case>>,
    last_coup: Option<((Case, Case), i8)>,
    coup: i128,
}

impl StrategoBoard {
    pub fn new(cases: Vec<Vec<Case>>) -> Self {
        StrategoBoard {
            cases,
            last_coup: None,
            coup: 0,
        }
    }

    pub fn get_coup(&self) -> i128 {
        self.coup.clone()
    }

    fn move_piece_when_empty(
        &mut self,
        to: Coordinate,
        case: &Case,
    ) -> Result<Vec<Case>, StrategoError> {
        let piece = case.get_content();
        let new_case = case::create_full_case(to, piece.to_owned());
        self.place(new_case.clone())?;
        let case_coord = case.get_coordinate();
        let from = case::create_empty_case(case_coord.to_owned());
        self.place(from.clone())?;

        Ok(vec![from, new_case])
    }

    fn move_piece_when_full(
        &mut self,
        case: &Case,
        aim_case: &Case,
    ) -> Result<(Vec<Case>, bool), StrategoError> {
        let piece = case.get_content();
        let case_coord = case.get_coordinate();

        match board_utils::attack(
            case::create_full_case(case_coord.to_owned(), piece.to_owned()),
            aim_case.to_owned(),
        ) {
            Ok(((from, to), res)) => {
                self.place(from.clone())?;
                self.place(to.clone())?;

                Ok((vec![from, to], res))
            }
            Err(e) => Err(e),
        }
    }

    /// 0 -> If moved on empty
    /// 1 -> If loss
    /// 2 -> If Both loss
    /// 3 -> If won
    fn register_last_move(
        &mut self,
        move_result: &[Case],
        has_kill: bool,
        color: &Color,
    ) -> Result<(), StrategoError> {
        let length = move_result.len();
        if 2 != length {
            return Err(StrategoError::AIExecuteError(String::from(
                "There is two cases in a move, something really bad happened",
            )));
        }

        let to = move_result[1].clone();
        let to_content = to.get_content();
        let state = if has_kill {
            3
        } else if &State::Empty == to.get_state() {
            2
        } else if color != to_content.get_color() {
            1
        } else {
            0
        };

        let from = move_result[0].clone();
        self.last_coup = Some(((from, to), state));
        Ok(())
    }

    pub fn get_last_coup(&self) -> &Option<((Case, Case), i8)> {
        &self.last_coup
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

        let mut has_kill = false;
        let aim_case = self.get_at(&to).to_owned();

        let move_action = match aim_case.get_state() {
            State::Empty => self.move_piece_when_empty(to, &case),
            State::Full => {
                let (move_action, res) = self.move_piece_when_full(&case, &aim_case)?;
                has_kill = res;
                Ok(move_action)
            }
            State::Unreachable => Err(StrategoError::MoveError(
                String::from("Unreachable"),
                case.clone(),
                to,
            )),
        }?;

        let content = case.get_content();
        self.register_last_move(&move_action, has_kill, &content.get_color())?;

        self.coup += 1;
        Ok(move_action)
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
    use crate::board::case::{self, Case, Coordinate, State};
    use crate::board::classic_board::StrategoBoard;
    use crate::board::piece::{Color, Piece, PieceType};
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
