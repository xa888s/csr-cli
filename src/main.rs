mod input;
mod threads;
use num_cpus;
use std::env;
use threads::Caesar;

fn main() {
    // get command line args
    let args: Vec<String> = env::args().collect();

    // checking for correct usage
    let text = match args.len() {
        1 | 2 => {
            eprintln!("Please specify an option.");
            std::process::exit(1);
        }
        3 => input::get_input(),
        4 => String::from(&args[3]),
        _ => {
            eprintln!("Please specify 3 or less options");
            std::process::exit(1);
        }
    };

    // get run mode
    let operation = match args[1].as_str() {
        "encrypt" => Caesar::encrypt,
        "decrypt" => Caesar::decrypt,
        _ => {
            eprintln!("Mode must be encrypt or decrypt");
            std::process::exit(1);
        }
    };

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
    threads::run_jobs(text, operation, key, num_cpus::get());
}
