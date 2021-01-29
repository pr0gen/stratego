use pyo3::prelude::{pyclass, pymethods, PyResult};
use serde::{Deserialize, Serialize};
use super::piece::{Color, Piece, PieceType};
use std::fmt;
use crate::parse;

pub type PyCoord = (i16, String);
pub type PyCoords = (PyCoord, PyCoord);
pub type PyState = String;

pub fn into(co: PyCoords) -> (Coordinate, Coordinate) {
    (Coordinate::from(co.0), Coordinate::from(co.1))
}

pub fn from(coord: &Coordinate) -> PyCoord {
    (coord.get_x(), parse::parse_i16_to_str(coord.get_y() + 65))
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Case {
    state: State,
    coordinate: Coordinate,
    content: Piece,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub struct Coordinate {
    x: i16,
    y: i16,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub enum State {
    Unreachable,
    Empty,
    Full,
}

pub fn create_full_case(coordinate: Coordinate, content: Piece) -> Case {
    Case {
        state: State::Full,
        coordinate,
        content,
    }
}

pub fn create_unreachable_case(coordinate: Coordinate) -> Case {
    Case {
        state: State::Unreachable,
        coordinate,
        content: Piece::new(PieceType::Null, Color::None),
    }
}

pub fn create_empty_case(coordinate: Coordinate) -> Case {
    Case {
        state: State::Empty,
        coordinate,
        content: Piece::new(PieceType::Null, Color::None),
    }
}

#[pymethods]
impl Case {
    pub fn py_get_state(&self) -> PyResult<String> {
        let state = self.state;
        Ok(state.to_string())
    }

    pub fn py_get_content(&self) -> PyResult<String> {
        let content = &self.content;
        Ok(content.to_string())
    }
}

impl Case {
    pub fn new(state: State, coordinate: Coordinate, content: Piece) -> Self {
        Case {
            state,
            coordinate,
            content,
        }
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn get_coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    pub fn get_content(&self) -> &Piece {
        &self.content
    }

    pub fn display(&self) -> String {
        match self.state {
            State::Unreachable => String::from("XXXXX"),
            State::Full | State::Empty => self.content.display(),
        }
    }

    pub fn display_by_color(&self, color: &Color) -> String {
        let content_color = self.content.get_color();
        if content_color == &Color::None || content_color == color {
            self.display()
        } else {
            String::from("-----")
        }
    }
}

impl Coordinate {
    pub fn new(x: i16, y: i16) -> Self {
        Coordinate { x, y }
    }

    pub fn get_x(&self) -> i16 {
        self.x
    }

    pub fn get_y(&self) -> i16 {
        self.y
    }
}

impl From<&str> for State {
    fn from(py_state: &str) -> Self {
        match py_state {
            "Full" => State::Full,
            "Empty" => State::Empty,
            "Unreachable" => State::Unreachable,
            _ => State::Empty,
        }
    }
}

impl Coordinate {
    pub fn into(&self) -> PyCoord {
        (self.x, parse::parse_i16_to_str(self.y))
    }
}

impl From<PyCoord> for Coordinate {
    fn from(py_coord: PyCoord) -> Self {
        Coordinate::new(py_coord.0, parse::parse_letter_to_i16(py_coord.1.as_str()))
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display: &str = match self {
            State::Unreachable => "Unreachable",
            State::Empty => "Empty",
            State::Full => "Full",
        };
        write!(f, "{}", display)
    }
}

