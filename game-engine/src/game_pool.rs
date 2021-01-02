use std::collections::HashSet;
use std::hash::Hash;

use crate::engine::Engine;
use crate::error::StrategoError;

pub struct GamePool<E: Engine> {
    games: HashSet<Game<E>>,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Game<E: Engine> {
    id: i128,
    engine: E,
}

impl<E> GamePool<E>
where
    E: Engine + Hash + Eq + Clone,
{
    pub fn new() -> Self {
        GamePool {
            games: HashSet::new(),
        }
    }

    pub fn register(&mut self, game: Game<E>) -> Result<(), StrategoError> {
        if !self.games.insert(game) {
            Err(StrategoError::InitGameError(String::from( "Failed to register game into pool")))
        } else {
            Ok(())
        }
    }

    pub fn find_game_by_id(&self, id: i128) -> Option<&Game<E>> {
        self.games.iter()
            .find(|game| game.is(id))
    }
}


impl<E> Game<E>
where
    E: Engine + Hash + Eq + Clone,
{
    pub fn new(id: i128, engine: E) -> Self {
        Game { id, engine }
    }

    pub fn is(&self, id: i128) -> bool {
        self.id == id
    }

    pub fn get_engine(&self) -> &E {
        &self.engine
    }
}

#[cfg(test)]
mod test {

    use crate::board::classic_board::create_stratego_board;
    use crate::board::piece::Color;
    use crate::engine::StrategoEngine;
    use crate::player::HumanPlayer;

    use super::{Game, GamePool};

    #[test]
    fn should_create_game() {
        let engine = StrategoEngine::new(
            create_stratego_board(),
            (
                HumanPlayer::new(Color::Red, String::from("Tigran")),
                HumanPlayer::new(Color::Blue, String::from("Cassiopee")),
            ),
        );
        let game = Game::new(0, engine);
        assert!(game.is(0));
    }

    #[test]
    fn should_register_and_find() {
        let mut pool = GamePool::new();
        let engine = StrategoEngine::new(
            create_stratego_board(),
            (
                HumanPlayer::new(Color::Red, String::from("Tigran")),
                HumanPlayer::new(Color::Blue, String::from("Cassiopee")),
            ),
        );
        let game = Game::new(0, engine);

        if let Err(e) = pool.register(game) {
            panic!("Should not happens, {}", e.message());
        } else {
            let game = pool.find_game_by_id(0).unwrap();
            assert!(game.is(0));
            assert_eq!(None, pool.find_game_by_id(1));
        }
    }
}
