mod error;
mod jobs;
use clap::{App, Arg};
use error::*;
use jobs::Source;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // get command line args
    let matches = App::new("csr")
        .version("0.7.0")
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
        .arg(
            Arg::with_name("file")
                .help("File to encrypt/decrypt")
                .takes_value(true)
                .short("f")
                .long("file"),
        )
        .get_matches();

    // parsing key
    let mut key = match matches.value_of("key") {
        Some(k) => k.parse::<u8>()?,
        None => unreachable!(),
    };

    key = check_key(key)?;

    let source = match matches.value_of("input") {
        Some(s) => Source::Text(s),
        None => match matches.value_of("file") {
            Some(f) => Source::File(f),
            None => Source::Stdin,
        },
    };

    let switch = matches.is_present("decrypt");

    // run main code
    jobs::run(source, switch, key)?;
    Ok(())
}

fn check_key(key: u8) -> Result<u8, ShiftSizeError> {
    match key {
        0..=26 => Ok(key),
        _ => Err(ShiftSizeError::TooBig),
    }
}
