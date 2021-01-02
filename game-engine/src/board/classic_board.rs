use rand::seq::SliceRandom;
use rand::thread_rng;

use super::board_utils::{attack, check_piece_move};
use super::case::{
    create_empty_case, create_full_case, create_unreachable_case, Case, Coordinate, State,
};
use super::piece::piece_utils::list_of_all_pieces;
use super::piece::Color;
use super::Board;
use crate::engine_utils::verify_board_integrity;
use crate::error::StrategoError;

#[derive(Hash, Clone,  Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct StrategoBoard {
    cases: Vec<Vec<Case>>,
}

//to test
pub fn create_stratego_board() -> impl Board {
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

    let cases = verify_board_integrity(cases)
        .unwrap_or_else(|e| panic!("failed to check engine integrity: {:?}", e));

    StrategoBoard::new(cases)
}

pub fn create_empty_stratego_board() -> impl Board {
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

#[cfg(test)]
mod test {

    use super::StrategoBoard;
    use super::{
        create_empty_case, create_empty_stratego_board, create_full_case, create_unreachable_case,
    };
    use crate::board::case::Coordinate;
    use crate::board::case::State;
    use crate::board::piece::deplacement::{AvailableMove, Move};
    use crate::board::piece::{Color, Piece, PieceType};
    use crate::board::Board;

    #[test]
    fn should_build_stratego_board() {
        let stratego_board = create_empty_stratego_board();
        let state = stratego_board.state();

        assert_eq!(state.len(), 10);
        assert_eq!(state[0].len(), 10);
    }

    #[test]
    fn should_display() {
        let bomb = Piece::new(PieceType::Bomb, Box::new(Color::Blue));
        let stratego_board = StrategoBoard {
            cases: vec![vec![create_full_case(Coordinate::new(0, 0), bomb)]],
        };

        assert_eq!("   |   A   |\n 0 |  B  B | \n", stratego_board.display());
    }

    #[test]
    fn should_place_piece_in_board() {
        let bomb = Piece::new(PieceType::Bomb, Box::new(Color::Blue));
        let stratego_board = StrategoBoard {
            cases: vec![vec![create_full_case(Coordinate::new(0, 0), bomb)]],
        };
        let actual_case = stratego_board.state().get(0).unwrap().get(0).unwrap();

        let content = actual_case.get_content();
        assert_eq!(content.get_rank(), &PieceType::Bomb);
        assert_eq!(content.get_move(), &Move::new(AvailableMove::Immovable));
        assert_eq!(content.get_color(), &Color::Blue);

        assert_eq!(actual_case.get_state(), &State::Full);
        let coord = actual_case.get_coordinate();
        assert_eq!(coord.get_x(), 0);
        assert_eq!(coord.get_y(), 0);
    }

    #[test]
    fn should_move_piece() {
        let general = Piece::new(PieceType::General, Box::new(Color::Blue));
        let mut stratego_board = StrategoBoard {
            cases: vec![
                vec![
                    create_full_case(Coordinate::new(0, 0), general.clone()),
                    create_empty_case(Coordinate::new(0, 1)),
                ],
                vec![
                    create_empty_case(Coordinate::new(1, 0)),
                    create_empty_case(Coordinate::new(1, 1)),
                ],
            ],
        };

        println!("{}", stratego_board.display());

        let result = stratego_board.moving(
            create_full_case(Coordinate::new(0, 0), general),
            Coordinate::new(0, 1),
        );

        match result {
            Ok(value) => {
                let p = value.get(0).unwrap();
                assert_eq!(&PieceType::General, p.get_content().get_rank());

                let state = stratego_board.state();
                let actual_case = state.get(0).unwrap().get(1).unwrap();
                let content = actual_case.get_content();
                assert_eq!(&PieceType::General, content.get_rank());
                assert_eq!(&Move::new(AvailableMove::Normal), content.get_move());
                assert_eq!(&Color::Blue, content.get_color());

                assert_eq!(&State::Full, actual_case.get_state());
                let coord = actual_case.get_coordinate();
                assert_eq!(0, coord.get_x());
                assert_eq!(1, coord.get_y());
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn should_not_move_piece_cause_unreachable() {
        let sergeant = Piece::new(PieceType::Sergeant, Box::new(Color::Blue));
        let mut stratego_board = StrategoBoard {
            cases: vec![
                vec![
                    create_empty_case(Coordinate::new(0, 0)),
                    create_empty_case(Coordinate::new(0, 1)),
                ],
                vec![
                    create_empty_case(Coordinate::new(1, 0)),
                    create_unreachable_case(Coordinate::new(1, 1)),
                ],
            ],
        };

        let result = stratego_board.moving(
            create_full_case(Coordinate::new(1, 0), sergeant),
            Coordinate::new(1, 1),
        );

        match result {
            Ok(_) => panic!("Should not happen"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn should_move_and_capture() {
        let sergeant = Piece::new(PieceType::Sergeant, Box::new(Color::Blue));
        let lieutenant = Piece::new(PieceType::Lieutenant, Box::new(Color::Red));
        let mut stratego_board = StrategoBoard {
            cases: vec![vec![
                create_full_case(Coordinate::new(0, 0), lieutenant.clone()),
                create_full_case(Coordinate::new(0, 0), sergeant.clone()),
            ]],
        };

        let result = stratego_board.moving(
            create_full_case(Coordinate::new(0, 0), lieutenant),
            Coordinate::new(0, 1),
        );

        match result {
            Ok(cases) => {
                let previous_case = cases.get(0).unwrap();
                assert_eq!(&State::Empty, previous_case.get_state());

                let actual_case = cases.get(1).unwrap();
                assert_eq!(&State::Full, actual_case.get_state());
                let piece = actual_case.get_content();
                assert_eq!(&Color::Red, piece.get_color());
                assert_eq!(&PieceType::Lieutenant, piece.get_rank());
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn should_not_move_in_diagonales() {
        let general = Piece::new(PieceType::General, Box::new(Color::Red));
        let mut stratego_board = StrategoBoard {
            cases: vec![
                vec![
                    create_full_case(Coordinate::new(0, 0), general.clone()),
                    create_empty_case(Coordinate::new(0, 1)),
                ],
                vec![
                    create_empty_case(Coordinate::new(1, 0)),
                    create_empty_case(Coordinate::new(1, 1)),
                ],
            ],
        };

        let result = stratego_board.moving(
            create_full_case(Coordinate::new(0, 0), general),
            Coordinate::new(1, 1),
        );

        match result {
            Ok(_) => panic!("Should not happen"),
            Err(_) => assert!(true),
        }
    }
}
