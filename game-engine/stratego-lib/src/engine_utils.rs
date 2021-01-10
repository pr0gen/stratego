use std::collections::HashSet;

use crate::board::case::{Case, Coordinate, State};
use crate::board::classic_board::StrategoBoard;
use crate::board::piece::deplacement::AvailableMove;
use crate::board::piece::piece_utils::list_all_pieces;
use crate::board::piece::{Color, PieceType};
use crate::board::Board;
use crate::error::StrategoError::{self, InitGameError};
use crate::player::Player;

pub fn get_availables_moves(board: &impl Board) -> Vec<(Coordinate, Coordinate, Color)> {
    let cases = board.state();
    let col_len = cases.len() - 1;
    let mut moves: Vec<(Coordinate, Coordinate, Color)> = Vec::with_capacity(col_len);

    // x -> col
    // y -> row
    for x in 0..=col_len {
        let row_len = cases[x].len() - 1;
        for y in 0..=row_len {
            let case = board.get_at(&Coordinate::new(x as i16, y as i16));
            if y == 0 || y == row_len || x == 0 || x == col_len {
                // It's a side
                if y == 0 && x == 0 {
                    // It's a corner (upper left)
                    moves.append(&mut check_cases(
                        &[
                            board.get_at(&Coordinate::new((x + 1) as i16, y as i16)),
                            board.get_at(&Coordinate::new(x as i16, (y + 1) as i16)),
                        ],
                        &case,
                    ));
                } else if y == row_len && x == col_len {
                    // It's a corner (lower right)
                    moves.append(&mut check_cases(
                        &[
                            board.get_at(&Coordinate::new((x - 1) as i16, y as i16)),
                            board.get_at(&Coordinate::new(x as i16, (y - 1) as i16)),
                        ],
                        &case,
                    ));
                } else if x == 0 && y == col_len {
                    // It's a corner (lower left)
                    moves.append(&mut check_cases(
                        &[
                            board.get_at(&Coordinate::new(x as i16, (y - 1) as i16)),
                            board.get_at(&Coordinate::new((x + 1) as i16, y as i16)),
                        ],
                        &case,
                    ));
                } else if x == col_len && y == 0 {
                    // It's a corner (upper right)
                    moves.append(&mut check_cases(
                        &[
                            board.get_at(&Coordinate::new((x - 1) as i16, y as i16)),
                            board.get_at(&Coordinate::new(x as i16, (y + 1) as i16)),
                        ],
                        &case,
                    ));
                } else {
                    //It's just a normal side
                    moves.append(&mut check_side(&cases, &case));
                }
            } else {
                // It's a random case in the middle
                moves.append(&mut check_cases(
                    &[
                        board.get_at(&Coordinate::new((x - 1) as i16, y as i16)),
                        board.get_at(&Coordinate::new((x + 1) as i16, y as i16)),
                        board.get_at(&Coordinate::new(x as i16, (y + 1) as i16)),
                        board.get_at(&Coordinate::new(x as i16, (y - 1) as i16)),
                    ],
                    &case,
                ));
            }
        }
    }

    moves
}

fn check_cases(cases: &[&Case], case: &Case) -> Vec<(Coordinate, Coordinate, Color)> {
    if case
        .get_content()
        .get_move()
        .equals(AvailableMove::Immovable)
    {
        Vec::new()
    } else {
        let mut moves = Vec::new();
        let coord_from = case.get_coordinate();
        let color = case.get_content().get_color();
        cases.iter().for_each(|case| {
            if let Some(coord_to) = check_case(case, &color) {
                moves.push((*coord_from, coord_to, *color));
            }
        });
        moves
    }
}

fn check_case(case: &Case, player_color: &Color) -> Option<Coordinate> {
    if &State::Unreachable == case.get_state() || player_color == case.get_content().get_color() {
        None
    } else {
        Some(*case.get_coordinate())
    }
}

fn check_side(cases: &[Vec<Case>], case: &Case) -> Vec<(Coordinate, Coordinate, Color)> {
    let col_len = cases.len() - 1;
    let row_len = cases.get(0).unwrap().len() - 1;
    let coord_case = case.get_coordinate();
    let x = coord_case.get_x() as usize;
    let y = coord_case.get_y() as usize;

    if x == 0 {
        // left
        check_cases(
            &[
                cases.get(x).unwrap().get(y - 1).unwrap(),
                cases.get(x).unwrap().get(y + 1).unwrap(),
                cases.get(x + 1).unwrap().get(y).unwrap(),
            ],
            case,
        )
    } else if x == col_len {
        // right
        check_cases(
            &[
                cases.get(x).unwrap().get(y - 1).unwrap(),
                cases.get(x).unwrap().get(y + 1).unwrap(),
                cases.get(x - 1).unwrap().get(y).unwrap(),
            ],
            case,
        )
    } else if y == 0 {
        // upper
        check_cases(
            &[
                cases.get(x - 1).unwrap().get(y).unwrap(),
                cases.get(x + 1).unwrap().get(y).unwrap(),
                cases.get(x).unwrap().get(y + 1).unwrap(),
            ],
            case,
        )
    } else if y == row_len {
        // lower
        check_cases(
            &[
                cases.get(x - 1).unwrap().get(y).unwrap(),
                cases.get(x + 1).unwrap().get(y).unwrap(),
                cases.get(x).unwrap().get(y - 1).unwrap(),
            ],
            case,
        )
    } else {
        panic!("Should not happen, impossible to process case"); // TODO
    }
}

pub fn ask_next_move(player: &dyn Player, board: &impl Board) -> (Case, Coordinate) {
    let (from, to) = player.ask_next_move();
    let case = board.get_at(&Coordinate::new(from.get_x(), from.get_y()));

    (case.clone(), to)
}

pub fn game_is_over(cases: &[Vec<Case>]) -> Result<Color, StrategoError> {
    let flatten_state: Vec<_> = cases.iter().flatten().collect();

    let blues: Vec<_> = flatten_state
        .iter()
        .filter(|&c| c.get_content().get_color() == &Color::Blue)
        .collect();
    let reds: Vec<_> = flatten_state
        .iter()
        .filter(|&c| c.get_content().get_color() == &Color::Red)
        .collect();

    //Check has Flag
    let res: Vec<_> = blues
        .iter()
        .filter(|&c| c.get_content().get_rank() == &PieceType::Flag)
        .collect();
    if res.is_empty() {
        return Ok(Color::Red);
    }

    let res: Vec<_> = reds
        .iter()
        .filter(|c| c.get_content().get_rank() == &PieceType::Flag)
        .collect();
    if res.is_empty() {
        return Ok(Color::Blue);
    }

    //Check moveable pieces
    let res: Vec<_> = blues
        .iter()
        .filter(|c| c.get_content().get_rank() != &PieceType::Flag)
        .filter(|c| !c.get_content().get_move().equals(AvailableMove::Immovable))
        .collect();
    if res.is_empty() {
        return Ok(Color::Red);
    }

    let res: Vec<_> = reds
        .iter()
        .filter(|c| c.get_content().get_rank() != &PieceType::Flag)
        .filter(|c| !c.get_content().get_move().equals(AvailableMove::Immovable))
        .collect();
    if res.is_empty() {
        return Ok(Color::Blue);
    }

    Err(StrategoError::GameNotOverError())
}

pub fn verify_board_integrity(board: impl Board) -> Result<StrategoBoard, StrategoError> {
    let state = board.state();
    if !check_board_size(&state) {
        Err(InitGameError(String::from(
            "Board is not official, GO OUT OF THERE !!",
        )))
    } else if !check_empty_middle(&state) {
        if !check_empty_lakes(&board) {
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
    } else if !check_player_has_correct_pieces(&state[0..=3])
        && !check_player_has_correct_pieces(&state[6..=9])
    {
        Err(InitGameError(String::from(
            "You need to start with the right pieces",
        )))
    } else {
        Ok(StrategoBoard::new(board.state().to_owned()))
    }
}

fn check_player_has_correct_pieces(cases: &[Vec<Case>]) -> bool {
    let piece_types: Vec<PieceType> = cases
        .iter()
        .flatten()
        .map(|case| case.get_content().get_rank().clone())
        .collect();

    let pieces: HashSet<(PieceType, i8)> = piece_types
        .iter()
        .map(|x| {
            (
                x.clone(),
                piece_types.iter().filter(|&y| y == x).count() as i8,
            )
        })
        .collect();

    let all_pieces = list_all_pieces();
    for (key, value) in pieces.iter() {
        if all_pieces.get(key) != Some(value) {
            return false;
        }
    }

    true
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

fn check_empty_middle(cases: &[Vec<Case>]) -> bool {
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

fn check_board_size(cases: &[Vec<Case>]) -> bool {
    if 10 != cases.len() {
        false
    } else {
        cases.iter().find(|row| !check_row_size(row)).is_none()
    }
}

fn check_row_size(row: &[Case]) -> bool {
    10 == row.len()
}

fn check_empty_lakes(board: &impl Board) -> bool {
    check_lake(board.get_at(&Coordinate::new(4, 2)))
        && check_lake(board.get_at(&Coordinate::new(4, 3)))
        && check_lake(board.get_at(&Coordinate::new(5, 2)))
        && check_lake(board.get_at(&Coordinate::new(5, 3)))
        && check_lake(board.get_at(&Coordinate::new(4, 6)))
        && check_lake(board.get_at(&Coordinate::new(4, 7)))
        && check_lake(board.get_at(&Coordinate::new(5, 6)))
        && check_lake(board.get_at(&Coordinate::new(5, 7)))
}

fn check_lake(c: &Case) -> bool {
    &State::Unreachable == c.get_state()
}

#[cfg(test)]
mod test {

    use crate::board::case::{
        create_empty_case, create_full_case, create_unreachable_case, Case, Coordinate,
    };
    use crate::board::classic_board::StrategoBoard;
    use crate::board::piece::{Color, Piece, PieceType};

    use super::{game_is_over, get_availables_moves, verify_board_integrity};

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
            Ok(val) => {
                assert_eq!(Color::Red, val);
            }
            Err(_) => panic!("Should not happen"),
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
            Ok(val) => {
                assert_eq!(Color::Red, val);
            }
            Err(_) => panic!("Should not happen"),
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
            Ok(_) => panic!("Should not happen"),
            Err(e) => {
                assert!(true);

                assert_eq!(e.message(), String::from("Game is not over"));
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

        use crate::board::classic_board::StrategoBoard;
        use crate::board::Board;

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
        board[5][7] = create_unreachable_case(Coordinate::new(7, 5));

        return board;
    }
}
