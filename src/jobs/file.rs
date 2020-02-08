use super::buffer;
use csr::Caesar;
use num_cpus as cpus;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Write};

pub fn run(decrypt: bool, key: u8, path: &str) -> Result<(), Box<dyn Error>> {
    let translate: fn(Caesar, &mut [u8]) = if decrypt {
        Caesar::decrypt_bytes
    } else {
        Caesar::encrypt_bytes
    };

    let caesar = Caesar::new(key);

    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut bufs: Vec<[u8; 8192]> = buffer::get();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    'outer: loop {
        let mut filled = cpus::get();
        let mut bytes = 0;
        for (i, buf) in (&mut bufs).iter_mut().enumerate() {
            let size = reader.read(buf)?;

            if size == 0 {
                match i {
                    0 => break 'outer,
                    // the amount of cpus filled with data
                    _ => {
                        filled = i;
                        break;
                    }
                }
            } else {
                bytes = size;
            }
        }

        bufs.par_iter_mut().for_each(|buf| translate(caesar, buf));

        // operating on each filled buffer
        for buf in 0..filled - 1 {
            let message = std::str::from_utf8(&bufs[buf])?;
            write!(&mut handle, "{}", message)?;
        }
        let last = std::str::from_utf8(&bufs[filled - 1][0..bytes])?;
        write!(&mut handle, "{}", last)?;
    }

    Ok(())
}
