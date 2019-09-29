mod input;
mod threads;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text: String;
    match args.len() {
        1 | 2 => {
            eprintln!("Please specify an option.");
            std::process::exit(1)
        }
        3 => text = input::get_input(),
        4 => text = String::from(&args[3]),
        _ => { 
            eprintln!("Please specify 3 or less options");
            std::process::exit(1);
        }
    } 
    let length = text.len();
    let mode = String::from(&args[1]);
    let key = match args[2].parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please enter a valid integer of usize (0 to 2^64-1)");
            std::process::exit(1);
        }
    };
    let jobs = threads::split_jobs(6, text);
    threads::run_jobs(jobs, mode, key, length);
}
