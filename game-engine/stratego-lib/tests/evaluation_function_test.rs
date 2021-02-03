#[cfg(test)]
mod evaluation_tests {

    use stratego_lib::board::case::{self, Coordinate};
    use stratego_lib::board::classic_board;
    use stratego_lib::board::piece::Color;
    use stratego_lib::board::piece::{Piece, PieceType};
    use stratego_lib::board::Board;
    use stratego_lib::py_bindings::evaluation_function;

    #[test]
    fn should_get_winner() {
        //Win because blue can't move
        let mut board = classic_board::create_empty_stratego_board();
        //Reds
        board.place(case::create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Flag, Color::Red),
        )).unwrap();
        board.place(case::create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Captain, Color::Red),
        )).unwrap();
        //Blues
        board.place(case::create_full_case(
            Coordinate::new(9, 0),
            Piece::new(PieceType::Flag, Color::Blue),
        )).unwrap();
        assert_eq!(
            Some(Color::Red),
            evaluation_function::basic_evaluation(&board)
        );

        //Win because red has no Flag anymore
        let mut board = classic_board::create_empty_stratego_board();
        //Reds
        board.place(case::create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Captain, Color::Red),
        )).unwrap();
        //Blues
        board.place(case::create_full_case(
            Coordinate::new(9, 0),
            Piece::new(PieceType::Flag, Color::Blue),
        )).unwrap();
        board.place(case::create_full_case(
            Coordinate::new(9, 1),
            Piece::new(PieceType::Captain, Color::Blue),
        )).unwrap();
        assert_eq!(
            Some(Color::Blue),
            evaluation_function::basic_evaluation(&board)
        );
    }

    #[test]
    fn should_have_no_winner_yet() {
        let mut board = classic_board::create_empty_stratego_board();
        //Reds
        board
            .place(case::create_full_case(
                Coordinate::new(0, 0),
                Piece::new(PieceType::Flag, Color::Red),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(0, 1),
                Piece::new(PieceType::Captain, Color::Red),
            ))
            .unwrap();

        //Blues
        board
            .place(case::create_full_case(
                Coordinate::new(9, 0),
                Piece::new(PieceType::Flag, Color::Blue),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(9, 1),
                Piece::new(PieceType::Captain, Color::Blue),
            ))
            .unwrap();
        assert_eq!(None, evaluation_function::basic_evaluation(&board));

        let board = classic_board::create_stratego_board();
        assert_eq!(None, evaluation_function::basic_evaluation(&board));
    }

    #[test]
    fn should_get_advantage_for_reds() {
        let mut board = classic_board::create_empty_stratego_board();
        board.place(case::create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Captain, Color::Blue)
        )).unwrap();

        eprintln!("{}", board.display());
        let (red, blue) = evaluation_function::material_evaluation(&board);
        assert_eq!((Color::Red, 14), red);
        assert_eq!((Color::Blue, 14), blue);
    }
}
