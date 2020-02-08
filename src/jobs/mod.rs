mod buffers;
mod file;
mod stdin;

pub use csr::Caesar;

pub fn run(source: Source, decrypt: bool, key: u8) -> Result<(), Box<dyn std::error::Error>> {
    let translate = if decrypt {
        Caesar::decrypt
    } else {
        Caesar::encrypt
    };

    let caesar = Caesar::new(key);

    match source {
        Source::Text(s) => {
            println!("{}", translate(caesar, s));
        }
        Source::File(p) => file::run(decrypt, caesar, p)?,
        Source::Stdin => stdin::run(decrypt, caesar)?,
    };

    Ok(())
}

pub enum Source<'a> {
    File(&'a str),
    Text(&'a str),
    Stdin,
}
