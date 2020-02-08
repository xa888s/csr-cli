use super::buffers;
use csr::Caesar;
use std::error::Error;

use std::io::{self, BufReader};

pub fn run(decrypt: bool, caesar: Caesar) -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let input = stdin.lock();
    let mut reader = BufReader::new(input);
    buffers::run(decrypt, caesar, &mut reader)?;

    Ok(())
}
