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
    let key = get_key(&args[2]);
    threads::run_jobs(text, mode, key, num_cpus::get());
}

fn get_key(arg: &String) -> u8 {
    let key = match arg.parse::<u8>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please enter a valid integer of 0 to 26");
            std::process::exit(1);
        }
    };
    if key > 26 {
        eprintln!("Please enter a valid integer of from 0 to 26");
        std::process::exit(1);
    }
    key
}
