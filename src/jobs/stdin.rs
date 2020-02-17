use super::{buffers, Mode};
use csr::Caesar;
use std::error::Error;

use std::io::{self, BufReader};

pub fn run(switch: Mode, caesar: Caesar) -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let input = stdin.lock();
    let mut reader = BufReader::new(input);
    buffers::run(switch, caesar, &mut reader)?;

    Ok(())
}
