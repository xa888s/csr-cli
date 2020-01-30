use rayon::prelude::*;
use std::io;
use std::io::Write;

pub use caesar::Caesar;

pub fn run_jobs(text: String, translate: fn(&Caesar, String) -> String, key: u8, threads: usize) {
    let (jobs, size) = get_jobs(threads, text.len());

    // new caesar struct
    let caesar = Caesar::new(key);

    // iterate over all threads and assign messages to each one
    let mut results: Vec<String> = (0..jobs)
        .into_par_iter()
        .map(|index| {
            let chunk = String::from(&text[index * size..(index + 1) * size]);
            translate(&caesar, chunk)
        })
        .collect();

    // last job is done on the main thread
    let last = String::from(&text[size * jobs..text.len()]);
    results.push(translate(&caesar, last));

    print_results(results);
}

fn get_jobs(threads: usize, length: usize) -> (usize, usize) {
    let size: usize = length / threads;

    // if the length of the message is less than the thread count, do the job on one thread only
    if length < threads {
        (0, size)
    } else {
        (threads, size)
    }
}

fn print_results(vec: Vec<String>) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in vec {
        write!(&mut handle, "{}", &line).expect("Failed to write to stdout");
    }
    write!(&mut handle, "\n").expect("Failed to write to stdout")
}
