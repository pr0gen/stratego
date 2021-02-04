#[cfg(test)]
mod engine_utils_tests {

    use stratego_lib::board::case::{self, Case, Coordinate};
    use stratego_lib::board::classic_board::StrategoBoard;
    use stratego_lib::board::piece::{Color, Piece, PieceType};
    use stratego_lib::board::Board;
    use stratego_lib::engine_utils;


    #[test]
    fn game_should_be_over_because_blue_has_looses_flag() {
        let mut cases = empty_board();

        //Red
        cases[0][0] = case::create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Flag, Color::Red),
        );
        cases[0][1] = case::create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::General, Color::Red),
        );

        //Blue
        cases[9][1] = case::create_full_case(
            Coordinate::new(9, 1),
            Piece::new(PieceType::General, Color::Blue),
        );

        let res = engine_utils::game_is_over(&cases);
        match res {
            Some(val) => {
                assert_eq!(Color::Red, val);
            }
            None => panic!("Should not happen"),
        }
    }
    #[test]
    fn game_should_be_over_because_blue_has_looses_moveable_pieces() {
        let mut cases = empty_board();

        //Red
        cases[0][0] = case::create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Flag, Color::Red),
        );
        cases[0][1] = case::create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::General, Color::Red),
        );

        //Blue
        cases[9][0] = case::create_full_case(
            Coordinate::new(9, 0),
            Piece::new(PieceType::Flag, Color::Blue),
        );

        let res = engine_utils::game_is_over(&cases);
        match res {
            Some(val) => {
                assert_eq!(Color::Red, val);
            }
            None => panic!("Should not happen"),
        }
    }

    #[test]
    fn game_should_not_be_over() {
        let mut cases = empty_board();

        //Red
        cases[0][0] = case::create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Flag, Color::Red),
        );
        cases[0][1] = case::create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::General, Color::Red),
        );

        //Blue
        cases[9][0] = case::create_full_case(
            Coordinate::new(9, 0),
            Piece::new(PieceType::Flag, Color::Blue),
        );
        cases[9][1] = case::create_full_case(
            Coordinate::new(9, 1),
            Piece::new(PieceType::General, Color::Blue),
        );

        let res = engine_utils::game_is_over(&cases);
        match res {
            Some(_) => panic!("Should not happen"),
            None => {
                assert!(true);
            }
        }
    }

    #[test]
    fn should_not_verify_board_integrity_cause_to_small() {
        let new_board = vec![
            vec![
                case::create_empty_case(Coordinate::new(0, 0)),
                case::create_empty_case(Coordinate::new(0, 1)),
            ],
            vec![
                case::create_empty_case(Coordinate::new(1, 0)),
                case::create_empty_case(Coordinate::new(1, 1)),
            ],
        ];

        let res = engine_utils::verify_board_integrity(StrategoBoard::new(new_board));

        match res {
            Ok(_) => panic!("Should not happen"),
            Err(e) => {
                assert!(true);
                assert_eq!(
                    e.message(),
                    String::from("Board is not official, GO OUT OF THERE !!")
                );
            }
        }
    }

    #[test]
    fn should_not_verify_board_integrity_cause_lakes_are_not_empty() {
        let mut new_board = create_statego_board();

        new_board[4][2] = case::create_full_case(
            Coordinate::new(4, 2),
            Piece::new(PieceType::Spy, Color::Blue),
        );

        let b = StrategoBoard::new(new_board.clone());
        println!("{}", b.display());
        let res = engine_utils::verify_board_integrity(StrategoBoard::new(new_board));

        match res {
            Ok(_) => panic!("Should not happen"),
            Err(e) => {
                assert!(true);
                assert_eq!(
                    e.message(),
                    String::from("You can not place pieces in lakes, please check again")
                );
            }
        }
    }

    #[test]
    fn should_check_only_2_rows_in_middle_are_empty() {
        let mut new_board = Vec::with_capacity(10);
        for i in 0..10 {
            new_board.push(Vec::with_capacity(10));
            for j in 0..10 {
                new_board[i].push(case::create_full_case(
                    Coordinate::new(i as i16, j as i16),
                    Piece::new(PieceType::Spy, Color::Blue),
                ));
            }
        }

        new_board[4][2] = case::create_unreachable_case(Coordinate::new(4, 2));
        new_board[4][3] = case::create_unreachable_case(Coordinate::new(4, 3));
        new_board[5][2] = case::create_unreachable_case(Coordinate::new(5, 2));
        new_board[5][3] = case::create_unreachable_case(Coordinate::new(5, 3));
        new_board[4][6] = case::create_unreachable_case(Coordinate::new(4, 6));
        new_board[4][7] = case::create_unreachable_case(Coordinate::new(4, 7));
        new_board[5][6] = case::create_unreachable_case(Coordinate::new(5, 6));
        new_board[5][7] = case::create_unreachable_case(Coordinate::new(7, 5));

        let res = engine_utils::verify_board_integrity(StrategoBoard::new(new_board));
        match res {
            Ok(_) => panic!("Should not happen"),
            Err(e) => {
                assert!(true);
                assert_eq!(
                    e.message(),
                    String::from("The 2 rows in the middle must be empty, :(")
                );
            }
        }
    }

    #[test]
    fn should_check_players_have_placed_theirs_pieces_in_the_four_rows() {
        let mut new_board = create_statego_board();
        new_board[0][4] = case::create_full_case(
            Coordinate::new(0, 4),
            Piece::new(PieceType::Spy, Color::Red),
        );
        let res = engine_utils::verify_board_integrity(StrategoBoard::new(new_board));
        match res {
            Ok(_) => panic!("Should not happen"),
            Err(e) => {
                assert!(true);
                assert_eq!(
                    e.message(),
                    String::from("Players must place theirs pieces in the right place !")
                );
            }
        }
    }

    #[test]
    fn should_check_players_have_the_right_pieces() {
        let cases = create_statego_board();
        let res = engine_utils::verify_board_integrity(StrategoBoard::new(cases));

        match res {
            Ok(_) => panic!("Should not happen"),
            Err(e) => {
                assert!(true);
                assert_eq!(
                    e.message(),
                    String::from("You need to start with the right pieces")
                );
            }
        }
    }


    //Good board
    fn create_statego_board() -> Vec<Vec<Case>> {
        let mut new_board = Vec::with_capacity(10);
        for i in 0..10 {
            new_board.push(Vec::with_capacity(10));
            for j in 0..10 {
                if i < 4 { new_board[i].push(case::create_full_case(
                        Coordinate::new(i as i16, j as i16),
                        Piece::new(PieceType::Spy, Color::Blue),
                    ));
                } else if i > 5 {
                    new_board[i].push(case::create_full_case(
                        Coordinate::new(i as i16, j as i16),
                        Piece::new(PieceType::Spy, Color::Red),
                    ));
                } else {
                    new_board[i].push(case::create_empty_case(Coordinate::new(i as i16, j as i16)));
                }
            }
        }

        new_board[4][2] = case::create_unreachable_case(Coordinate::new(4, 2));
        new_board[4][3] = case::create_unreachable_case(Coordinate::new(4, 3));
        new_board[5][2] = case::create_unreachable_case(Coordinate::new(5, 2));
        new_board[5][3] = case::create_unreachable_case(Coordinate::new(5, 3));
        new_board[4][6] = case::create_unreachable_case(Coordinate::new(4, 6));
        new_board[4][7] = case::create_unreachable_case(Coordinate::new(4, 7));
        new_board[5][6] = case::create_unreachable_case(Coordinate::new(5, 6));
        new_board[5][7] = case::create_unreachable_case(Coordinate::new(5, 7));

        new_board
    }

    fn empty_board() -> Vec<Vec<Case>> {
        let size = 10;
        let mut board = Vec::with_capacity(size);
        for i in 0..size {
            board.push(Vec::with_capacity(size));
            for j in 0..size {
                board[i].push(case::create_empty_case(Coordinate::new(i as i16, j as i16)));
            }
        }

        board[4][2] = case::create_unreachable_case(Coordinate::new(4, 2));
        board[4][3] = case::create_unreachable_case(Coordinate::new(4, 3));
        board[5][2] = case::create_unreachable_case(Coordinate::new(5, 2));
        board[5][3] = case::create_unreachable_case(Coordinate::new(5, 3));
        board[4][6] = case::create_unreachable_case(Coordinate::new(4, 6));
        board[4][7] = case::create_unreachable_case(Coordinate::new(4, 7));
        board[5][6] = case::create_unreachable_case(Coordinate::new(5, 6));
        board[5][7] = case::create_unreachable_case(Coordinate::new(5, 7));

        return board;
    }
}
