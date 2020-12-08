use crate::board::case::{Case, State};
use crate::board::piece::Color;
use crate::error::StrategoError;
use crate::error::StrategoError::InitGameError;

pub fn verify_board_integrity(state: Vec<Vec<Case>>) -> Result<Vec<Vec<Case>>, StrategoError> {
    if !check_board_size(&state) {
        Err(InitGameError(String::from(
            "Board is not official, GO OUT OF THERE !!",
        )))
    } else if !check_empty_middle(&state) {
        if !check_empty_lakes(&state) {
            Err(InitGameError(String::from(
                "You can not place pieces in lakes, please check again",
            )))
        } else {
            Err(InitGameError(String::from(
                "The 2 rows in the middle must be empty, :(",
            )))
        }
    }
    // blue: 0 -> 4 || red: 5 -> 9
    else if !check_player_has_piece_in_the_for_rows_of_his_color(&state[0..4], &Color::Blue)
        && !check_player_has_piece_in_the_for_rows_of_his_color(&state[5..9], &Color::Red)
    {
        Err(InitGameError(String::from(
            "Players must place theirs pieces in the right place !",
        )))
    } else {
        Ok(state)
    }
}

fn check_player_has_piece_in_the_for_rows_of_his_color(cases: &[Vec<Case>], color: &Color) -> bool {
    for row in cases {
        for c in row {
            let piece = c.get_content();
            if piece.get_color() != color {
                return false;
            }
        }
    }
    true
}

fn check_empty_middle(cases: &Vec<Vec<Case>>) -> bool {
    let forth_row: Vec<_> = cases
        .get(4)
        .unwrap()
        .iter()
        .filter(|row| !is_supposed_to_be_in_the_middle(row))
        .collect();

    let fifth_row: Vec<_> = cases
        .get(5)
        .unwrap()
        .iter()
        .filter(|row| !is_supposed_to_be_in_the_middle(row))
        .collect();

    forth_row.is_empty() && fifth_row.is_empty()
}

fn is_supposed_to_be_in_the_middle(c: &Case) -> bool {
    &State::Full != c.get_state()
}

fn check_board_size(cases: &Vec<Vec<Case>>) -> bool {
    if 10 != cases.len() {
        false
    } else {
        cases
            .iter()
            .filter(|row| !check_row_size(row))
            .collect::<Vec<_>>()
            .is_empty()
    }
}

fn check_row_size(row: &Vec<Case>) -> bool {
    10 == row.len()
}

fn check_empty_lakes(cases: &Vec<Vec<Case>>) -> bool {
    check_lake(cases.get(4).unwrap().get(2).unwrap())
        && check_lake(cases.get(4).unwrap().get(3).unwrap())
        && check_lake(cases.get(5).unwrap().get(2).unwrap())
        && check_lake(cases.get(5).unwrap().get(3).unwrap())
        && check_lake(cases.get(4).unwrap().get(6).unwrap())
        && check_lake(cases.get(4).unwrap().get(7).unwrap())
        && check_lake(cases.get(5).unwrap().get(6).unwrap())
        && check_lake(cases.get(5).unwrap().get(7).unwrap())
}

fn check_lake(c: &Case) -> bool {
    &State::Unreachable == c.get_state()
}

#[cfg(test)]
mod test {

    use crate::board::case::{
        create_empty_case, create_full_case, create_unreachable_case, Case, Coordinate,
    };
    use crate::board::piece::{Color, Piece, PieceType};

    use super::verify_board_integrity;

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

        let res = verify_board_integrity(new_board);

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
            Piece::new(PieceType::Spy, Box::new(Color::Blue)),
        );

        use crate::board::classic_board::StrategoBoard;
        use crate::board::Board;

        let b = StrategoBoard::new(new_board.clone());
        println!("{}", b.display());
        let res = verify_board_integrity(new_board);

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
                    Piece::new(PieceType::Spy, Box::new(Color::Blue)),
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

        let res = verify_board_integrity(new_board);
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
            Piece::new(PieceType::Spy, Box::new(Color::Red)),
        );
        let res = verify_board_integrity(new_board);
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
    fn should_verify_board_integrity() {
        let res = verify_board_integrity(create_statego_board());
        match res {
            Ok(_) => assert!(true),
            Err(_) => panic!("Should not happen"),
        }
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
                        Piece::new(PieceType::Spy, Box::new(Color::Blue)),
                    ));
                } else if i > 5 {
                    new_board[i].push(create_full_case(
                        Coordinate::new(i as i16, j as i16),
                        Piece::new(PieceType::Spy, Box::new(Color::Red)),
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
}
