use crate::board::case::{self, Case, Coordinate, State};
use crate::board::classic_board::StrategoBoard;
use crate::board::piece::deplacement::AvailableMove;
use crate::board::piece::piece_utils;
use crate::board::piece::Piece;
use crate::board::piece::{Color, PieceType};
use crate::board::Board;
use crate::error::StrategoError::{self, InitGameError};
use rand;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fs;
use crate::py_bindings::board_wrapper;
use rand::prelude::*;

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


pub fn create_stratego_board_from_file(file_name: &str) -> Result<StrategoBoard, StrategoError> {
    let content = read_file(file_name)?;
    // TODO
    let content: Vec<&str> = content.split('\n').collect();
    let length = content.len();
    let board_row = rand::thread_rng().gen_range(0, length);
    let board = content[0]; 
    let board: Vec<&str> = board.split('|').collect();
    let mut cases: Vec<Vec<String>> = Vec::new();
    let mut index = 0;
    let mut parsed_row: Vec<String> = Vec::new(); 
    for row in board {
        if index == 9 {
            cases.push(parsed_row);
            parsed_row =  Vec::new();
            index = 0;
        } else {
            parsed_row.push(String::from(row));
            index += 1;
        }
        
    }

    Ok(StrategoBoard::new(board_wrapper::parse_python_cases(cases)))
}

fn read_file(file_name: &str) -> Result<String, StrategoError> {
    match fs::read_to_string(file_name) {
        Ok(content) => Ok(content),
        Err(e) => Err(StrategoError::ParsingError(format!(
            "Failed to read file {} to parse StrategoBoard. \n {}",
            file_name,
            e
        ))),
    }
}


pub fn create_stratego_board_with_same_pieces() -> StrategoBoard {
    let board = create_empty_stratego_board();
    let mut cases = board.state().clone();

    let mut pieces = piece_utils::list_pieces();
    pieces.shuffle(&mut rand::thread_rng());
    let mut others_pieces = Vec::with_capacity(pieces.len());

    let max = 4;
    for (i, row) in cases.iter_mut().enumerate().take(max) {
        for (j, case) in row.iter_mut().enumerate() {
            let piece = pieces.pop().unwrap();
            *case = case::create_full_case(
                Coordinate::new(i as i16, j as i16),
                Piece::new(piece.clone(), Color::Blue),
            );
            others_pieces.push(piece);
        }
    }

    let max = 10;
    for (i, row) in cases.iter_mut().enumerate().take(max).skip(6) {
        for (j, case) in row.iter_mut().enumerate() {
            let piece = others_pieces.pop();
            *case = case::create_full_case(
                Coordinate::new(i as i16, j as i16),
                Piece::new(piece.unwrap(), Color::Red),
            );
        }
    }

    verify_board_integrity(StrategoBoard::new(cases))
        .unwrap_or_else(|e| panic!("failed to check engine integrity: {:?}", e))
}

pub fn create_stratego_board() -> StrategoBoard {
    let board = create_empty_stratego_board();

    let mut cases = board.state().clone();

    let mut pieces = piece_utils::list_of_all_pieces(Color::Blue);
    pieces.shuffle(&mut rand::thread_rng());

    let max = 4;
    for (i, row) in cases.iter_mut().enumerate().take(max) {
        for (j, case) in row.iter_mut().enumerate() {
            let piece = pieces.pop();
            *case = case::create_full_case(Coordinate::new(i as i16, j as i16), piece.unwrap());
        }
    }

    let mut pieces = piece_utils::list_of_all_pieces(Color::Red);
    pieces.shuffle(&mut rand::thread_rng());
    let max = 10;
    for (i, row) in cases.iter_mut().enumerate().take(max).skip(6) {
        for (j, case) in row.iter_mut().enumerate() {
            let piece = pieces.pop();
            *case = case::create_full_case(Coordinate::new(i as i16, j as i16), piece.unwrap());
        }
    }

    verify_board_integrity(StrategoBoard::new(cases))
        .unwrap_or_else(|e| panic!("failed to check engine integrity: {:?}", e))
}

pub fn create_empty_stratego_board() -> StrategoBoard {
    let size = 10;
    let mut board: Vec<Vec<Case>> = Vec::with_capacity(size);
    for i in 0..size {
        board.push(Vec::with_capacity(size));
        for j in 0..size {
            board[i].push(case::create_empty_case(Coordinate::new(i as i16, j as i16)));
        }
    }
    let mut board = StrategoBoard::new(board);

    board
        .place(case::create_unreachable_case(Coordinate::new(4, 2)))
        .unwrap();
    board
        .place(case::create_unreachable_case(Coordinate::new(4, 3)))
        .unwrap();
    board
        .place(case::create_unreachable_case(Coordinate::new(5, 2)))
        .unwrap();
    board
        .place(case::create_unreachable_case(Coordinate::new(5, 3)))
        .unwrap();
    board
        .place(case::create_unreachable_case(Coordinate::new(4, 6)))
        .unwrap();
    board
        .place(case::create_unreachable_case(Coordinate::new(4, 7)))
        .unwrap();
    board
        .place(case::create_unreachable_case(Coordinate::new(5, 6)))
        .unwrap();
    board
        .place(case::create_unreachable_case(Coordinate::new(5, 7)))
        .unwrap();

    board
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

    let all_pieces = piece_utils::list_all_pieces();
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
        cases.iter().find(|row| !10 == row.len()).is_none()
    }
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
