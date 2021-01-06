use pyo3::prelude::pyclass;
use serde::{Serialize, Deserialize};

use crate::parse;
use super::piece::{Color, Piece, PieceType};


pub type PyCoord = (i16, String);
pub type PyCoords = (PyCoord, PyCoord);

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


pub fn into(co: PyCoords) -> (Coordinate, Coordinate) {
    let co_0 = co.0;
    let co_1 = co.1;

    (
        Coordinate::new(co_0.0, parse::parse_letter_to_i16(co_0.1.as_str())),
        Coordinate::new(co_1.0, parse::parse_letter_to_i16(co_1.1.as_str())),
    )
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
        content: Piece::new(PieceType::Null, Box::new(Color::None)),
    }
}

pub fn create_empty_case(coordinate: Coordinate) -> Case {
    Case {
        state: State::Empty,
        coordinate,
        content: Piece::new(PieceType::Null, Box::new(Color::None)),
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

    use super::{create_empty_case, create_full_case, create_unreachable_case, Coordinate, State};
    use crate::board::piece::{Color, Piece, PieceType};

    #[test]
    fn should_create_full_case() {
        let case = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Captain, Box::new(Color::Blue)),
        );

        assert_eq!(State::Full, case.state);
        assert_eq!(Coordinate::new(0, 0), case.coordinate);
        assert_eq!(
            Piece::new(PieceType::Captain, Box::new(Color::Blue)),
            case.content
        );
    }

    #[test]
    fn should_create_empty_case() {
        let case = create_empty_case(Coordinate::new(0, 0));

        assert_eq!(State::Empty, case.state);
        assert_eq!(Coordinate::new(0, 0), case.coordinate);
        assert_eq!(
            Piece::new(PieceType::Null, Box::new(Color::None)),
            case.content
        );
    }

    #[test]
    fn should_create_unreachable_case() {
        let case = create_unreachable_case(Coordinate::new(0, 0));

        assert_eq!(State::Unreachable, case.state);
        assert_eq!(Coordinate::new(0, 0), case.coordinate);
        assert_eq!(
            Piece::new(PieceType::Null, Box::new(Color::None)),
            case.content
        );
    }

    #[test]
    fn should_display() {
        let case = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Captain, Box::new(Color::Blue)),
        );
        assert_eq!(String::from("Cap B"), case.display());

        let case = create_unreachable_case(Coordinate::new(0, 0));
        assert_eq!(String::from("XXXXX"), case.display());

        let case = create_empty_case(Coordinate::new(0, 0));
        assert_eq!(String::from("     "), case.display());
    }
}
