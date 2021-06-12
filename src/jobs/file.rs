use super::{buffers, Mode};
use csr::Caesar;

use std::fs::File;
use std::io::{self, BufReader};

pub fn run(switch: Mode, caesar: Caesar, path: &str) -> Result<(), io::Error> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let translate = match switch {
        Mode::Encrypt => Caesar::encrypt_bytes,
        Mode::Decrypt => Caesar::decrypt_bytes,
    };

    buffers::run(translate, caesar, &mut reader)?;

    Ok(())
}
