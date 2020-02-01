use num_cpus as cpus;
use rayon::prelude::*;
use std::io;
use std::io::Write;

pub use caesar::Caesar;

pub fn run(text: String, key: u8, decrypt: bool) {
    let (jobs, size) = chunks(text.len());
    let translate = if decrypt {
        Caesar::decrypt
    } else {
        Caesar::encrypt
    };

    // new caesar struct
    let caesar = Caesar::new(key);

    // iterate over all threads and assign messages to each one
    let mut results: Vec<String> = (0..jobs)
        .into_par_iter()
        .map(|index| {
            let chunk = &text[index * size..(index + 1) * size];
            translate(caesar, chunk)
        })
        .collect();

    // last job is done on the main thread
    let last = &text[size * jobs..text.len()];
    results.push(translate(caesar, last));

    match print_results(results) {
        Ok(_) => (),
        Err(err) => panic!("Failed to write to stdout: {}", err),
    }
}

fn chunks(length: usize) -> (usize, usize) {
    let threads = cpus::get();
    let size: usize = length / threads;

    // if the length of the message is less than the thread count, do the job on one thread only
    if length < threads {
        (0, size)
    } else {
        (threads, size)
    }
}

fn print_results(vec: Vec<String>) -> std::io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in vec {
        write!(&mut handle, "{}", &line)?;
    }
    write!(&mut handle, "\n")?;
    Ok(())
}
