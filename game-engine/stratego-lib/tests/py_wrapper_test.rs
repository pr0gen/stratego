#[cfg(test)]
mod py_wrapper_tests {
    use stratego_lib::board::case::{self, Coordinate};
    use stratego_lib::board::classic_board;
    use stratego_lib::board::piece::{Color, Piece, PieceType};
    use stratego_lib::py_bindings::board_wrapper;
    use stratego_lib::py_bindings::board_wrapper::StrategoBoardWrapper;

    #[test]
    fn should_parse_rust_content_to_python() {
        let mut board = StrategoBoardWrapper::new(classic_board::create_empty_stratego_board());
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

}
