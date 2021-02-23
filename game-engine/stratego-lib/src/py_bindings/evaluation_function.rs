use crate::board::case::Case;
use crate::board::piece::{Color, PieceType};
use crate::board::Board;
use crate::engine_utils;

pub type Material = ((Color, i32), (Color, i32));
/// Return winner of the game, if one there is
pub fn basic_evaluation(board: &impl Board) -> Option<Color> {
    engine_utils::game_is_over(board.state())
}

/// Evaluate material of both Players, (red, blue)
pub fn material_evaluation(
    board: &impl Board,
    material_values: &[(PieceType, i16)],
) ->  Material {
    let (reds, blues) = get_all_pieces(board.state());
    (
        (Color::Red, count_material(&reds, &material_values)),
        (Color::Blue, count_material(&blues, &material_values)),
    )
}

fn count_material(cases: &[Case], material_values: &[(PieceType, i16)]) -> i32 {
    cases
        .iter()
        .map(|case| case.get_content().get_rank())
        .map(|rank| {
            if let Some((_, v)) = material_values.iter().find(|(r, _)| r == rank) {
                *v as i32
            } else {
                0
            }
        })
        .sum()
}

pub fn get_all_pieces(cases: &[Vec<Case>]) -> (Vec<Case>, Vec<Case>) {
    let flatten_state: Vec<_> = cases.iter().flatten().collect();
    let blues: Vec<_> = flatten_state
        .iter()
        .filter(|c| c.get_content().get_color() == &Color::Blue)
        .map(|&c| c.clone())
        .collect::<Vec<Case>>();
    let reds: Vec<_> = flatten_state
        .iter()
        .filter(|c| c.get_content().get_color() == &Color::Red)
        .map(|&c| c.clone())
        .collect::<Vec<Case>>();
    (reds, blues)
}
