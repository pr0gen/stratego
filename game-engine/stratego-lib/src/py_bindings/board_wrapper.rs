use crate::board::case::{self, Case, Coordinate, PyCoord, PyState, State};
use crate::board::classic_board::StrategoBoard;
use crate::board::piece::{Color, Piece, PyColor, PyPieceType};
use crate::board::Board;
use crate::engine_utils;
use crate::py_bindings::evaluation_function;
use crate::utils;
use pyo3::exceptions;
use pyo3::prelude::*;
use pythonize::pythonize;
use serde::{Deserialize, Serialize};

pub fn parse_python_cases(py_cases: Vec<Vec<PyCase>>) -> Vec<Vec<Case>> {
    py_cases.iter().map(|row| parse_python_row(row)).collect()
}

fn parse_python_row(py_cases: &Vec<PyCase>) -> Vec<Case> {
    py_cases
        .iter()
        .map(|(state, piece_type, coord, color): &PyCase | {
            let piece_type: PyPieceType = *piece_type;
            Case::new(
                State::from(state.as_str()),
                Coordinate::from(coord.clone()),
                Piece::new(piece_type.into(), Color::from(color.as_str())),
            )
        })
        .collect()
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StrategoBoardWrapper {
    board: StrategoBoard,
}

pub type PyCase = (String, PyPieceType, PyCoord, PyColor);

impl StrategoBoardWrapper {
    pub fn new(board: StrategoBoard) -> Self {
        StrategoBoardWrapper { board }
    }
}

#[pymethods]
impl StrategoBoardWrapper {
    #[new]
    pub fn from(cases: Vec<Vec<PyCase>>) -> Self {
        StrategoBoardWrapper {
            board: StrategoBoard::new(parse_python_cases(cases)),
        }
    }

    pub fn clone_board(&self) -> Self {
        self.clone()
    }

    pub fn state(&self) -> PyResult<Py<PyAny>> {
        let gil_holder = utils::get_gild_holder()
            .unwrap_or_else(|e| panic!("Failed to get python gil holder, {}", e.message()));
        let gil = gil_holder.get();
        Ok(pythonize(gil.python(), self.board.state())?)
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
        let gil_holder = utils::get_gild_holder()
            .unwrap_or_else(|e| panic!("Failed to get python gil holder, {}", e.message()));
        let gil = gil_holder.get();
        Ok(pythonize(gil.python(), &moves)?)
    }

    pub fn place(
        &mut self,
        state: PyState,
        coordinate: PyCoord,
        piece_type: PyPieceType,
        color: PyColor,
    ) -> PyResult<Py<PyAny>> {
        if let Ok(case) = self.board.place(Case::new(
            State::from(state.as_str()),
            Coordinate::from(coordinate),
            Piece::new(piece_type.into(), color.into()),
        )) {
            let gil_holder = utils::get_gild_holder()
                .unwrap_or_else(|e| panic!("Failed to get python gil holder, {}", e.message()));
            let gil = gil_holder.get();
            let pythonized_case = pythonize(gil.python(), &case)?;
            Ok(pythonized_case)
        } else {
            Err(exceptions::PyTypeError::new_err("Failed to place case "))
        }
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
        let gil_holder = utils::get_gild_holder()
            .unwrap_or_else(|e| panic!("Failed to get python gil holder, {}", e.message()));
        let gil = gil_holder.get();
        Ok(pythonize(gil.python(), &moves)?)
    }

    pub fn basic_evaluation(&self) -> PyResult<PyColor> {
        if let Some(color) = evaluation_function::basic_evaluation(&self.board) {
            Ok(String::from(color.as_str()))
        } else {
            Ok(String::from("None"))
        }
    }
}
