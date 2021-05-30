use std::env;
use std::sync::{Arc, Mutex};

pub mod threads;
pub mod writter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args[1].clone();
    let number_of_parties = args[2]
        .parse::<usize>()
        .unwrap_or_else(|e| panic!("Failed to parse number of parties, {}", e));
    let first_ai_name = args[3].clone();
    let second_ai_name = args[4].clone();

    let file_name = Arc::new(Mutex::new(file_name));

    // let mut threads = Vec::new();
    // for i in 0..number_of_parties {
        // threads.push(threads::spawn_thread_for_stratego(
            // i,
            // Arc::clone(&file_name),
            // first_ai_name.clone(),
            // second_ai_name.clone(),
        // ));
    // }

    // // Wait for all threads to be finished
    // threads.into_iter().for_each(|thread| {
        // if let Err(e) = thread.unwrap().join() {
            // println!("Failed to get result from one thread {:?}", e);
        // }
    // });
    //

    threads::play_many_turns(file_name, number_of_parties as i32, first_ai_name, second_ai_name)
}
