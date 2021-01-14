use std::collections::HashSet;
use std::hash::Hash;

use crate::board::classic_board::StrategoBoard;
use crate::engine::{Engine, StrategoEngine};
use crate::error::StrategoError;
use crate::player::ai_player::AIPlayer;
use crate::player::HumanPlayer;
use crate::GAME_POOL;

pub type HumamAIEngine = StrategoEngine<StrategoBoard, HumanPlayer, AIPlayer>;
pub type HumanAIGamePool = GamePool<HumamAIEngine>;

pub fn register_to_pool(game: Game<HumamAIEngine>) -> Result<(), StrategoError> {
    GAME_POOL.lock().unwrap().register(game)
}

pub fn find_game_by_id(game_id: i128) -> Option<Game<HumamAIEngine>> {
    if let Some(game) = GAME_POOL.lock().unwrap().find_game_by_id(game_id) {
        Some(game.clone())
    } else {
        None
    }
}

#[derive(Debug)]
pub struct GamePool<E>
where
    E: Engine<StrategoBoard>,
{
    games: HashSet<Game<E>>,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Game<E>
where
    E: Engine<StrategoBoard>,
{
    id: i128,
    engine: E,
}

impl<E> Default for GamePool<E>
where
    E: Engine<StrategoBoard> + Hash + Eq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<E> GamePool<E>
where
    E: Engine<StrategoBoard> + Hash + Eq + Clone,
{
    pub fn new() -> Self {
        GamePool {
            games: HashSet::new(),
        }
    }

    pub fn register(&mut self, game: Game<E>) -> Result<(), StrategoError> {
        let game_id = *game.get_id();
        if !self.games.insert(game) {
            Err(StrategoError::InitGameError(String::from(
                "Failed to register game into pool",
            )))
        } else {
            println!("Game has been registered {}", game_id);
            Ok(())
        }
    }

    pub fn find_game_by_id(&self, id: i128) -> Option<&Game<E>> {
        self.games.iter().find(|game| game.is(id))
    }
}

impl<E> Game<E>
where
    E: Engine<StrategoBoard> + Hash + Eq + Clone,
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

    pub fn get_engine_mut(&mut self) -> &mut E {
        &mut self.engine
    }

    pub fn get_id(&self) -> &i128 {
        &self.id
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
