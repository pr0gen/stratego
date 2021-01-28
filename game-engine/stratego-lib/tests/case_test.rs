#[cfg(test)]
mod case_tests {

    use stratego_lib::board::case::{
        create_empty_case, create_full_case, create_unreachable_case, into, Coordinate, State,
    };
    use stratego_lib::board::piece::{Color, Piece, PieceType};

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

        assert_eq!(&State::Full, case.get_state());
        assert_eq!(&Coordinate::new(0, 0), case.get_coordinate());
        assert_eq!(
            &Piece::new(PieceType::Captain, Color::Blue),
            case.get_content()
        );
    }

    #[test]
    fn should_create_empty_case() {
        let case = create_empty_case(Coordinate::new(0, 0));

        assert_eq!(&State::Empty, case.get_state());
        assert_eq!(
            &Piece::new(PieceType::Null, Color::None),
            case.get_content()
        );
    }

    #[test]
    fn should_create_unreachable_case() {
        let case = create_unreachable_case(Coordinate::new(0, 0));

        assert_eq!(&State::Unreachable, case.get_state());
        assert_eq!(
            &Piece::new(PieceType::Null, Color::None),
            case.get_content()
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
