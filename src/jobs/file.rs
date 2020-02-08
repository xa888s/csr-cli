use csr::Caesar;
use num_cpus as cpus;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Write};

pub fn run(decrypt: bool, key: u8, path: &str) -> Result<(), Box<dyn Error>> {
    let translate = if decrypt {
        Caesar::decrypt_bytes
    } else {
        Caesar::encrypt_bytes
    };

    let caesar = Caesar::new(key);
    let cpus = cpus::get();

    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut bufs: Vec<[u8; 8192]> = buffers(cpus);

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    'outer: loop {
        for (i, buf) in (&mut bufs).iter_mut().enumerate() {
            let size = reader.read(buf)?;

            if size == 0 {
                match i {
                    0 => break 'outer,
                    _ => break,
                }
            }
        }

        bufs.par_iter_mut().for_each(|buf| translate(caesar, buf));

        for buf in &bufs {
            let message = std::str::from_utf8(buf)?;
            write!(&mut handle, "{}", message)?;
        }
    }

    Ok(())
}

fn buffers(cpus: usize) -> Vec<[u8; 8192]> {
    let mut bufs: Vec<[u8; 8192]> = Vec::with_capacity(cpus);

    for _ in 0..=cpus {
        let buf: [u8; 8192] = [0; 8192];
        bufs.push(buf);
    }

    bufs
}
