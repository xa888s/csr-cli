mod buffers;
mod file;
mod stdin;

pub use csr::Caesar;
use std::error::Error;

pub fn run(source: Source, switch: Mode, key: u8) -> Result<(), Box<dyn Error>> {
    let caesar = Caesar::new(key);

    match source {
        // if input is provided as an argument
        // just run one job on the main thread
        Source::Text(s) => {
            let translate = match switch {
                Mode::Encrypt => Caesar::encrypt,
                Mode::Decrypt => Caesar::decrypt,
            };
            println!("{}", translate(caesar, s));
        }
        // if input is provided by a bufferable
        // source, run in parallel
        Source::File(p) => file::run(switch, caesar, p)?,
        Source::Stdin => stdin::run(switch, caesar)?,
    };

    Ok(())
}

// where the data to process comes from
pub enum Source<'a> {
    File(&'a str),
    Text(&'a str),
    Stdin,
}

pub enum Mode {
    Encrypt,
    Decrypt,
}
