use stratego_lib::board::piece::Color;
use stratego_lib::board::Board;
use stratego_lib::engine::{Engine, StrategoEngine};
use stratego_lib::engine_utils::game_is_over;
//use stratego_lib::game_pool::{self, Game};
use stratego_lib::player::ai_player::AIPlayer;
use stratego_lib::player::HumanPlayer;
use stratego_lib::board::classic_board::create_stratego_board;
//use stratego_lib::{GAME_POOL, GAME_POOL_ID};



fn main() {
    //let engine = StrategoEngine::new(
        //create_stratego_board(),
        //(
            //HumanPlayer::new(Color::Red, String::from("Tigran")),
            ////HumanPlayer::new(Color::Blue, String::from("Cassiopee")),
            //AIPlayer::new(Color::Blue, String::from("test")),
        //),
    //);

    //let game = Game::new(*GAME_POOL_ID.lock().unwrap(), engine);
    //*GAME_POOL_ID.lock().unwrap() += 1;

    //game_pool::register_to_pool(game)
        //.unwrap_or_else(|_| panic!("Failed to get game pool"));
    //let game_id = 0;
    //let pool = GAME_POOL.lock().unwrap();
    //let game = pool.find_game_by_id(game_id).
        //unwrap_or_else(|| panic!("Failed to find game {} ", game_id));

    //let mut engine = game.get_engine().clone();
    //println!("{}", engine.display_by_color(&engine.get_turn()));
    //loop {
        //let board = engine.status();
        //match game_is_over(board.state()) {
            //Ok(Color::Red) => {
                //println!("Red wins");
                //break;
            //}
            //Ok(Color::Blue) => {
                //println!("Blue wins");
                //break;
            //}
            //_ => {
                //if let Err(e) = engine.moving() {
                    //panic!("{:#?}", e)
                //}
            //}
        //}
    //}
}
