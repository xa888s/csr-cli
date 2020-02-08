mod buffer;
mod file;
mod stdin;

pub use csr::Caesar;

pub fn run(source: Source, decrypt: bool, key: u8) -> Result<(), Box<dyn std::error::Error>> {
    let translate = if decrypt {
        Caesar::decrypt
    } else {
        Caesar::encrypt
    };

    match source {
        Source::Text(s) => {
            let caesar = Caesar::new(key);
            println!("{}", translate(caesar, s));
        }
        Source::File(p) => file::run(decrypt, key, p)?,
        Source::Stdin => unimplemented!(),
    };

    Ok(())
}

pub enum Source<'a> {
    File(&'a str),
    Text(&'a str),
    Stdin,
}
