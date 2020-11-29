use super::case::{create_empty_case, create_full_case, Case, Coordinate};

pub fn attack(from: Case, to: Case) -> (Case, Case) {
    let attacker = from.get_content();
    let defenser = to.get_content();

    if attacker.get_rank() > defenser.get_rank() {
        (
            create_empty_case(*from.get_coordinate()),
            create_full_case(*to.get_coordinate(), from.get_content().clone()),
        )
    } else if attacker.get_rank() == defenser.get_rank() {
        (
            create_empty_case(*from.get_coordinate()),
            create_empty_case(*to.get_coordinate()),
        )
    } else {
        (
            create_full_case(*from.get_coordinate(), to.get_content().clone()),
            create_empty_case(*to.get_coordinate()),
        )
    }
}

pub fn check_piece_move(case: &Case, to: &Coordinate) -> bool {
    let m = case.get_content().get_move();
    let c_coord = case.get_coordinate();
    let delta_x = (to.get_x() - c_coord.get_x()).abs();
    let delta_y = (to.get_y() - c_coord.get_y()).abs();
    if delta_x <= m.get_max() && delta_y == 0 || delta_x == 0 && delta_y <= m.get_max() {
        return true;
    }
    return false;
}

#[cfg(test)]
mod test {

    use crate::board::case::{Coordinate, State, create_full_case};
    use crate::board::piece::{PieceType, Piece, Color};
    
    use super::attack;
    use super::check_piece_move;

    #[test]
    fn attacker_should_win() {
        let attacker = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Colonel, Box::new(Color::Blue)),
        );
        let defenser = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Lieutenant, Box::new(Color::Red)),
        );
        let res = attack(attacker, defenser);

        assert_eq!(res.0.get_state(), &State::Empty);

        let defenser2 = res.1;
        assert_eq!(defenser2.get_state(), &State::Full);
        assert_eq!(defenser2.get_content().get_color(), &Color::Blue);
    }

    #[test]
    fn defenser_should_win() {
        let attacker = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Lieutenant, Box::new(Color::Blue)),
        );
        let defenser = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Colonel, Box::new(Color::Red)),
        );
        let res = attack(attacker, defenser);

        assert_eq!(res.1.get_state(), &State::Empty);

        let defenser2 = res.0;
        assert_eq!(defenser2.get_state(), &State::Full);
        assert_eq!(defenser2.get_content().get_color(), &Color::Red);
    }

    #[test]
    fn both_should_loose() {
        let attacker = create_full_case(
            Coordinate::new(0, 0),
            Piece::new(PieceType::Colonel, Box::new(Color::Blue)),
        );
        let defenser = create_full_case(
            Coordinate::new(0, 1),
            Piece::new(PieceType::Colonel, Box::new(Color::Red)),
        );
        let res = attack(attacker, defenser);

        assert_eq!(res.0.get_state(), &State::Empty);
        assert_eq!(res.1.get_state(), &State::Empty);
    }

    #[test]
    fn should_enable_move() {
        let mut piece = Piece::new(PieceType::General, Box::new(Color::Blue));
        let mut case = create_full_case(Coordinate::new(0, 0), piece);
        assert_eq!(check_piece_move(&case, &Coordinate::new(0, 1)), true);

        piece = Piece::new(PieceType::Scout, Box::new(Color::Blue));
        case = create_full_case(Coordinate::new(0, 0), piece);
        assert_eq!(check_piece_move(&case, &Coordinate::new(0, 9)), true);

        piece = Piece::new(PieceType::General, Box::new(Color::Blue));
        case = create_full_case(Coordinate::new(0, 0), piece);
        assert_eq!(check_piece_move(&case, &Coordinate::new(1, 0)), true);
    }

    #[test]
    fn should_not_enable_move() {
        let mut piece = Piece::new(PieceType::General, Box::new(Color::Blue));
        let mut case = create_full_case(Coordinate::new(0, 0), piece);
        assert_eq!(check_piece_move(&case, &Coordinate::new(0, 2)), false);

        piece = Piece::new(PieceType::Bomb, Box::new(Color::Blue));
        case = create_full_case(Coordinate::new(0, 0), piece);
        assert_eq!(check_piece_move(&case, &Coordinate::new(0, 2)), false);

        piece = Piece::new(PieceType::Bomb, Box::new(Color::Blue));
        case = create_full_case(Coordinate::new(0, 0), piece);
        assert_eq!(check_piece_move(&case, &Coordinate::new(1, 1)), false);
    }
}

