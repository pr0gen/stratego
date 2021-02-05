use std::collections::HashSet;
use crate::board::case::{Case, Coordinate, State};
use crate::board::classic_board::StrategoBoard;
use crate::board::piece::deplacement::AvailableMove;
use crate::board::piece::piece_utils::list_all_pieces;
use crate::board::piece::{Color, PieceType};
use crate::board::Board;
use crate::error::StrategoError::{self, InitGameError};
use crate::player::Player;


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
        eprintln!("Blue has no flag");
        return Some(Color::Red);
    }

    let res: Vec<_> = reds
        .iter()
        .filter(|c| c.get_content().get_rank() == &PieceType::Flag)
        .collect();
    if res.is_empty() {
        eprintln!("Red has no flag");
        return Some(Color::Blue);
    }

    //Check moveable pieces
    let res: Vec<_> = blues
        .iter()
        .filter(|c| c.get_content().get_rank() != &PieceType::Flag)
        .filter(|c| !c.get_content().get_move().equals(AvailableMove::Immovable))
        .collect();
    if res.is_empty() {
        eprintln!("Blue can not move");
        return Some(Color::Red);
    }

    let res: Vec<_> = reds
        .iter()
        .filter(|c| c.get_content().get_rank() != &PieceType::Flag)
        .filter(|c| !c.get_content().get_move().equals(AvailableMove::Immovable))
        .collect();
    if res.is_empty() {
        eprintln!("Red can not move");
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
