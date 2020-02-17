use super::{buffers, Mode};
use csr::Caesar;
use std::error::Error;

use std::fs::File;
use std::io::BufReader;

pub fn run(switch: Mode, caesar: Caesar, path: &str) -> Result<(), Box<dyn Error>> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    buffers::run(switch, caesar, &mut reader)?;

    Ok(())
}
