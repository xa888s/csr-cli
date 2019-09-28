mod crypt;
mod input;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text;
    match args.len() {
        1 | 2 => {
            println!("Please specify an option.");
            std::process::exit(1)
        }
        3 => text = input::get_input(),
        4 => text = String::from(&args[3]),
        _ => { 
            println!("Please specify 3 or less options");
            std::process::exit(1);
        }
    } 

    // TODO: replace unwrap() with some error handling
    let mode = String::from(&args[1]);
    let key = args[2].parse::<usize>().unwrap();
    
    if mode == "encrypt" {
        let ciphertext = crypt::encrypt(&text, key);
        println!("{}", ciphertext);
    } else if mode == "decrypt" {
        let plaintext = crypt::decrypt(&text, key);
        println!("{}", plaintext);
    } else {
        println!("Mode should be encrypt or decrypt");
        std::process::exit(1);
    }
}
