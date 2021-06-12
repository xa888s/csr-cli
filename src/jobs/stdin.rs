use super::{buffers, Mode};
use csr::Caesar;

use std::io::{self, BufReader};

pub fn run(switch: Mode, caesar: Caesar) -> Result<(), io::Error> {
    let stdin = io::stdin();
    let input = stdin.lock();
    let mut reader = BufReader::new(input);

    let translate = match switch {
        Mode::Encrypt => Caesar::encrypt_bytes,
        Mode::Decrypt => Caesar::decrypt_bytes,
    };

    buffers::run(translate, caesar, &mut reader)?;

    Ok(())
}
