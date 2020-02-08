use super::buffers;
use csr::Caesar;
use std::error::Error;

use std::fs::File;
pub use std::io::{self, prelude::*, BufReader};

pub fn run(decrypt: bool, caesar: Caesar, path: &str) -> Result<(), Box<dyn Error>> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    buffers::run(decrypt, caesar, &mut reader)?;

    Ok(())
}
