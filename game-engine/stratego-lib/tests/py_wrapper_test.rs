#[cfg(test)]
mod py_wrapper_tests {
    use pyo3::PyResult;
    use pythonize::*;
    use stratego_lib::board::case::{self, Coordinate};
    use stratego_lib::board::piece::{Color, Piece, PieceType, PyPieceType};
    use stratego_lib::board::Board;
    use stratego_lib::engine_utils;
    use stratego_lib::py_bindings::board_wrapper;
    use stratego_lib::py_bindings::board_wrapper::StrategoBoardWrapper;
    use stratego_lib::simulation;
    use stratego_lib::simulation::evaluation::EvaluationFunction;
    use stratego_lib::simulation::evaluation::EvaluationFunctionResponse;
    use stratego_lib::utils;

    #[test]
    fn should_simulate_game_between_two_ais_red() -> PyResult<()> {
        let board = engine_utils::create_empty_stratego_board();
        let mut pyboard = StrategoBoardWrapper::new(board);

        pyboard.place(
            String::from("Full"),
            (0, String::from("A")),
            9,
            String::from("Red"),
        )?;
        pyboard.place(
            String::from("Full"),
            (0, String::from("B")),
            -1,
            String::from("Red"),
        )?;
        pyboard.place(
            String::from("Full"),
            (0, String::from("C")),
            -2,
            String::from("Red"),
        )?;
        pyboard.place(
            String::from("Full"),
            (9, String::from("A")),
            -2,
            String::from("Blue"),
        )?;
        pyboard.place(
            String::from("Full"),
            (8, String::from("A")),
            3,
            String::from("Blue"),
        )?;

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

        eprintln!("{}", pyboard.display()?);

        let stopping_criteria = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let eval = EvaluationFunction::Material(material_values, stopping_criteria, Color::Red);
        let res = simulation::simulate(
            &pyboard,
            &simulation::choose_randomly,
            &simulation::choose_randomly,
            &eval,
            &20,
        );
        match res {
            Ok((b, _)) => assert!(b),
            Err(e) => {
                eprintln!("{}", e.message());
                assert!(false)
            }
        }
        Ok(())
    }

    #[test]
    fn should_simulate_game_between_two_ais_blue() {
        let mut board = engine_utils::create_empty_stratego_board();

        board
            .place(case::create_full_case(
                Coordinate::new(0, 0),
                Piece::new(PieceType::General, Color::Blue),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(0, 1),
                Piece::new(PieceType::Bomb, Color::Red),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(0, 2),
                Piece::new(PieceType::Flag, Color::Red),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(9, 3),
                Piece::new(PieceType::Flag, Color::Blue),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(9, 4),
                Piece::new(PieceType::General, Color::Red),
            ))
            .unwrap();

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
        eprintln!("{}", board.display());

        let pyboard = StrategoBoardWrapper::new(board);
        let stopping_criteria = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let res = pyboard.simulate_games_material(
            material_values,
            stopping_criteria,
            20,
            String::from("Red"),
            4,
        );
        match res {
            Ok(res_simulation) => {
                let gil_holder = utils::get_gild_holder()
                    .unwrap_or_else(|e| panic!("Failed to get python gil holder, {}", e.message()));
                let gil = gil_holder.get();
                let vec = depythonize::<Vec<EvaluationFunctionResponse>>(
                    res_simulation.as_ref(gil.python()),
                )
                .unwrap();
                assert_ne!(0, vec.len());
            }
            Err(_) => assert!(false),
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
