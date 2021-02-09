use std::env;

pub mod threads;
pub mod writter;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let file_name = args[1].clone();
    let number_of_thread = args[2].parse::<usize>()
                .unwrap_or_else(|e| panic!("Failed to parse number of thread, {}", e));
    let number_of_parties = args[3].parse::<usize>()
                .unwrap_or_else(|e| panic!("Failed to parse number of parties, {}", e));

    let mut threads = Vec::new();

    let per_threads = number_of_parties / number_of_thread;

    for _ in 0..per_threads {
        for j in 0..number_of_thread {
            threads.push(threads::spawn_thread_for_stratego(j, file_name.clone()));
        }
    }

    // Wait for all threads to be finished
    threads.into_iter()
        .for_each(|thread| thread.unwrap().join().unwrap());
}
