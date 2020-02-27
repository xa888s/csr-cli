use super::Mode;
use csr::Caesar;
use num_cpus as cpus;
use rayon::prelude::*;
use std::error::Error;
use std::io::{self, prelude::*, BufReader};

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
    switch: Mode,
    caesar: Caesar,
    reader: &mut BufReader<R>,
) -> Result<(), Box<dyn Error>> {
    let translate = match switch {
        Mode::Encrypt => Caesar::encrypt_bytes,
        Mode::Decrypt => Caesar::decrypt_bytes,
    };

    let cpus = cpus::get();
    let mut bufs = get(cpus);
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    let mut filled = cpus - 1;
    let mut bytes = BUFFER_SIZE;

    // runs until there is no data left
    loop {
        if filled < (cpus - 1) {
            break;
        }
        // assign messages to each buffer and break the
        // loop if no more data is coming in
        for (i, buf) in (&mut bufs).iter_mut().enumerate() {
            bytes = reader.read(buf)?;

            if bytes != BUFFER_SIZE {
                filled = i;
                break;
            }
        }

        // work on each buffer in parallel
        bufs.par_iter_mut()
            .take(filled + 1)
            .for_each(|buf| translate(caesar, buf));

        // print all filled buffers except the last one
        for buf in bufs.iter().take(filled) {
            writer.write(buf)?;
        }

        // print the last filled buffer until the amount of
        // bytes read (to make sure no junk is printed)
        let last = &bufs[filled][0..bytes];
        writer.write(last)?;
    }

    Ok(())
}
