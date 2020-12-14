#[derive(Debug, Eq, PartialEq)]
pub enum AvailableMove {
    Immovable,
    Normal,
    Scout,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Move(i16, i16);

impl Move {
    pub fn new(available_move: AvailableMove) -> Self {
        match available_move {
            AvailableMove::Immovable => Move(0, 0),
            AvailableMove::Normal => Move(0, 1),
            AvailableMove::Scout => Move(0, 10),
        }
    }

    pub fn get_min(&self) -> i16 {
        self.0
    }

    pub fn get_max(&self) -> i16 {
        self.1
    }

    pub fn equals(&self, available_move: AvailableMove) -> bool {
        let to = Move::new(available_move);
        to.0 == self.0 && to.1 == self.1
    }
}

#[cfg(test)]
mod test {
    use super::{AvailableMove, Move};

    #[test]
    fn should_create_move() {
        assert_eq!(Move(0, 0), Move::new(AvailableMove::Immovable));
        assert_eq!(Move(0, 1), Move::new(AvailableMove::Normal));
        assert_eq!(Move(0, 10), Move::new(AvailableMove::Scout));
    }

    #[test]
    fn should_get_min_and_max() {
        let m = Move::new(AvailableMove::Immovable);
        assert_eq!(0, m.get_min());
        assert_eq!(0, m.get_max());
    }
}
