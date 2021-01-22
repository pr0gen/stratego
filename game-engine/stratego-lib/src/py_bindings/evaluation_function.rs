use crate::board::piece::Color;
use crate::board::Board;
use crate::engine_utils;

pub fn basic_evaluation(board: &impl Board) -> Option<Color> {
    engine_utils::game_is_over(board.state())
}

#[cfg(test)]
mod test {

    use crate::board::classic_board;
    use crate::board::piece::Color;
    use crate::board::Board;
    use crate::board::classic_board::StrategoBoard;
    use crate::board::case::{self, Coordinate};
    use crate::board::piece::{Piece, PieceType};

    #[test]
    fn should_get_winner() {
        //Win because blue can't move
        let board = classic_board::create_empty_stratego_board();
        let mut cases = board.state().to_owned();
        //Reds
        cases[0][0] = case::create_full_case(Coordinate::new(0, 0), Piece::new(PieceType::Flag, Color::Red));
        cases[0][1] = case::create_full_case(Coordinate::new(0, 1), Piece::new(PieceType::Captain, Color::Red));
        //Blues
        cases[9][0] = case::create_full_case(Coordinate::new(9, 0), Piece::new(PieceType::Flag, Color::Blue));
        let board = StrategoBoard::new(cases);
        assert_eq!(Some(Color::Red), super::basic_evaluation(&board));

        //Win because red has no Flag anymore
        let board = classic_board::create_empty_stratego_board();
        let mut cases = board.state().to_owned();
        //Reds
        cases[0][1] = case::create_full_case(Coordinate::new(0, 1), Piece::new(PieceType::Captain, Color::Red));
        //Blues
        cases[9][0] = case::create_full_case(Coordinate::new(9, 0), Piece::new(PieceType::Flag, Color::Blue));
        cases[9][1] = case::create_full_case(Coordinate::new(9, 1), Piece::new(PieceType::Captain, Color::Blue));
        let board = StrategoBoard::new(cases);
        assert_eq!(Some(Color::Blue), super::basic_evaluation(&board));
    }

    #[test]
    fn should_have_no_winner_yet() {
        let board = classic_board::create_empty_stratego_board();
        let mut cases = board.state().to_owned();
        //Reds
        cases[0][0] = case::create_full_case(Coordinate::new(0, 0), Piece::new(PieceType::Flag, Color::Red));
        cases[0][1] = case::create_full_case(Coordinate::new(0, 1), Piece::new(PieceType::Captain, Color::Red));
        //Blues
        cases[9][0] = case::create_full_case(Coordinate::new(9, 0), Piece::new(PieceType::Flag, Color::Blue));
        cases[9][1] = case::create_full_case(Coordinate::new(9, 1), Piece::new(PieceType::Captain, Color::Blue));
        let board = StrategoBoard::new(cases);
        assert_eq!(None, super::basic_evaluation(&board));

        let board = classic_board::create_stratego_board();
        assert_eq!(None, super::basic_evaluation(&board));
    }
}
