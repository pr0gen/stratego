use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;
use std::env::current_dir;
use crate::py_bindings::board_wrapper::StrategoBoardWrapper;
use crate::board::case::{self, Case, Coordinate, PyCoord};
use crate::board::piece::{PyColor, Piece, PyPieceType};
use crate::error::StrategoError;
use crate::engine_utils;

pub mod board_wrapper;
pub mod evaluation_function;

#[pymodule]
fn stratego_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<StrategoBoardWrapper>()?;
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


#[pyfunction]
fn rust_create_empty_stratego_board() -> PyResult<StrategoBoardWrapper> {
    let board = engine_utils::create_empty_stratego_board();
    Ok(StrategoBoardWrapper::new(board))
}

#[pyfunction]
fn rust_create_stratego_board() -> PyResult<StrategoBoardWrapper> {
    let board = engine_utils::create_stratego_board();
    Ok(StrategoBoardWrapper::new(board))
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
