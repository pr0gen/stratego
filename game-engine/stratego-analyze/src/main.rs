use std::env;
use std::sync::{Arc, Mutex};

pub mod threads;
pub mod writter;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let file_name = args[1].clone();
    let number_of_thread = args[2]
        .parse::<usize>()
        .unwrap_or_else(|e| panic!("Failed to parse number of thread, {}", e));
    let number_of_parties = args[3]
        .parse::<usize>()
        .unwrap_or_else(|e| panic!("Failed to parse number of parties, {}", e));
    let first_ai_name = args[4].clone();
    let second_ai_name = args[5].clone();
    let mut threads = Vec::new();

    let per_threads = number_of_parties / number_of_thread;
    let file_name = Arc::new(Mutex::new(file_name));

    for _ in 0..per_threads {
        for j in 0..number_of_thread {
            threads.push(threads::spawn_thread_for_stratego(
                j,
                Arc::clone(&file_name),
                first_ai_name.clone(),
                second_ai_name.clone(),
            ));
        }
    }

    // Wait for all threads to be finished
    threads.into_iter().for_each(|thread| {
        if let Err(e) = thread.unwrap().join() {
            println!("Failed to get result from one thread {:?}", e);
        }
    });
}
