use crate::board::board_utils;
use crate::board::case::Coordinate;
use crate::board::classic_board::StrategoBoard;
use crate::board::piece::Color;
use crate::board::Board;
use crate::error::StrategoError;
use crate::py_bindings::board_wrapper::StrategoBoardWrapper;
use rand::prelude::*;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::any::Any;
use crate::simulation::evaluation::EvaluationFunction;

pub mod evaluation;

pub type Move = (Coordinate, Coordinate);

pub fn simulate_multi_thread(
    pyboard: StrategoBoardWrapper,
    first_ai: &'static (dyn Fn(&StrategoBoard, &Color) -> Option<Move> + Sync),
    second_ai: &'static (dyn Fn(&StrategoBoard, &Color) -> Option<Move> + Sync),
    evaluation_function: EvaluationFunction,
    iteration_max: i32,
) -> Result<Option<Move>, StrategoError> {
    let board = pyboard.get_board().clone();
    let moves: Vec<_> = board_utils::get_availables_moves_by_color(&board, &Color::Red);
    let mut threads = Vec::with_capacity(moves.len());
    let locked_board = Arc::new(Mutex::new(pyboard));
    let locked_eval = Arc::new(Mutex::new(evaluation_function));

    moves.into_iter().for_each(|(from, to, _, _)| {
        let locked_board = Arc::clone(&locked_board);
        let locked_eval = Arc::clone(&locked_eval);
        threads.push(thread::spawn(move || {
            let pyboard = locked_board.lock().unwrap();
            let evaluation_function = locked_eval.lock().unwrap();
            simulate(
                &pyboard,
                first_ai,
                second_ai,
                &*evaluation_function,
                &iteration_max,
                (from.clone(), to.clone()),
            )
        }));
    });

    let results: Vec<_> = threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .collect();

    todo!()
    //results.iter()
    //.filter(||)
}

pub fn simulate(
    board: &StrategoBoardWrapper,
    first_ai: &'static (dyn Fn(&StrategoBoard, &Color) -> Option<Move> + Sync),
    second_ai: &'static (dyn Fn(&StrategoBoard, &Color) -> Option<Move> + Sync),
    evaluation_function: &EvaluationFunction,
    iteration_max: &i32,
    first_move: Move,
) -> Result<(bool, Box<dyn Any + Send>), StrategoError> {
    let mut iteration_max = *iteration_max;
    let mut board = board.get_board().clone();
    eprintln!("{:?}", first_move);
    board.moving(first_move.0, first_move.1)?;

    while iteration_max > 0 {
        if let Some((from, to)) = first_ai(&board, &Color::Red) {
            board.moving(from, to)?;
        }
        if let Some((from, to)) = second_ai(&board, &Color::Blue) {
            board.moving(from, to)?;
        }

        if let (true, eval) = evaluation_function.evaluate(&board) {
            return Ok((true, eval));
        }
        iteration_max -= 1;
    }
    Err(StrategoError::AIExecuteError(String::from(
        "Failed to win with this move",
    )))
}

pub fn choose_randomly(board: &impl Board, color: &Color) -> Option<Move> {
    let moves: Vec<_> = board_utils::get_availables_moves_by_color(board, &color);
    if moves.is_empty() {
        None
    } else {
        let mut rng = rand::thread_rng();
        let index: usize = rng.gen_range(0, moves.len());
        if let Some((from, to, _, _)) = moves.get(index) {
            Some((*from, *to))
        } else {
            None
        }
    }
}
