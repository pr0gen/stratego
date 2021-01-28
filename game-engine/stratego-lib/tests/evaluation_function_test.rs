#[cfg(test)]
mod evaluation_tests {

    use stratego_lib::board::case::{self, Coordinate};
    use stratego_lib::board::classic_board;
    use stratego_lib::board::classic_board::StrategoBoard;
    use stratego_lib::board::piece::Color;
    use stratego_lib::board::piece::{Piece, PieceType};
    use stratego_lib::board::Board;
    use stratego_lib::py_bindings::evaluation_function;

    #[test]
    fn should_get_winner() {
        //Win because blue can't move
        let board = classic_board::create_empty_stratego_board();
        let mut cases = board.state().to_owned();
        //Reds
        cases[0][0] = case::create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Flag, Color::Red),
        );
        cases[0][1] = case::create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Captain, Color::Red),
        );
        //Blues
        cases[9][0] = case::create_full_case(
            Coordinate::new(9, 0),
            Piece::new(PieceType::Flag, Color::Blue),
        );
        let board = StrategoBoard::new(cases);
        assert_eq!(
            Some(Color::Red),
            evaluation_function::basic_evaluation(&board)
        );

        //Win because red has no Flag anymore
        let board = classic_board::create_empty_stratego_board();
        let mut cases = board.state().to_owned();
        //Reds
        cases[0][1] = case::create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Captain, Color::Red),
        );
        //Blues
        cases[9][0] = case::create_full_case(
            Coordinate::new(9, 0),
            Piece::new(PieceType::Flag, Color::Blue),
        );
        cases[9][1] = case::create_full_case(
            Coordinate::new(9, 1),
            Piece::new(PieceType::Captain, Color::Blue),
        );
        let board = StrategoBoard::new(cases);
        assert_eq!(
            Some(Color::Blue),
            evaluation_function::basic_evaluation(&board)
        );
    }

    #[test]
    fn should_have_no_winner_yet() {
        let board = classic_board::create_empty_stratego_board();
        let mut cases = board.state().to_owned();
        //Reds
        cases[0][0] = case::create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Flag, Color::Red),
        );
        cases[0][1] = case::create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Captain, Color::Red),
        );
        //Blues
        cases[9][0] = case::create_full_case(
            Coordinate::new(9, 0),
            Piece::new(PieceType::Flag, Color::Blue),
        );
        cases[9][1] = case::create_full_case(
            Coordinate::new(9, 1),
            Piece::new(PieceType::Captain, Color::Blue),
        );
        let board = StrategoBoard::new(cases);
        assert_eq!(None, evaluation_function::basic_evaluation(&board));

        let board = classic_board::create_stratego_board();
        assert_eq!(None, evaluation_function::basic_evaluation(&board));
    }
}
