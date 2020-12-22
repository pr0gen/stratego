use super::piece::{Color, Piece, PieceType};

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Case {
    state: State,
    coordinate: Coordinate,
    content: Piece,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Copy, Clone)]
pub struct Coordinate(i16, i16);

#[derive(Debug, Eq, PartialEq, PartialOrd, Copy, Clone)]
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
        Coordinate(x, y)
    }

    pub fn get_x(&self) -> i16 {
        self.0
    }

    pub fn get_y(&self) -> i16 {
        self.1
    }
}

#[cfg(test)]
mod test {

    use super::{create_empty_case, create_full_case, create_unreachable_case, Coordinate, State};
    use crate::board::piece::{Color, Piece, PieceType};

    #[test]
    fn should_create_full_case() {
        let case = create_full_case(
            Coordinate(0, 0),
            Piece::new(PieceType::Captain, Box::new(Color::Blue)),
        );

        assert_eq!(State::Full, case.state);
        assert_eq!(Coordinate(0, 0), case.coordinate);
        assert_eq!(
            Piece::new(PieceType::Captain, Box::new(Color::Blue)),
            case.content
        );
    }

    #[test]
    fn should_create_empty_case() {
        let case = create_empty_case(Coordinate(0, 0));

        assert_eq!(State::Empty, case.state);
        assert_eq!(Coordinate(0, 0), case.coordinate);
        assert_eq!(
            Piece::new(PieceType::Null, Box::new(Color::None)),
            case.content
        );
    }

    #[test]
    fn should_create_unreachable_case() {
        let case = create_unreachable_case(Coordinate(0, 0));

        assert_eq!(State::Unreachable, case.state);
        assert_eq!(Coordinate(0, 0), case.coordinate);
        assert_eq!(
            Piece::new(PieceType::Null, Box::new(Color::None)),
            case.content
        );
    }

    #[test]
    fn should_display() {
        let case = create_full_case(
            Coordinate(0, 0),
            Piece::new(PieceType::Captain, Box::new(Color::Blue)),
        );
        assert_eq!(String::from("Cap B"), case.display());

        let case = create_unreachable_case(Coordinate(0, 0));
        assert_eq!(String::from("XXXXX"), case.display());

        let case = create_empty_case(Coordinate(0, 0));
        assert_eq!(String::from("     "), case.display());
    }
}
