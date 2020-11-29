use crate::error::StrategoError;

use super::board_utils::{attack, check_piece_move};
use super::case::{
    create_empty_case, create_full_case, create_unreachable_case, Case, Coordinate, State,
};
use super::Board;

pub struct StrategoBoard {
    cases: Vec<Vec<Case>>,
}

pub fn create_empty_stratego_board() -> Box<dyn Board> {
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

    Box::new(StrategoBoard { cases: board })
}

impl Board for StrategoBoard {
    fn moving(&mut self, case: Case, to: Coordinate) -> Result<Vec<Case>, StrategoError> {
        if check_piece_move(&case, &to) {
            return Err(StrategoError::MoveError(case, to));
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
                let new_case = create_full_case(to.clone(), piece.to_owned());
                self.cases[to_x as usize][to_y as usize] = new_case.clone();
                let case_coord = case.get_coordinate();
                self.cases[case_coord.get_x() as usize][case_coord.get_y() as usize] =
                    create_empty_case(case_coord.to_owned());
                Ok(vec![new_case])
            }
            State::Full => {
                let case_coord = case.get_coordinate();
                let result = attack(
                    create_full_case(case_coord.to_owned(), piece.to_owned()),
                    aim_case.to_owned(),
                );
                self.cases[case_coord.get_x() as usize][case_coord.get_y() as usize] =
                    result.0.clone();
                self.cases[to_x as usize][to_y as usize] = result.1.clone();
                Ok(vec![result.0, result.1])
            }
            State::Unreachable => Err(StrategoError::MoveError(case, to)),
        }
    }

    fn state(&self) -> &Vec<Vec<Case>> {
        &self.cases
    }

    fn display(&self) -> String {
        let mut display = String::from(" | ");
        for i in 0..self.cases.len() {
            display.push_str(format!("  {}   |  ", i).as_str());
        }
        display.push('\n');
        for i in 0..self.cases.len() {
            let row = self.cases.get(i).unwrap();
            display.push_str(
                format!(
                    "{}|{}\n",
                    i,
                    row.iter()
                        .map(|case| format!(" {} | ", case.display()))
                        .collect::<String>()
                )
                .as_str(),
            );
        }
        display
    }
}

#[cfg(test)]
mod test {
    use super::create_empty_stratego_board;

    #[test]
    fn should_build_stratego_board() {
        let stratego_board = create_empty_stratego_board();
        let state = stratego_board.state();

        println!("{}", stratego_board.display());
        assert_eq!(state.len(), 10);
        assert_eq!(state[0].len(), 10);
    }

    //#[test]

}
