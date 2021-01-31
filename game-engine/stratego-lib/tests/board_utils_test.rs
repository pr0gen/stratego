#[cfg(test)]
mod board_utils_test {

    use stratego_lib::board::board_utils::attack;
    use stratego_lib::board::case::{create_full_case, Coordinate, State};
    use stratego_lib::board::piece::{Color, Piece, PieceType};

    #[test]
    fn attacker_should_win() {
        let attacker = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Colonel, Color::Blue),
        );
        let defenser = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Lieutenant, Color::Red),
        );
        let res = attack(attacker, defenser).unwrap();

        assert_eq!(res.0.get_state(), &State::Empty);

        let defenser2 = res.1;
        assert_eq!(defenser2.get_state(), &State::Full);
        assert_eq!(defenser2.get_content().get_color(), &Color::Blue);
    }


    #[test]
    fn defenser_should_win() {
        let attacker = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Lieutenant, Color::Blue),
        );
        let defenser = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Colonel, Color::Red),
        );
        let res = attack(attacker, defenser).unwrap();

        assert_eq!(res.0.get_state(), &State::Empty);

        let defenser2 = res.1;
        assert_eq!(defenser2.get_state(), &State::Full);
        assert_eq!(defenser2.get_content().get_color(), &Color::Red);
    }

    #[test]
    fn both_should_loose() {
        let attacker = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Colonel, Color::Blue),
        );
        let defenser = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Colonel, Color::Red),
        );
        let res = attack(attacker, defenser).unwrap();

        assert_eq!(res.0.get_state(), &State::Empty);
        assert_eq!(res.1.get_state(), &State::Empty);
    }

    #[test]
    fn spy_should_kill_marshal() {
        let attacker = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Spy, Color::Blue),
        );
        let defenser = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Marshal, Color::Red),
        );
        let res = attack(attacker, defenser).unwrap();

        assert_eq!(res.0.get_state(), &State::Empty);

        let defenser2 = res.1;
        assert_eq!(defenser2.get_state(), &State::Full);
        assert_eq!(defenser2.get_content().get_color(), &Color::Blue);
    }

    #[test]
    fn miner_should_kill_bomb() {
        let attacker = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Miner, Color::Blue),
        );
        let defenser = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Bomb, Color::Red),
        );
        let res = attack(attacker, defenser).unwrap();

        assert_eq!(res.0.get_state(), &State::Empty);

        let defenser2 = res.1;
        assert_eq!(defenser2.get_state(), &State::Full);
        assert_eq!(defenser2.get_content().get_color(), &Color::Blue);
    }

    #[test]
    fn should_loose_against_bomb() {
        let attacker = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::General, Color::Blue),
        );
        let defenser = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Bomb, Color::Red),
        );
        let res = attack(attacker, defenser).unwrap();

        assert_eq!(res.0.get_state(), &State::Empty);

        let defenser2 = res.1;
        assert_eq!(defenser2.get_state(), &State::Full);
        assert_eq!(defenser2.get_content().get_color(), &Color::Red);
    }

    #[test]
    fn should_loose_against_marshal() {
        let attacker = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Lieutenant, Color::Blue),
        );
        let defenser = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Marshal, Color::Red),
        );
        let res = attack(attacker, defenser).unwrap();

        assert_eq!(res.0.get_state(), &State::Empty);

        let defenser2 = res.1;
        assert_eq!(defenser2.get_state(), &State::Full);
        assert_eq!(defenser2.get_content().get_color(), &Color::Red);
    }
}

#[cfg(test)]
mod board_tests {

    use stratego_lib::board::case::{create_empty_case, create_full_case, create_unreachable_case};
    use stratego_lib::board::case::{Case, Coordinate, State};
    use stratego_lib::board::classic_board::{create_empty_stratego_board, StrategoBoard};
    use stratego_lib::board::piece::{Color, Piece, PieceType};
    use stratego_lib::board::Board;

    fn create_3_x_3_stratego_board() -> Vec<Vec<Case>> {
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

    #[test]
    fn should_allow_scout_move() {
        let mut board = create_empty_stratego_board();
        board
            .place(create_full_case(
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
    fn should_get_cases_in_board_straight() {
        let board = StrategoBoard::new(create_3_x_3_stratego_board());
        let at = board.get_at(&Coordinate::new(0, 0));
        let piece = at.get_content();
        assert_eq!(&Color::Blue, piece.get_color());
        assert_eq!(&PieceType::Flag, piece.get_rank());

        let at = board.get_at(&Coordinate::new(1, 2));
        assert_eq!(&State::Empty, at.get_state());

        let at = board.get_at(&Coordinate::new(2, 1));
        let piece = at.get_content();
        assert_eq!(&Color::Red, piece.get_color());
        assert_eq!(&PieceType::Major, piece.get_rank());
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
        let stratego_board =
            StrategoBoard::new(vec![vec![create_full_case(Coordinate::new(0, 0), bomb)]]);

        assert_eq!("   |   A   |\n 0 |  B  B | \n", stratego_board.display());
    }

    #[test]
    fn should_move_piece() {
        let general = Piece::new(PieceType::General, Color::Blue);
        let mut stratego_board = StrategoBoard::new(vec![
            vec![
                create_full_case(Coordinate::new(0, 0), general.clone()),
                create_empty_case(Coordinate::new(0, 1)),
            ],
            vec![
                create_empty_case(Coordinate::new(1, 0)),
                create_empty_case(Coordinate::new(1, 1)),
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
                create_empty_case(Coordinate::new(0, 0)),
                create_empty_case(Coordinate::new(0, 1)),
            ],
            vec![
                create_full_case(Coordinate::new(1, 0), sergeant.clone()),
                create_unreachable_case(Coordinate::new(1, 1)),
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
                create_full_case(Coordinate::new(0, 0), scout.clone()),
                create_unreachable_case(Coordinate::new(0, 1)),
                create_empty_case(Coordinate::new(0, 2)),
            ],
            vec![
                create_empty_case(Coordinate::new(1, 0)),
                create_unreachable_case(Coordinate::new(1, 1)),
                create_empty_case(Coordinate::new(1, 2)),
            ],
            vec![
                create_empty_case(Coordinate::new(2, 0)),
                create_unreachable_case(Coordinate::new(2, 1)),
                create_empty_case(Coordinate::new(2, 2)),
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
        stratego_board.place(create_full_case(Coordinate::new(0, 0), lieutenant.clone())).unwrap();
        stratego_board.place(create_full_case(Coordinate::new(0, 1), sergeant.clone())).unwrap();

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
                create_full_case(Coordinate::new(0, 0), general.clone()),
                create_empty_case(Coordinate::new(0, 1)),
            ],
            vec![
                create_empty_case(Coordinate::new(1, 0)),
                create_empty_case(Coordinate::new(1, 1)),
            ],
        ]);

        let result = stratego_board.moving(Coordinate::new(0, 0), Coordinate::new(1, 1));

        match result {
            Ok(_) => panic!("Should not happen"),
            Err(_) => assert!(true),
        }
    }
}
