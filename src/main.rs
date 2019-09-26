mod input;
mod crypt;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => panic!("WHIP!!!!"),
        2 => {
            let mode = args[1].parse::<String>().unwrap();
            let old_text = input::get_input("Please enter some text:");
            let key = input::get_input("Please enter a key (0 to 2^64-1):")
                .trim()
                .parse::<usize>()
                .ok()
                .expect("Not a usize!");

            if mode == "encrypt" {
                let text = crypt::encrypt(&old_text, key);
                println!("Encrypted text: {}", text);
            } else if mode == "decrypt" {
                let text = crypt::decrypt(&old_text, key); 
                println!("Decrypted text: {}", text);
            } else {
                panic!("What are you doing lmao");
            }
        },
        _ => panic!("BRUH!!!!"),
    }
}
