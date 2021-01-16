use pyo3::prelude::{pyfunction, pyclass};
use serde::{Deserialize, Serialize};

use super::piece::{Color, Piece, PieceType};
use crate::parse;

pub type PyCoord = (i16, String);
pub type PyCoords = (PyCoord, PyCoord);

pub fn into(co: PyCoords) -> (Coordinate, Coordinate) {
    (Coordinate::from(co.0), Coordinate::from(co.1))
}

pub fn from(coord: &Coordinate) -> PyCoord {
    (coord.get_x(), parse::parse_i16_to_str(coord.get_y() + 65)) 
}

#[pyclass]
#[derive(Serialize, Deserialize, Hash, Debug, Clone, Ord, Eq, PartialOrd, PartialEq)]
pub struct Case {
    state: State,
    coordinate: Coordinate,
    content: Piece,
}

#[pyclass]
#[derive(Serialize, Deserialize, Hash, Debug, Eq, Ord, PartialEq, PartialOrd, Copy, Clone)]
pub struct Coordinate {
    x: i16,
    y: i16,
}

#[derive(Serialize, Deserialize, Hash, Debug, Eq, Ord, PartialEq, PartialOrd, Copy, Clone)]
pub enum State {
    Unreachable,
    Empty,
    Full,
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

impl Case {
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

#[cfg(test)]
mod test {

    use super::{
        create_empty_case, create_full_case, create_unreachable_case, into, Coordinate, State,
    };
    use crate::board::piece::{Color, Piece, PieceType};

    #[test]
    fn should_parse_py_coord() {
        let py_coords = ((0, String::from("B")), (1, String::from("C")));
        assert_eq!(
            (Coordinate::new(0, 1), Coordinate::new(1, 2)),
            into(py_coords)
        );
        let py_coords = ((3, String::from("E")), (2, String::from("D")));
        assert_eq!(
            (Coordinate::new(3, 4), Coordinate::new(2, 3)),
            into(py_coords)
        );
    }

    #[test]
    fn should_create_full_case() {
        let case = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Captain, Color::Blue),
        );

        assert_eq!(State::Full, case.state);
        assert_eq!(Coordinate::new(0, 0), case.coordinate);
        assert_eq!(
            Piece::new(PieceType::Captain, Color::Blue),
            case.content
        );
    }

    #[test]
    fn should_create_empty_case() {
        let case = create_empty_case(Coordinate::new(0, 0));

        assert_eq!(State::Empty, case.state);
        assert_eq!(Coordinate::new(0, 0), case.coordinate);
        assert_eq!(
            Piece::new(PieceType::Null, Color::None),
            case.content
        );
    }

    #[test]
    fn should_create_unreachable_case() {
        let case = create_unreachable_case(Coordinate::new(0, 0));

        assert_eq!(State::Unreachable, case.state);
        assert_eq!(Coordinate::new(0, 0), case.coordinate);
        assert_eq!(
            Piece::new(PieceType::Null, Color::None),
            case.content
        );
    }

    #[test]
    fn should_display() {
        let case = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Captain, Color::Blue),
        );
        assert_eq!(String::from("Cap B"), case.display());

        let case = create_unreachable_case(Coordinate::new(0, 0));
        assert_eq!(String::from("XXXXX"), case.display());

        let case = create_empty_case(Coordinate::new(0, 0));
        assert_eq!(String::from("     "), case.display());
    }
}
