use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;
use pythonize::pythonize;
use std::env::current_dir;

use crate::board::case::{self, Case, Coordinate, PyCoord};
use crate::board::classic_board::{self, StrategoBoard};
use crate::board::piece::{Color, Piece, PieceType};
use crate::board::Board;
use crate::engine_utils;
use crate::error::StrategoError;
use crate::utils;

pub mod evaluation_function;

#[pymodule]
fn stratego_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RustStrategoBoard>()?;
    m.add_class::<Case>()?;
    m.add_class::<Piece>()?;
    m.add_class::<Coordinate>()?;

    m.add_wrapped(wrap_pyfunction!(rust_get_available_moves))?;
    m.add_wrapped(wrap_pyfunction!(rust_create_full_case))?;
    m.add_wrapped(wrap_pyfunction!(rust_create_empty_case))?;
    m.add_wrapped(wrap_pyfunction!(rust_create_unreachable_case))?;
    m.add_wrapped(wrap_pyfunction!(rust_create_piece))?;
    m.add_wrapped(wrap_pyfunction!(rust_create_empty_stratego_board))?;
    m.add_wrapped(wrap_pyfunction!(rust_create_stratego_board))?;

    m.add_wrapped(wrap_pyfunction!(rust_basic_evaluation))?;

    Ok(())
}

type PyColor = String;
type PyPieceType = i8;

#[pyclass]
#[derive(Clone)]
struct RustStrategoBoard {
    board: StrategoBoard,
}

#[pyfunction]
fn rust_basic_evaluation(board: RustStrategoBoard) -> PyResult<PyColor> {
    if let Some(color) = engine_utils::game_is_over(board.board.state()) {
        Ok(color.as_str().to_string())
    } else {
        Ok(String::from("None"))
    }
}

#[pyfunction]
fn rust_create_empty_stratego_board() -> PyResult<RustStrategoBoard> {
    let board = classic_board::create_empty_stratego_board();
    Ok(RustStrategoBoard::new(board.state().to_owned()))
}

#[pyfunction]
fn rust_create_stratego_board() -> PyResult<RustStrategoBoard> {
    let board = classic_board::create_stratego_board();
    Ok(RustStrategoBoard::new(board.state().to_owned()))
}

#[pyfunction]
fn rust_get_available_moves(board: RustStrategoBoard) -> PyResult<Py<PyAny>> {
    let moves = engine_utils::get_availables_moves(&board.board);
    let moves: Vec<(PyCoord, PyCoord, Color)> = moves
        .iter()
        .map(|(from, to, color)| (case::from(from), case::from(to), *color))
        .collect();
    let gil_holder = utils::get_gild_holder().unwrap();
    let gil = gil_holder.get();
    //Ok(moves)
    Ok(pythonize(gil.python(), &moves).unwrap())
}

#[pyfunction]
fn rust_create_full_case(coordinate: PyCoord, content: Piece) -> PyResult<Case> {
    Ok(case::create_full_case(
        Coordinate::from(coordinate),
        content,
    ))
}

#[pyfunction]
fn rust_create_empty_case(coordinate: PyCoord) -> PyResult<Case> {
    Ok(case::create_empty_case(Coordinate::from(coordinate)))
}

#[pyfunction]
fn rust_create_unreachable_case(coordinate: PyCoord) -> PyResult<Case> {
    Ok(case::create_unreachable_case(Coordinate::from(coordinate)))
}

#[pyfunction]
fn rust_create_piece(piece_type: PyPieceType, color: PyColor) -> PyResult<Piece> {
    Ok(Piece::new(piece_type.into(), color.into()))
}

impl Into<Color> for PyColor {
    fn into(self) -> Color {
        match self.as_str() {
            "Blue" => Color::Blue,
            "Red" => Color::Red,
            _ => Color::None,
        }
    }
}

impl Into<PieceType> for PyPieceType {
    fn into(self) -> PieceType {
        match self {
            -1 => PieceType::Flag,
            -2 => PieceType::Bomb,
            10 => PieceType::Marshal,
            9 => PieceType::General,
            8 => PieceType::Colonel,
            7 => PieceType::Major,
            6 => PieceType::Captain,
            5 => PieceType::Lieutenant,
            4 => PieceType::Sergeant,
            3 => PieceType::Miner,
            2 => PieceType::Scout,
            1 => PieceType::Spy,
            _ => PieceType::Null,
        }
    }
}

#[pymethods]
impl RustStrategoBoard {
    #[new]
    pub fn new(cases: Vec<Vec<Case>>) -> Self {
        RustStrategoBoard {
            board: StrategoBoard::new(cases),
        }
    }

    pub fn moving(&mut self, case: Case, to: PyCoord) -> PyResult<Vec<String>> {
        let cases = self.board.moving(case, Coordinate::from(to)).unwrap();
        Ok(cases.iter().map(|case| case.display()).collect())
    }

    pub fn display_by_color(&self, color: PyColor) -> PyResult<String> {
        Ok(self.board.display_by_color(&color.into()))
    }

    pub fn at(&self, coordinate: PyCoord) -> PyResult<Case> {
        Ok(self.board.get_at(&Coordinate::from(coordinate)).clone())
    }

    //TODO
    pub fn evaluate_simple(&self) -> PyResult<PyColor> {
        todo!()
    }
}

pub fn load_stratego_ai_module(py: &Python) -> Result<(), StrategoError> {
    let syspath: &PyList = py
        .import("sys")
        .unwrap_or_else(|e| {
            panic!(StrategoError::AILoadingError(format!(
                "Failed to find sys python module, {}",
                e
            )))
        })
        .get("path")
        .unwrap_or_else(|e| {
            panic!(StrategoError::AILoadingError(format!(
                "Failed to find path function in sys python module, {}",
                e
            )))
        })
        .try_into()
        .unwrap_or_else(|e| {
            panic!(StrategoError::AILoadingError(format!(
                "Failed to get result from path function in sys python module, {}",
                e
            )))
        });

    let cur = current_dir().unwrap_or_else(|e| {
        panic!(StrategoError::AILoadingError(format!(
            "Failed to find pwd, {}",
            e
        )))
    });

    let pwd = cur.as_path().as_os_str().to_str().unwrap();
    match syspath.insert(0, pwd.to_string()) {
        Ok(_) => Ok(()),
        Err(e) => panic!(StrategoError::AILoadingError(format!(
            "Failed to load ai for stratego, {}",
            e
        ))),
    }
}
