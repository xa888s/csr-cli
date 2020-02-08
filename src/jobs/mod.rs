mod buffers;
mod file;
mod stdin;

pub use csr::Caesar;
use std::error::Error;

pub fn run(source: Source, decrypt: bool, key: u8) -> Result<(), Box<dyn Error>> {
    let caesar = Caesar::new(key);

    match source {
        // if input is provided as an argument
        // just run one job on the main thread
        Source::Text(s) => {
            let translate = if decrypt {
                Caesar::decrypt
            } else {
                Caesar::encrypt
            };
            println!("{}", translate(caesar, s));
        }
        // if input is provided by a bufferable
        // source, run in parallel
        Source::File(p) => file::run(decrypt, caesar, p)?,
        Source::Stdin => stdin::run(decrypt, caesar)?,
    };

    Ok(())
}

// where the data to process comes from
pub enum Source<'a> {
    File(&'a str),
    Text(&'a str),
    Stdin,
}
