#[cfg(test)]
mod engine_utils_tests {

    use stratego_lib::board::case::{
        create_empty_case, create_full_case, create_unreachable_case, Case, Coordinate,
    };
    use stratego_lib::board::classic_board::StrategoBoard;
    use stratego_lib::board::piece::{Color, Piece, PieceType};
    use stratego_lib::board::Board;

    use stratego_lib::engine_utils::{game_is_over, get_availables_moves, verify_board_integrity};

    #[test]
    fn should_check_scouts_can_not_move_behind_pieces() {
        let mut cases = empty_board();
        cases[8][1] = create_full_case(
            Coordinate::new(8, 1),
            Piece::new(PieceType::Scout, Color::Red),
        );
        cases[8][3] = create_full_case(
            Coordinate::new(8, 3),
            Piece::new(PieceType::Bomb, Color::Blue),
        );
        cases[8][4] = create_full_case(
            Coordinate::new(8, 4),
            Piece::new(PieceType::Bomb, Color::Blue),
        );

        cases[2][1] = create_full_case(
            Coordinate::new(2, 1),
            Piece::new(PieceType::Bomb, Color::Blue),
        );

        let board = &StrategoBoard::new(cases);
        eprintln!("{}", board.display());
        let res = get_availables_moves(board);
        eprintln!("{:?}", res);
        assert_eq!(10, res.len());
    }

    #[test]
    fn should_check_scout_move_scouts_alone() {
        let mut cases = empty_board();
        cases[2][5] = create_full_case(
            Coordinate::new(2, 5),
            Piece::new(PieceType::Scout, Color::Red),
        );
        cases[8][1] = create_full_case(
            Coordinate::new(8, 1),
            Piece::new(PieceType::Scout, Color::Red),
        );

        let board = &StrategoBoard::new(cases);
        eprintln!("{}", board.display());
        let res = get_availables_moves(board);
        eprintln!("{:?}", res);
        assert_eq!(36, res.len());
    }

    #[test]
    fn should_check_scout_move_scouts_with_others() {
        let mut cases = empty_board();
        cases[2][5] = create_full_case(
            Coordinate::new(2, 5),
            Piece::new(PieceType::Scout, Color::Red),
        );
        cases[9][5] = create_full_case(
            Coordinate::new(9, 5),
            Piece::new(PieceType::Bomb, Color::Red),
        );
        cases[8][1] = create_full_case(
            Coordinate::new(8, 1),
            Piece::new(PieceType::Scout, Color::Red),
        );
        cases[8][3] = create_full_case(
            Coordinate::new(8, 3),
            Piece::new(PieceType::Bomb, Color::Red),
        );

        let board = &StrategoBoard::new(cases);
        eprintln!("{}", board.display());
        let res = get_availables_moves(board);
        eprintln!("{:?}", res);
        assert_eq!(28, res.len());
    }

    //#[test]
    //fn should_check_scout_does_not_cross_water() {
        //let mut cases = empty_board();
        //cases[4][1] = create_full_case(
            //Coordinate::new(4, 1),
            //Piece::new(PieceType::Scout, Color::Red),
        //);
        //cases[5][1] = create_full_case(
            //Coordinate::new(9, 5),
            //Piece::new(PieceType::Bomb, Color::Blue),
        //);

        //let board = &StrategoBoard::new(cases);
        //eprintln!("{}", board.display());
        //let res = get_availables_moves(board);
        //eprintln!("{:?}", res);
        //assert_eq!(5, res.len());
    //}

    #[test]
    fn should_retrieve_available_moves_3x3() {
        let cases = create_3_x_3_stratego_board();
        let res = get_availables_moves(&StrategoBoard::new(cases));
        assert_eq!(4, res.len());
    }

    #[test]
    fn should_retrieve_available_moves_4x4() {
        let cases = create_4_x_4_stratego_board();
        let res = get_availables_moves(&StrategoBoard::new(cases));
        assert_eq!(6, res.len());
    }

    #[test]
    fn should_retrieve_and_detect_capture() {
        let mut cases = create_3_x_3_stratego_board();
        cases[1][1] = create_full_case(
            Coordinate::new(1, 1),
            Piece::new(PieceType::Sergeant, Color::Red),
        );

        let res = get_availables_moves(&StrategoBoard::new(cases));
        assert_eq!(6, res.len());
    }

    #[test]
    fn game_should_be_over_because_blue_has_looses_flag() {
        let mut cases = empty_board();

        //Red
        cases[0][0] = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Flag, Color::Red),
        );
        cases[0][1] = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::General, Color::Red),
        );

        //Blue
        cases[9][1] = create_full_case(
            Coordinate::new(9, 1),
            Piece::new(PieceType::General, Color::Blue),
        );

        let res = game_is_over(&cases);
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
        cases[0][0] = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Flag, Color::Red),
        );
        cases[0][1] = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::General, Color::Red),
        );

        //Blue
        cases[9][0] = create_full_case(
            Coordinate::new(9, 0),
            Piece::new(PieceType::Flag, Color::Blue),
        );

        let res = game_is_over(&cases);
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
        cases[0][0] = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Flag, Color::Red),
        );
        cases[0][1] = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::General, Color::Red),
        );

        //Blue
        cases[9][0] = create_full_case(
            Coordinate::new(9, 0),
            Piece::new(PieceType::Flag, Color::Blue),
        );
        cases[9][1] = create_full_case(
            Coordinate::new(9, 1),
            Piece::new(PieceType::General, Color::Blue),
        );

        let res = game_is_over(&cases);
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
                create_empty_case(Coordinate::new(0, 0)),
                create_empty_case(Coordinate::new(0, 1)),
            ],
            vec![
                create_empty_case(Coordinate::new(1, 0)),
                create_empty_case(Coordinate::new(1, 1)),
            ],
        ];

        let res = verify_board_integrity(StrategoBoard::new(new_board));

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

        new_board[4][2] = create_full_case(
            Coordinate::new(4, 2),
            Piece::new(PieceType::Spy, Color::Blue),
        );

        let b = StrategoBoard::new(new_board.clone());
        println!("{}", b.display());
        let res = verify_board_integrity(StrategoBoard::new(new_board));

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
                new_board[i].push(create_full_case(
                    Coordinate::new(i as i16, j as i16),
                    Piece::new(PieceType::Spy, Color::Blue),
                ));
            }
        }

        new_board[4][2] = create_unreachable_case(Coordinate::new(4, 2));
        new_board[4][3] = create_unreachable_case(Coordinate::new(4, 3));
        new_board[5][2] = create_unreachable_case(Coordinate::new(5, 2));
        new_board[5][3] = create_unreachable_case(Coordinate::new(5, 3));
        new_board[4][6] = create_unreachable_case(Coordinate::new(4, 6));
        new_board[4][7] = create_unreachable_case(Coordinate::new(4, 7));
        new_board[5][6] = create_unreachable_case(Coordinate::new(5, 6));
        new_board[5][7] = create_unreachable_case(Coordinate::new(7, 5));

        let res = verify_board_integrity(StrategoBoard::new(new_board));
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
        new_board[0][4] = create_full_case(
            Coordinate::new(0, 4),
            Piece::new(PieceType::Spy, Color::Red),
        );
        let res = verify_board_integrity(StrategoBoard::new(new_board));
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
        //let board = StrategoBoard::new(cases);
        //console.log(board.display());
        let res = verify_board_integrity(StrategoBoard::new(cases));

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

    //#[test]
    //fn should_verify_board_integrity() {
    //let res = verify_board_integrity(create_statego_board());
    //match res {
    //Ok(_) => assert!(true),
    //Err(_) => panic!("Should not happen"),
    //}
    //}
    fn create_4_x_4_stratego_board() -> Vec<Vec<Case>> {
        vec![
            vec![
                create_full_case(
                    Coordinate::new(0, 0),
                    Piece::new(PieceType::Flag, Color::Blue),
                ),
                create_full_case(
                    Coordinate::new(0, 1),
                    Piece::new(PieceType::Major, Color::Blue),
                ),
                create_full_case(
                    Coordinate::new(0, 2),
                    Piece::new(PieceType::Spy, Color::Blue),
                ),
                create_full_case(
                    Coordinate::new(0, 3),
                    Piece::new(PieceType::Spy, Color::Blue),
                ),
            ],
            vec![
                create_empty_case(Coordinate::new(1, 0)),
                create_empty_case(Coordinate::new(1, 1)),
                create_empty_case(Coordinate::new(1, 2)),
                create_empty_case(Coordinate::new(1, 3)),
            ],
            vec![
                create_empty_case(Coordinate::new(2, 0)),
                create_empty_case(Coordinate::new(2, 1)),
                create_empty_case(Coordinate::new(2, 2)),
                create_empty_case(Coordinate::new(2, 3)),
            ],
            vec![
                create_full_case(
                    Coordinate::new(3, 0),
                    Piece::new(PieceType::Flag, Color::Red),
                ),
                create_full_case(
                    Coordinate::new(3, 1),
                    Piece::new(PieceType::Major, Color::Red),
                ),
                create_full_case(
                    Coordinate::new(3, 2),
                    Piece::new(PieceType::Spy, Color::Red),
                ),
                create_full_case(
                    Coordinate::new(3, 3),
                    Piece::new(PieceType::Spy, Color::Red),
                ),
            ],
        ]
    }

    pub fn create_3_x_3_stratego_board() -> Vec<Vec<Case>> {
        vec![
            vec![
                create_full_case(
                    Coordinate::new(0, 0),
                    Piece::new(PieceType::Flag, Color::Blue),
                ),
                create_full_case(
                    Coordinate::new(0, 1),
                    Piece::new(PieceType::Major, Color::Blue),
                ),
                create_full_case(
                    Coordinate::new(0, 2),
                    Piece::new(PieceType::Spy, Color::Blue),
                ),
            ],
            vec![
                create_empty_case(Coordinate::new(1, 0)),
                create_empty_case(Coordinate::new(1, 1)),
                create_empty_case(Coordinate::new(1, 2)),
            ],
            vec![
                create_full_case(
                    Coordinate::new(2, 0),
                    Piece::new(PieceType::Flag, Color::Red),
                ),
                create_full_case(
                    Coordinate::new(2, 1),
                    Piece::new(PieceType::Major, Color::Red),
                ),
                create_full_case(
                    Coordinate::new(2, 2),
                    Piece::new(PieceType::Spy, Color::Red),
                ),
            ],
        ]
    }

    //Good board
    fn create_statego_board() -> Vec<Vec<Case>> {
        let mut new_board = Vec::with_capacity(10);
        for i in 0..10 {
            new_board.push(Vec::with_capacity(10));
            for j in 0..10 {
                if i < 4 {
                    new_board[i].push(create_full_case(
                        Coordinate::new(i as i16, j as i16),
                        Piece::new(PieceType::Spy, Color::Blue),
                    ));
                } else if i > 5 {
                    new_board[i].push(create_full_case(
                        Coordinate::new(i as i16, j as i16),
                        Piece::new(PieceType::Spy, Color::Red),
                    ));
                } else {
                    new_board[i].push(create_empty_case(Coordinate::new(i as i16, j as i16)));
                }
            }
        }

        new_board[4][2] = create_unreachable_case(Coordinate::new(4, 2));
        new_board[4][3] = create_unreachable_case(Coordinate::new(4, 3));
        new_board[5][2] = create_unreachable_case(Coordinate::new(5, 2));
        new_board[5][3] = create_unreachable_case(Coordinate::new(5, 3));
        new_board[4][6] = create_unreachable_case(Coordinate::new(4, 6));
        new_board[4][7] = create_unreachable_case(Coordinate::new(4, 7));
        new_board[5][6] = create_unreachable_case(Coordinate::new(5, 6));
        new_board[5][7] = create_unreachable_case(Coordinate::new(7, 5));

        new_board
    }

    fn empty_board() -> Vec<Vec<Case>> {
        let size = 10;
        let mut board = Vec::with_capacity(size);
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
        board[5][7] = create_unreachable_case(Coordinate::new(5, 7));

        return board;
    }
}
