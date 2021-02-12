#[cfg(test)]
mod py_wrapper_tests {
    use stratego_lib::board::case::{self, Coordinate};
    use stratego_lib::board::piece::{Color, Piece, PieceType, PyPieceType};
    use stratego_lib::board::Board;
    use stratego_lib::engine_utils;
    use stratego_lib::py_bindings::board_wrapper;
    use stratego_lib::py_bindings::board_wrapper::StrategoBoardWrapper;
    use stratego_lib::py_bindings::evaluation_function;
    use stratego_lib::simulation;

    #[test]
    fn should_simulate_game_between_two_ais() {
        let mut board = engine_utils::create_empty_stratego_board();

        board
            .place(case::create_full_case(
                Coordinate::new(0, 0),
                Piece::new(PieceType::General, Color::Red),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(0, 1),
                Piece::new(PieceType::Bomb, Color::Blue),
            ))
            .unwrap();

        if let Ok(res_simulation) = simulation::simulate(
            &board,
            &simulation::choose_randomly,
            &simulation::choose_randomly,
            &evaluation_function::basic_evaluation,
            50,
            Some(Color::Red),
            Color::Red,
        ) {
            assert_eq!(
                res_simulation,
                (Coordinate::new(0, 0), Coordinate::new(0, 1))
            );
        } else {
            assert!(false);
        }
    }

    #[test]
    fn should_parse_rust_content_to_python() {
        let mut board = StrategoBoardWrapper::new(engine_utils::create_empty_stratego_board());
        let case = board.place(
            String::from("Full"),
            (1, String::from("A")),
            1,
            String::from("Red"),
        );
        assert!(case.is_ok());
    }

    #[test]
    fn should_create_board_from_for_python() {
        let py_cases = vec![
            vec![(
                String::from("Full"),
                1,
                (0, String::from("A")),
                String::from("Red"),
            )],
            vec![(
                String::from("Full"),
                4,
                (3, String::from("A")),
                String::from("Blue"),
            )],
        ];

        let expected = vec![
            vec![case::create_full_case(
                Coordinate::new(0, 0),
                Piece::new(PieceType::Spy, Color::Red),
            )],
            vec![case::create_full_case(
                Coordinate::new(3, 0),
                Piece::new(PieceType::Sergeant, Color::Blue),
            )],
        ];
        let cases = board_wrapper::parse_python_cases(py_cases);
        assert_eq!(expected[0][0], cases[0][0]);
        assert_eq!(expected[1][0], cases[1][0]);
    }

    #[test]
    fn should_ask_for_material_evaluation() {
        let material_values: Vec<(PyPieceType, i16)> = vec![
            (-2, 0),
            (10, 10),
            (9, 9),
            (8, 8),
            (7, 7),
            (6, 6),
            (5, 5),
            (4, 4),
            (3, 3),
            (2, 2),
            (1, 1),
            (1, 1),
        ];

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

        let board = StrategoBoardWrapper::new(board);
        assert!(board.material_evaluation(material_values).is_ok());
    }
}
