use csr::Caesar;
use num_cpus as cpus;
use rayon::prelude::*;
use std::error::Error;
use std::io::{self, prelude::*, BufReader, Write};
use std::str;

const BUFFER_SIZE: usize = 32768;

// gets the initial buffers
fn get(cpus: usize) -> Vec<[u8; BUFFER_SIZE]> {
    // Vec to store buffers since thread count is
    // acquired at runtime
    let mut bufs: Vec<[u8; BUFFER_SIZE]> = Vec::with_capacity(cpus);

    // creates a buffer for every logical thread
    // so if you have 4 threads, you will get 4 buffers
    for _ in 0..cpus {
        let buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        bufs.push(buf);
    }

    bufs
}

// runs the caesar cipher in parallel on any BufReader
// that contains a type that implements Read
pub fn run<R: Read>(
    decrypt: bool,
    caesar: Caesar,
    reader: &mut BufReader<R>,
) -> Result<(), Box<dyn Error>> {
    // chooses which mode it will be run in (encrypt/decrypt)
    let translate: fn(Caesar, &mut [u8]) = if decrypt {
        Caesar::decrypt_bytes
    } else {
        Caesar::encrypt_bytes
    };

    let cpus = cpus::get();
    let mut bufs = get(cpus);
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // runs until there is no data left
    'outer: loop {
        let mut filled = cpus;
        let mut bytes = 0;

        // assign messages to each buffer and break the
        // loop if no more data is coming in
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

        // work on each buffer in parallel
        bufs.par_iter_mut().for_each(|buf| translate(caesar, buf));

        // print all filled buffers except the last one
        for buf in 0..filled - 1 {
            let message = str::from_utf8(&bufs[buf])?;
            write!(&mut handle, "{}", message)?;
        }

        // print the last filled buffer until the amount of
        // bytes read (to make sure no junk is printed)
        let last = str::from_utf8(&bufs[filled - 1][0..bytes])?;
        write!(&mut handle, "{}", last)?;
    }

    Ok(())
}
