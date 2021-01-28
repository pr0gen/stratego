use crate::board::case::{self, Case, Coordinate, PyCoord, PyState, State};
use crate::board::classic_board::{self, StrategoBoard};
use crate::board::piece::{Color, Piece, PieceType};
use crate::board::Board;
use crate::engine_utils;
use crate::error::StrategoError;
use crate::utils;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;
use pythonize::pythonize;
use serde::{Deserialize, Serialize};
use std::env::current_dir;

pub mod evaluation_function;

#[pymodule]
fn stratego_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RustStrategoBoard>()?;
    m.add_class::<Case>()?;
    m.add_class::<Piece>()?;
    m.add_class::<Coordinate>()?;

    m.add_wrapped(wrap_pyfunction!(rust_create_full_case))?;
    m.add_wrapped(wrap_pyfunction!(rust_create_empty_case))?;
    m.add_wrapped(wrap_pyfunction!(rust_create_unreachable_case))?;
    m.add_wrapped(wrap_pyfunction!(rust_create_piece))?;
    m.add_wrapped(wrap_pyfunction!(rust_create_empty_stratego_board))?;
    m.add_wrapped(wrap_pyfunction!(rust_create_stratego_board))?;

    Ok(())
}

type PyColor = String;
type PyPieceType = i8;

#[pyclass]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RustStrategoBoard {
    board: StrategoBoard,
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

impl RustStrategoBoard {}

#[pymethods]
impl RustStrategoBoard {
    #[new]
    pub fn new(cases: Vec<Vec<Case>>) -> Self {
        RustStrategoBoard {
            board: StrategoBoard::new(cases),
        }
    }

    pub fn clone_board(&self) -> Self {
        self.clone()
    }

    pub fn state(&self) -> PyResult<Py<PyAny>> {
        let gil_holder = utils::get_gild_holder().unwrap();
        let gil = gil_holder.get();
        Ok(pythonize(gil.python(), self.board.state()).unwrap())
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

    pub fn get_available_moves(&self) -> PyResult<Py<PyAny>> {
        let moves = engine_utils::get_availables_moves(&self.board);
        let moves: Vec<(PyCoord, PyCoord, Color, Color)> = moves
            .iter()
            .map(|(from, to, origin_color, target_color)| {
                (
                    case::from(from),
                    case::from(to),
                    *origin_color,
                    *target_color,
                )
            })
            .collect();
        let gil_holder = utils::get_gild_holder().unwrap();
        let gil = gil_holder.get();
        Ok(pythonize(gil.python(), &moves).unwrap())
    }

    pub fn place(
        &mut self,
        state: PyState,
        coordinate: PyCoord,
        piece_type: PyPieceType,
        color: PyColor,
    ) -> PyResult<()> {
        self.board.place(Case::new(
            State::from(state.as_str()),
            Coordinate::from(coordinate),
            Piece::new(piece_type.into(), color.into()),
        ));
        Ok(())
    }

    pub fn get_available_moves_by_color(&self, color: PyColor) -> PyResult<Py<PyAny>> {
        let moves = engine_utils::get_availables_moves(&self.board);
        let moves: Vec<(PyCoord, PyCoord, Color, Color)> = moves
            .iter()
            .map(|(from, to, origin_color, target_color)| {
                (
                    case::from(from),
                    case::from(to),
                    *origin_color,
                    *target_color,
                )
            })
            .filter(|(_, _, player_color, _)| player_color.as_str() == color)
            .collect();
        let gil_holder = utils::get_gild_holder().unwrap();
        let gil = gil_holder.get();
        Ok(pythonize(gil.python(), &moves).unwrap())
    }

    pub fn basic_evaluation(&self) -> PyResult<PyColor> {
        if let Some(color) = evaluation_function::basic_evaluation(&self.board) {
            Ok(color.as_str().to_string())
        } else {
            Ok(String::from("None"))
        }
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
