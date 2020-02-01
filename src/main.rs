mod error;
mod input;
mod jobs;
use clap::{App, Arg};
use error::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // get command line args
    let matches = App::new("caesar-cli")
        .version("0.4.1")
        .about("A simple caesar cipher and decryption tool")
        .author("desolate")
        .arg(
            Arg::with_name("decrypt")
                .help("Decryption mode (default: encryption)")
                .short("d")
                .long("decrypt"),
        )
        .arg(
            Arg::with_name("key")
                .help("Number to shift by")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("input")
                .help("Input to encrypt/decrypt")
                .index(2),
        )
        .get_matches();

    // parsing key
    let mut key = matches.value_of("key").unwrap().parse::<u8>()?;
    key = check_key(key)?;

    let text = if !matches.is_present("input") {
        input::get()?
    } else {
        String::from(matches.value_of("input").unwrap())
    };

    let mode = matches.is_present("decrypt");

    // run main code
    jobs::run(text, key, mode);
    Ok(())
}

fn check_key(key: u8) -> Result<u8, ShiftSizeError> {
    match key {
        0..=26 => Ok(key),
        _ => Err(ShiftSizeError::TooBig),
    }
}
