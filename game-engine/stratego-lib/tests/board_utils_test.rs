#[cfg(test)]
mod board_utils_test {
    use stratego_lib::board::board_utils;
    use stratego_lib::board::case::{self, Case, Coordinate, State};
    use stratego_lib::board::classic_board::{self, StrategoBoard};
    use stratego_lib::board::piece::{Color, Piece, PieceType};
    use stratego_lib::board::Board;

    #[test]
    fn attacker_should_win() {
        let attacker = case::create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Colonel, Color::Blue),
        ); let defenser = case::create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Lieutenant, Color::Red),
        );

        let (attacker, defenser) = board_utils::attack(attacker, defenser).unwrap();
        assert_eq!(case::create_empty_case(Coordinate::new(0, 0)), attacker);
        assert_eq!(
            case::create_full_case(
                Coordinate::new(0, 1),
                Piece::new(PieceType::Colonel, Color::Blue)
            ),
            defenser
        );
    }

    #[test]
    fn defenser_should_win() {
        let attacker = case::create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Lieutenant, Color::Blue),
        );
        let defenser = case::create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Colonel, Color::Red),
        );

        let (attacker, defenser) = board_utils::attack(attacker, defenser).unwrap();
        assert_eq!(case::create_empty_case(Coordinate::new(0, 0)), attacker);
        assert_eq!(
            case::create_full_case(
                Coordinate::new(0, 1),
                Piece::new(PieceType::Colonel, Color::Red)
            ),
            defenser
        );
    }

    #[test]
    fn both_should_loose() {
        let attacker = case::create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Colonel, Color::Blue),
        );
        let defenser = case::create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Colonel, Color::Red),
        );

        let (attacker, defenser) = board_utils::attack(attacker, defenser).unwrap();

        assert_eq!(attacker.get_state(), &State::Empty);
        assert_eq!(defenser.get_state(), &State::Empty);
    }

    #[test]
    fn spy_should_kill_marshal() {
        let attacker = case::create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Spy, Color::Blue),
        );
        let defenser = case::create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Marshal, Color::Red),
        );
        let res = board_utils::attack(attacker, defenser).unwrap();

        assert_eq!(res.0.get_state(), &State::Empty);

        let defenser2 = res.1;
        assert_eq!(defenser2.get_state(), &State::Full);
        assert_eq!(defenser2.get_content().get_color(), &Color::Blue);
    }

    #[test]
    fn miner_should_kill_bomb() {
        let attacker = case::create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Miner, Color::Blue),
        );
        let defenser = case::create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Bomb, Color::Red),
        );
        let res = board_utils::attack(attacker, defenser).unwrap();

        assert_eq!(res.0.get_state(), &State::Empty);

        let defenser2 = res.1;
        assert_eq!(defenser2.get_state(), &State::Full);
        assert_eq!(defenser2.get_content().get_color(), &Color::Blue);
    }

    #[test]
    fn should_loose_against_bomb() {
        let attacker = case::create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::General, Color::Blue),
        );
        let defenser = case::create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Bomb, Color::Red),
        );
        let res = board_utils::attack(attacker, defenser).unwrap();

        assert_eq!(res.0.get_state(), &State::Empty);

        let defenser2 = res.1;
        assert_eq!(defenser2.get_state(), &State::Full);
        assert_eq!(defenser2.get_content().get_color(), &Color::Red);
    }

    #[test]
    fn should_loose_against_marshal() {
        let attacker = case::create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Lieutenant, Color::Blue),
        );
        let defenser = case::create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Marshal, Color::Red),
        );
        let res = board_utils::attack(attacker, defenser).unwrap();

        assert_eq!(res.0.get_state(), &State::Empty);

        let defenser2 = res.1;
        assert_eq!(defenser2.get_state(), &State::Full);
        assert_eq!(defenser2.get_content().get_color(), &Color::Red);
    }

    #[test]
    fn should_check_scouts_can_not_move_behind_pieces() {
        let mut board = classic_board::create_empty_stratego_board();

        board
            .place(case::create_full_case(
                Coordinate::new(8, 1),
                Piece::new(PieceType::Scout, Color::Red),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(8, 2),
                Piece::new(PieceType::Bomb, Color::Blue),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(8, 4),
                Piece::new(PieceType::Bomb, Color::Blue),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(2, 1),
                Piece::new(PieceType::Bomb, Color::Blue),
            ))
            .unwrap();

        eprintln!("{}", board.display());

        let res = board_utils::get_availables_moves(&board);
        eprintln!("{:?}", res);
        assert_eq!(9, res.len());
    }

    #[test]
    fn should_check_scout_move_scouts_alone() {
        let mut board = classic_board::create_empty_stratego_board();
        board
            .place(case::create_full_case(
                Coordinate::new(2, 5),
                Piece::new(PieceType::Scout, Color::Red),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(8, 1),
                Piece::new(PieceType::Scout, Color::Red),
            ))
            .unwrap();

        eprintln!("{}", board.display());
        let res = board_utils::get_availables_moves(&board);
        eprintln!("{:?}", res);
        assert_eq!(36, res.len());
    }

    #[test]
    fn should_get_move_for_scout() {
        let mut board = classic_board::create_empty_stratego_board();
        board
            .place(case::create_full_case(
                Coordinate::new(7, 5),
                Piece::new(PieceType::Scout, Color::Red),
            ))
            .unwrap();

        board
            .place(case::create_full_case(
                Coordinate::new(7, 4),
                Piece::new(PieceType::Lieutenant, Color::Red),
            ))
            .unwrap();

        board
            .place(case::create_full_case(
                Coordinate::new(7, 6),
                Piece::new(PieceType::Lieutenant, Color::Blue),
            ))
            .unwrap();

        eprintln!("{}", board.display());
        let res = board_utils::get_availables_moves(&board);
        eprintln!("{:?}", res);
        assert_eq!(17, res.len());
    }

    #[test]
    fn should_get_move_for_scout_by_color() {
        let mut board = classic_board::create_empty_stratego_board();
        board
            .place(case::create_full_case(
                Coordinate::new(7, 5),
                Piece::new(PieceType::Scout, Color::Red),
            ))
            .unwrap();

        board
            .place(case::create_full_case(
                Coordinate::new(7, 4),
                Piece::new(PieceType::Lieutenant, Color::Red),
            ))
            .unwrap();

        board
            .place(case::create_full_case(
                Coordinate::new(7, 6),
                Piece::new(PieceType::Lieutenant, Color::Blue),
            ))
            .unwrap();

        eprintln!("{}", board.display());
        let res = board_utils::get_availables_moves_by_color(&board, &Color::Red);
        eprintln!("{:?}", res);
        assert_eq!(13, res.len());
    }

    #[test]
    fn should_check_scout_move_scouts_with_others() {
        let mut board = classic_board::create_empty_stratego_board();
        board
            .place(case::create_full_case(
                Coordinate::new(2, 5),
                Piece::new(PieceType::Scout, Color::Red),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(9, 5),
                Piece::new(PieceType::Bomb, Color::Red),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(8, 1),
                Piece::new(PieceType::Scout, Color::Red),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(8, 3),
                Piece::new(PieceType::Bomb, Color::Red),
            ))
            .unwrap();
        eprintln!("{}", board.display());

        let res = board_utils::get_availables_moves(&board);
        eprintln!("{:?}", res);

        assert_eq!(28, res.len());
    }

    #[test]
    fn should_check_scout_does_not_cross_water() {
        let mut board = classic_board::create_empty_stratego_board();
        board
            .place(case::create_full_case(
                Coordinate::new(4, 1),
                Piece::new(PieceType::Scout, Color::Red),
            ))
            .unwrap();
        board
            .place(case::create_full_case(
                Coordinate::new(5, 1),
                Piece::new(PieceType::Bomb, Color::Blue),
            ))
            .unwrap();
        eprintln!("{}", board.display());
        let res = board_utils::get_availables_moves(&board);
        eprintln!("{:?}", res);
        assert_eq!(6, res.len());
    }

    #[test]
    fn should_retrieve_available_moves_3x3() {
        let cases = create_3_x_3_stratego_board();
        let res = board_utils::get_availables_moves(&StrategoBoard::new(cases));
        assert_eq!(4, res.len());
    }

    #[test]
    fn should_retrieve_available_moves_4x4() {
        let cases = create_4_x_4_stratego_board();
        let res = board_utils::get_availables_moves(&StrategoBoard::new(cases));
        assert_eq!(6, res.len());
    }

    #[test]
    fn should_retrieve_and_detect_capture() {
        let mut cases = create_3_x_3_stratego_board();
        cases[1][1] = case::create_full_case(
            Coordinate::new(1, 1),
            Piece::new(PieceType::Sergeant, Color::Red),
        );

        let res = board_utils::get_availables_moves(&StrategoBoard::new(cases));
        assert_eq!(6, res.len());
    }

    fn create_4_x_4_stratego_board() -> Vec<Vec<Case>> {
        vec![
            vec![
                case::create_full_case(
                    Coordinate::new(0, 0),
                    Piece::new(PieceType::Flag, Color::Blue),
                ),
                case::create_full_case(
                    Coordinate::new(0, 1),
                    Piece::new(PieceType::Major, Color::Blue),
                ),
                case::create_full_case(
                    Coordinate::new(0, 2),
                    Piece::new(PieceType::Spy, Color::Blue),
                ),
                case::create_full_case(
                    Coordinate::new(0, 3),
                    Piece::new(PieceType::Spy, Color::Blue),
                ),
            ],
            vec![
                case::create_empty_case(Coordinate::new(1, 0)),
                case::create_empty_case(Coordinate::new(1, 1)),
                case::create_empty_case(Coordinate::new(1, 2)),
                case::create_empty_case(Coordinate::new(1, 3)),
            ],
            vec![
                case::create_empty_case(Coordinate::new(2, 0)),
                case::create_empty_case(Coordinate::new(2, 1)),
                case::create_empty_case(Coordinate::new(2, 2)),
                case::create_empty_case(Coordinate::new(2, 3)),
            ],
            vec![
                case::create_full_case(
                    Coordinate::new(3, 0),
                    Piece::new(PieceType::Flag, Color::Red),
                ),
                case::create_full_case(
                    Coordinate::new(3, 1),
                    Piece::new(PieceType::Major, Color::Red),
                ),
                case::create_full_case(
                    Coordinate::new(3, 2),
                    Piece::new(PieceType::Spy, Color::Red),
                ),
                case::create_full_case(
                    Coordinate::new(3, 3),
                    Piece::new(PieceType::Spy, Color::Red),
                ),
            ],
        ]
    }

    pub fn create_3_x_3_stratego_board() -> Vec<Vec<Case>> {
        vec![
            vec![
                case::create_full_case(
                    Coordinate::new(0, 0),
                    Piece::new(PieceType::Flag, Color::Blue),
                ),
                case::create_full_case(
                    Coordinate::new(0, 1),
                    Piece::new(PieceType::Major, Color::Blue),
                ),
                case::create_full_case(
                    Coordinate::new(0, 2),
                    Piece::new(PieceType::Spy, Color::Blue),
                ),
            ],
            vec![
                case::create_empty_case(Coordinate::new(1, 0)),
                case::create_empty_case(Coordinate::new(1, 1)),
                case::create_empty_case(Coordinate::new(1, 2)),
            ],
            vec![
                case::create_full_case(
                    Coordinate::new(2, 0),
                    Piece::new(PieceType::Flag, Color::Red),
                ),
                case::create_full_case(
                    Coordinate::new(2, 1),
                    Piece::new(PieceType::Major, Color::Red),
                ),
                case::create_full_case(
                    Coordinate::new(2, 2),
                    Piece::new(PieceType::Spy, Color::Red),
                ),
            ],
        ]
    }
}

#[cfg(test)]
mod board_tests {

    use stratego_lib::board::case::{self, Coordinate, State};
    use stratego_lib::board::classic_board::{create_empty_stratego_board, StrategoBoard};
    use stratego_lib::board::piece::{Color, Piece, PieceType};
    use stratego_lib::board::Board;

    #[test]
    fn should_allow_scout_move() {
        let mut board = create_empty_stratego_board();
        board
            .place(case::create_full_case(
                Coordinate::new(0, 0),
                Piece::new(PieceType::Scout, Color::Blue),
            ))
            .unwrap();

        match board.moving(Coordinate::new(0, 0), Coordinate::new(0, 1)) {
            Ok(_) => assert!(true),
            Err(e) => {
                eprintln!("{}", e.message());
                assert!(false);
            }
        }

        match board.moving(Coordinate::new(0, 1), Coordinate::new(0, 9)) {
            Ok(_) => assert!(true),
            Err(e) => {
                eprintln!("{}", e.message());
                assert!(false);
            }
        }
    }

    #[test]
    fn should_build_stratego_board() {
        let stratego_board = create_empty_stratego_board();
        let state = stratego_board.state();

        assert_eq!(state.len(), 10);
        assert_eq!(state[0].len(), 10);
    }

    #[test]
    fn should_display() {
        let bomb = Piece::new(PieceType::Bomb, Color::Blue);
        let stratego_board = StrategoBoard::new(vec![vec![case::create_full_case(
            Coordinate::new(0, 0),
            bomb,
        )]]);

        assert_eq!("   |   A   |\n 0 |  B  B | \n", stratego_board.display());
    }

    #[test]
    fn should_move_piece() {
        let general = Piece::new(PieceType::General, Color::Blue);
        let mut stratego_board = StrategoBoard::new(vec![
            vec![
                case::create_full_case(Coordinate::new(0, 0), general.clone()),
                case::create_empty_case(Coordinate::new(0, 1)),
            ],
            vec![
                case::create_empty_case(Coordinate::new(1, 0)),
                case::create_empty_case(Coordinate::new(1, 1)),
            ],
        ]);

        println!("{}", stratego_board.display());

        let result = stratego_board.moving(Coordinate::new(0, 0), Coordinate::new(0, 1));

        match result {
            Ok(_) => assert!(true),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn should_not_move_piece_cause_unreachable() {
        let sergeant = Piece::new(PieceType::Sergeant, Color::Blue);
        let mut stratego_board = StrategoBoard::new(vec![
            vec![
                case::create_empty_case(Coordinate::new(0, 0)),
                case::create_empty_case(Coordinate::new(0, 1)),
            ],
            vec![
                case::create_full_case(Coordinate::new(1, 0), sergeant.clone()),
                case::create_unreachable_case(Coordinate::new(1, 1)),
            ],
        ]);

        let result = stratego_board.moving(Coordinate::new(1, 0), Coordinate::new(1, 1));

        match result {
            Ok(_) => panic!("Should not happen"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn shoud_not_move_piece_cause_scouts_does_not_cross_water() {
        let scout = Piece::new(PieceType::Scout, Color::Blue);
        let mut stratego_board = StrategoBoard::new(vec![
            vec![
                case::create_full_case(Coordinate::new(0, 0), scout.clone()),
                case::create_unreachable_case(Coordinate::new(0, 1)),
                case::create_empty_case(Coordinate::new(0, 2)),
            ],
            vec![
                case::create_empty_case(Coordinate::new(1, 0)),
                case::create_unreachable_case(Coordinate::new(1, 1)),
                case::create_empty_case(Coordinate::new(1, 2)),
            ],
            vec![
                case::create_empty_case(Coordinate::new(2, 0)),
                case::create_unreachable_case(Coordinate::new(2, 1)),
                case::create_empty_case(Coordinate::new(2, 2)),
            ],
        ]);

        eprintln!("{}", stratego_board.display());
        let result = stratego_board.moving(Coordinate::new(0, 0), Coordinate::new(0, 2));

        eprintln!("{}", stratego_board.display());
        match result {
            Ok(_) => panic!("Should not happen"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn should_move_and_capture() {
        let sergeant = Piece::new(PieceType::Sergeant, Color::Blue);
        let lieutenant = Piece::new(PieceType::Lieutenant, Color::Red);
        let mut stratego_board = create_empty_stratego_board();
        stratego_board
            .place(case::create_full_case(
                Coordinate::new(0, 0),
                lieutenant.clone(),
            ))
            .unwrap();
        stratego_board
            .place(case::create_full_case(
                Coordinate::new(0, 1),
                sergeant.clone(),
            ))
            .unwrap();

        eprintln!("{}", stratego_board.display());
        let result = stratego_board.moving(Coordinate::new(0, 0), Coordinate::new(0, 1));
        eprintln!("{}", stratego_board.display());

        match result {
            Ok(_) => {
                let previous_case = stratego_board.get_at(&Coordinate::new(0, 0));
                assert_eq!(&State::Empty, previous_case.get_state());

                let actual_case = stratego_board.get_at(&Coordinate::new(0, 1));
                assert_eq!(&State::Full, actual_case.get_state());
                let piece = actual_case.get_content();
                assert_eq!(&Color::Red, piece.get_color());
                assert_eq!(&PieceType::Lieutenant, piece.get_rank());
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn should_not_move_in_diagonales() {
        let general = Piece::new(PieceType::General, Color::Red);
        let mut stratego_board = StrategoBoard::new(vec![
            vec![
                case::create_full_case(Coordinate::new(0, 0), general.clone()),
                case::create_empty_case(Coordinate::new(0, 1)),
            ],
            vec![
                case::create_empty_case(Coordinate::new(1, 0)),
                case::create_empty_case(Coordinate::new(1, 1)),
            ],
        ]);

        let result = stratego_board.moving(Coordinate::new(0, 0), Coordinate::new(1, 1));

        match result {
            Ok(_) => panic!("Should not happen"),
            Err(_) => assert!(true),
        }
    }

}
