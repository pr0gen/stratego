#[cfg(test)]
mod evaluation_tests {

    use stratego_lib::board::case::{self, Coordinate};
    use stratego_lib::board::piece::Color;
    use stratego_lib::board::piece::{Piece, PieceType};
    use stratego_lib::board::Board;
    use stratego_lib::engine_utils;
    use stratego_lib::py_bindings::evaluation_function;

    #[test]
    fn should_get_winner() {
        //Win because blue can't move
        let mut board = engine_utils::create_empty_stratego_board();
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
        assert_eq!(
            Some(Color::Red),
            evaluation_function::basic_evaluation(&board)
        );

        //Win because red has no Flag anymore
        let mut board = engine_utils::create_empty_stratego_board();
        //Reds
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
        assert_eq!(
            Some(Color::Blue),
            evaluation_function::basic_evaluation(&board)
        );
    }

    #[test]
    fn should_have_no_winner_yet() {
        let mut board = engine_utils::create_empty_stratego_board();
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

        let board = engine_utils::create_stratego_board();
        assert_eq!(None, evaluation_function::basic_evaluation(&board));
    }

    #[test]
    fn should_get_advantage_for_reds() {
        let mut board = engine_utils::create_empty_stratego_board();

        //Reds
        board
            .place(case::create_full_case(
                Coordinate::new(9, 0),
                Piece::new(PieceType::Lieutenant, Color::Red),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(9, 1),
                Piece::new(PieceType::Miner, Color::Red),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(9, 2),
                Piece::new(PieceType::Scout, Color::Red),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(9, 3),
                Piece::new(PieceType::Bomb, Color::Red),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(8, 3),
                Piece::new(PieceType::Sergeant, Color::Red),
            ))
            .unwrap();

        //Blues
        board
            .place(case::create_full_case(
                Coordinate::new(0, 0),
                Piece::new(PieceType::Captain, Color::Blue),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(0, 1),
                Piece::new(PieceType::Major, Color::Blue),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(0, 2),
                Piece::new(PieceType::Flag, Color::Blue),
            ))
            .unwrap();

        let material_values: Vec<(PieceType, i16)> = vec![
            (PieceType::Bomb, 0),
            (PieceType::Marshal, 10),
            (PieceType::General, 9),
            (PieceType::Colonel, 8),
            (PieceType::Major, 7),
            (PieceType::Captain, 6),
            (PieceType::Lieutenant, 5),
            (PieceType::Sergeant, 4),
            (PieceType::Miner, 3),
            (PieceType::Scout, 2),
            (PieceType::Spy, 1),
            (PieceType::Flag, 1),
        ];
        eprintln!("{}", board.display());
        assert_eq!(((Color::Red, 14), (Color::Blue, 14)), evaluation_function::material_evaluation(&board, &material_values));
    }
}
