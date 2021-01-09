use super::case::{create_empty_case, create_full_case, Case, Coordinate};
use crate::error::StrategoError;

pub fn attack(from: Case, to: Case) -> Result<(Case, Case), StrategoError> {
    let attacker = from.get_content();
    let defenser = to.get_content();

    if attacker.get_color() == defenser.get_color() {
        Err(StrategoError::MoveError(
            String::from("Don't attack your self"),
            from,
            to.get_coordinate().to_owned(),
        ))
    } else if attacker.get_rank() > defenser.get_rank() {
        Ok((
            create_empty_case(*from.get_coordinate()),
            create_full_case(*to.get_coordinate(), from.get_content().clone()),
        ))
    } else if attacker.get_rank() == defenser.get_rank() {
        Ok((
            create_empty_case(*from.get_coordinate()),
            create_empty_case(*to.get_coordinate()),
        ))
    } else {
        Ok((
            create_empty_case(*from.get_coordinate()),
            create_full_case(*to.get_coordinate(), to.get_content().clone()),
        ))
    }
}

pub fn check_piece_move(case: &Case, to: &Coordinate) -> bool {
    let m = case.get_content().get_move();
    let c_coord = case.get_coordinate();
    let delta_x = (to.get_x() - c_coord.get_x()).abs();
    let delta_y = (to.get_y() - c_coord.get_y()).abs();
    (delta_x <= m.get_max() && delta_y == 0) || (delta_x == 0 && delta_y <= m.get_max())
}

#[cfg(test)]
mod test {

    use super::{attack, check_piece_move};
    use crate::board::case::{create_full_case, Coordinate, State};
    use crate::board::piece::{Color, Piece, PieceType};

    #[test]
    fn attacker_should_win() {
        let attacker = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Colonel, Color::Blue),
        );
        let defenser = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Lieutenant, Color::Red),
        );
        let res = attack(attacker, defenser).unwrap();

        assert_eq!(res.0.get_state(), &State::Empty);

        let defenser2 = res.1;
        assert_eq!(defenser2.get_state(), &State::Full);
        assert_eq!(defenser2.get_content().get_color(), &Color::Blue);
    }

    #[test]
    fn defenser_should_win() {
        let attacker = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Lieutenant, Color::Blue),
        );
        let defenser = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Colonel, Color::Red),
        );
        let res = attack(attacker, defenser).unwrap();

        assert_eq!(res.0.get_state(), &State::Empty);

        let defenser2 = res.1;
        assert_eq!(defenser2.get_state(), &State::Full);
        assert_eq!(defenser2.get_content().get_color(), &Color::Red);
    }

    #[test]
    fn both_should_loose() {
        let attacker = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Colonel, Color::Blue),
        );
        let defenser = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Colonel, Color::Red),
        );
        let res = attack(attacker, defenser).unwrap();

        assert_eq!(res.0.get_state(), &State::Empty);
        assert_eq!(res.1.get_state(), &State::Empty);
    }

    #[test]
    fn should_enable_move() {
        let mut piece = Piece::new(PieceType::General, Color::Blue);
        let mut case = create_full_case(Coordinate::new(0, 0), piece);
        assert_eq!(check_piece_move(&case, &Coordinate::new(0, 1)), true);

        piece = Piece::new(PieceType::Scout, Color::Blue);
        case = create_full_case(Coordinate::new(0, 0), piece);
        assert_eq!(check_piece_move(&case, &Coordinate::new(0, 9)), true);

        piece = Piece::new(PieceType::General, Color::Blue);
        case = create_full_case(Coordinate::new(0, 0), piece);
        assert_eq!(check_piece_move(&case, &Coordinate::new(1, 0)), true);
    }

    #[test]
    fn should_not_enable_move() {
        let mut piece = Piece::new(PieceType::General, Color::Blue);
        let mut case = create_full_case(Coordinate::new(0, 0), piece);
        assert_eq!(check_piece_move(&case, &Coordinate::new(0, 2)), false);

        piece = Piece::new(PieceType::Bomb, Color::Blue);
        case = create_full_case(Coordinate::new(0, 0), piece);
        assert_eq!(check_piece_move(&case, &Coordinate::new(0, 2)), false);

        piece = Piece::new(PieceType::Bomb, Color::Blue);
        case = create_full_case(Coordinate::new(0, 0), piece);
        assert_eq!(check_piece_move(&case, &Coordinate::new(1, 1)), false);
    }
}
