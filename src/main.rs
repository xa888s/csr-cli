mod input;
mod threads;
use num_cpus;
use std::env;
use threads::{Message, Mode};

fn main() {
    // get command line args
    let args: Vec<String> = env::args().collect();

    // forward declare variables so they stay in main scope.
    let text;

    // checking for correct usage
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

    // get run mode
    let mode = match args[1].as_str() {
        "encrypt" => Mode::Encrypt,
        "decrypt" => Mode::Decrypt,
        _ => {
            eprintln!("Mode must be encrypt or decrypt");
            std::process::exit(1);
        }
    };

    let message = Message::new(text);

    // parsing key
    let key = match &args[2].parse::<u8>() {
        Ok(num) => {
            if *num > 26 {
                eprintln!("Please enter a valid integer from 0 to 26");
                std::process::exit(1);
            }
            *num
        }
        Err(_) => {
            eprintln!("Please enter a valid integer from 0 to 26");
            std::process::exit(1);
        }
    };

    // run main code
    threads::run_jobs(message, mode, key, num_cpus::get());
}
