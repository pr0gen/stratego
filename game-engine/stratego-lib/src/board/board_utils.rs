use super::case::{create_empty_case, create_full_case, Case, Coordinate, State};
use crate::board::piece::PieceType;
use crate::board::Board;
use crate::error::StrategoError;
use crate::board::piece::deplacement::AvailableMove;
use crate::board::piece::Color;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn attack(from: Case, to: Case) -> Result<(Case, Case), StrategoError> {
    let attacker = from.get_content();
    let defenser = to.get_content();
    let rank_d = defenser.get_rank();
    let rank_a = attacker.get_rank();

    if &PieceType::Bomb == rank_d && &PieceType::Miner != rank_a {
        attacker_looses(*from.get_coordinate(), to)
    } else if &PieceType::Marshal == rank_d && &PieceType::Spy == rank_a {
        attacker_wins(from, *to.get_coordinate())
    } else if attacker.get_color() == defenser.get_color() {
        Err(StrategoError::MoveError(
            String::from("Don't attack your self"),
            from,
            to.get_coordinate().to_owned(),
        ))
    } else if attacker.get_rank() > defenser.get_rank() {
        attacker_wins(from, *to.get_coordinate())
    } else if attacker.get_rank() == defenser.get_rank() {
        equality(*from.get_coordinate(), *to.get_coordinate())
    } else {
        attacker_looses(*from.get_coordinate(), to)
    }
}

fn equality(from: Coordinate, to: Coordinate) -> Result<(Case, Case), StrategoError> {
    Ok((create_empty_case(from), create_empty_case(to)))
}

fn attacker_wins(from: Case, to: Coordinate) -> Result<(Case, Case), StrategoError> {
    Ok((
        create_empty_case(*from.get_coordinate()),
        create_full_case(to, from.get_content().clone()),
    ))
}

fn attacker_looses(from: Coordinate, to: Case) -> Result<(Case, Case), StrategoError> {
    Ok((
        create_empty_case(from),
        create_full_case(*to.get_coordinate(), to.get_content().clone()),
    ))
}

pub fn check_move(board: &impl Board, from: &Coordinate, to: &Coordinate) -> bool {
    let response: Vec<_> = get_availables_moves(board)
        .into_iter()
        .filter(|(f, t, _, _)| f == from && t == to)
        .collect();
    let size = response.len();
    size == 1
}

pub fn get_availables_moves_by_color(
    board: &impl Board,
    color: &Color,
) -> Vec<(Coordinate, Coordinate, Color, Color)> {
    get_availables_moves(board)
        .into_iter()
        .filter(|(_, _, player_color, _)| player_color == color)
        .collect()
}

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

fn find_vertical_cases(cases: &[Vec<Case>], y: i16) -> Vec<Case> {
    let mut vec = Vec::new();
    for row in cases {
        for case in row {
            let coordinate = case.get_coordinate();
            if y == coordinate.get_y() {
                vec.push(case.to_owned());
            }
        }
    }
    vec
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

fn check_for_scouts(
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
    let vertical = find_vertical_cases(&cases, y as i16);
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

fn check_case(case: &Case, player_color: &Color) -> Option<Coordinate> {
    if &State::Unreachable == case.get_state() || player_color == case.get_content().get_color() {
        None
    } else {
        Some(*case.get_coordinate())
    }
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
