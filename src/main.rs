mod input;
mod threads;
use num_cpus;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text: String;
    match args.len() {
        1 | 2 => {
            eprintln!("Please specify an option.");
            std::process::exit(1);
        }
        3 => text = input::get_input(),
        4 => text = String::from(&args[3]),
        _ => {
            eprintln!("Please specify 3 or less options");
            std::process::exit(1);
        }
    }
    let mode = String::from(&args[1]);
    let key = match args[2].parse::<u8>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please enter a valid integer of u8 (0 to 26)");
            std::process::exit(1);
        }
    };
    threads::run_jobs(text, mode, key, num_cpus::get());
}
