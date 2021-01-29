use std::collections::HashSet;

use crate::board::board_utils::{self, Direction};
use crate::board::case::{Case, Coordinate, State};
use crate::board::classic_board::StrategoBoard;
use crate::board::piece::deplacement::AvailableMove;
use crate::board::piece::piece_utils::list_all_pieces;
use crate::board::piece::{Color, PieceType};
use crate::board::Board;
use crate::error::StrategoError::{self, InitGameError};
use crate::player::Player;

pub fn get_availables_moves(board: &impl Board) -> Vec<(Coordinate, Coordinate, Color, Color)> {
    let cases = board.state();
    let row_len = cases.len() - 1;
    let mut moves: Vec<(Coordinate, Coordinate, Color, Color)> = Vec::with_capacity(row_len);

    // x -> row
    // y -> col
    for x in 0..=row_len {
        let col_len = cases[x].len() - 1;
        for y in 0..=col_len {
            let co = Coordinate::new(x as i16, y as i16);
            let case = board.get_at(&co);
            let content = case.get_content();
            if &PieceType::Scout == content.get_rank() {
                moves.append(&mut check_for_scouts(case, cases, &co, row_len));
            } else if y == 0 || y == col_len || x == 0 || x == row_len {
                moves.append(&mut check_for_side(
                    board, case, &co, row_len, col_len, cases,
                ));
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

fn check_for_side(
    board: &impl Board,
    case: &Case,
    coordinate: &Coordinate,
    row_len: usize,
    col_len: usize,
    cases: &[Vec<Case>],
) -> Vec<(Coordinate, Coordinate, Color, Color)> {
    let x = coordinate.get_x() as usize;
    let y = coordinate.get_y() as usize;
    let mut moves: Vec<(Coordinate, Coordinate, Color, Color)> = Vec::with_capacity(row_len);
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
    } else if y == col_len && x == row_len {
        // It's a corner (lower right)
        moves.append(&mut check_cases(
            &[
                board.get_at(&Coordinate::new((x - 1) as i16, y as i16)),
                board.get_at(&Coordinate::new(x as i16, (y - 1) as i16)),
            ],
            &case,
        ));
    } else if x == 0 && y == row_len {
        // It's a corner (lower left)
        moves.append(&mut check_cases(
            &[
                board.get_at(&Coordinate::new(x as i16, (y - 1) as i16)),
                board.get_at(&Coordinate::new((x + 1) as i16, y as i16)),
            ],
            &case,
        ));
    } else if x == row_len && y == 0 {
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
    moves
}

pub fn check_for_scouts(
    case: &Case,
    cases: &[Vec<Case>],
    coordinate: &Coordinate,
    row_len: usize,
) -> Vec<(Coordinate, Coordinate, Color, Color)> {
    let x = coordinate.get_x() as usize;
    let y = coordinate.get_y() as usize;
    let mut moves: Vec<(Coordinate, Coordinate, Color, Color)> = Vec::with_capacity(row_len);
    let content = case.get_content();
    let color = content.get_color();
    //horizontal
    moves.append(&mut check_part_of_row(
        &cases[x],
        case,
        color,
        Direction::Left,
    ));
    moves.append(&mut check_part_of_row(
        &cases[x],
        case,
        color,
        Direction::Right,
    ));

    //vertical
    let vertical = board_utils::find_vertical_cases(&cases, y as i16);
    moves.append(&mut check_part_of_row(
        &vertical,
        case,
        color,
        Direction::Up,
    ));
    moves.append(&mut check_part_of_row(
        &vertical,
        case,
        color,
        Direction::Down,
    ));
    moves
}

fn check_part_of_row(
    cases: &[Case],
    case: &Case,
    player_color: &Color,
    direction: Direction,
) -> Vec<(Coordinate, Coordinate, Color, Color)> {
    let coordinate = case.get_coordinate();
    match direction {
        Direction::Up => {
            let mut up = cases[0..coordinate.get_x() as usize].to_vec();
            up.reverse();
            check_row_for_scout(&up, case, player_color)
        }
        Direction::Left => {
            let mut left = cases[0..coordinate.get_y() as usize].to_vec();
            left.reverse();
            check_row_for_scout(&left, case, player_color)
        }
        Direction::Right => check_row_for_scout(
            &cases[(coordinate.get_y() + 1) as usize..cases.len()],
            case,
            player_color,
        ),
        Direction::Down => check_row_for_scout(
            &cases[(coordinate.get_x() + 1) as usize..cases.len()],
            case,
            player_color,
        ),
    }
}

fn check_row_for_scout(
    cases: &[Case],
    case: &Case,
    player_color: &Color,
) -> Vec<(Coordinate, Coordinate, Color, Color)> {
    let mut moves = Vec::new();
    let coord_from = case.get_coordinate();
    for to_go_case in cases {
        let coord_to = to_go_case.get_coordinate();
        let state = to_go_case.get_state();
        let color = to_go_case.get_content().get_color();
        match state {
            State::Unreachable => break,
            State::Full => {
                if player_color != color {
                    moves.push((*coord_from, *coord_to, *player_color, *color));
                }
                break;
            }
            State::Empty => moves.push((*coord_from, *coord_to, *player_color, *color)),
        }
    }
    moves
}

fn check_cases(cases: &[&Case], case: &Case) -> Vec<(Coordinate, Coordinate, Color, Color)> {
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
                moves.push((
                    *coord_from,
                    coord_to,
                    *color,
                    *case.get_content().get_color(),
                ));
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

fn check_side(cases: &[Vec<Case>], case: &Case) -> Vec<(Coordinate, Coordinate, Color, Color)> {
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

pub fn ask_next_move(player: &dyn Player, board: &StrategoBoard) -> (Coordinate, Coordinate) {
    player.ask_next_move(board.to_owned())
}

pub fn game_is_over(cases: &[Vec<Case>]) -> Option<Color> {
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
        return Some(Color::Red);
    }

    let res: Vec<_> = reds
        .iter()
        .filter(|c| c.get_content().get_rank() == &PieceType::Flag)
        .collect();
    if res.is_empty() {
        return Some(Color::Blue);
    }

    //Check moveable pieces
    let res: Vec<_> = blues
        .iter()
        .filter(|c| c.get_content().get_rank() != &PieceType::Flag)
        .filter(|c| !c.get_content().get_move().equals(AvailableMove::Immovable))
        .collect();
    if res.is_empty() {
        return Some(Color::Red);
    }

    let res: Vec<_> = reds
        .iter()
        .filter(|c| c.get_content().get_rank() != &PieceType::Flag)
        .filter(|c| !c.get_content().get_move().equals(AvailableMove::Immovable))
        .collect();
    if res.is_empty() {
        return Some(Color::Blue);
    }

    None
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
