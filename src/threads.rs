use std::io;
use std::io::Write;
use std::thread;

pub use caesar::Caesar;

pub fn run_jobs(text: String, translate: fn(&Caesar, String) -> String, key: u8, threads: usize) {
    let (jobs, size) = get_jobs(threads, text.len());

    // new caesar struct
    let caesar = Caesar::new(key);
    let mut children = Vec::with_capacity(jobs);

    // iterate over all threads and assign messages to each one
    for index in 0..jobs {
        let chunk = String::from(&text[index * size..(index + 1) * size]);

        children.push(thread::spawn(move || translate(&caesar, chunk)));
    }

    // last job is done on the main thread
    let last = String::from(&text[size * jobs..text.len()]);

    let last = translate(&caesar, last);

    print_results(children, last);
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

fn print_results(children: Vec<thread::JoinHandle<String>>, last: String) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for child in children {
        match child.join() {
            Ok(ans) => {
                write!(&mut handle, "{}", &ans).expect("Failed to write to stdout");
            }
            Err(_) => {
                eprintln!("Threads failed");
                std::process::exit(1);
            }
        }
    }

    writeln!(&mut handle, "{}", &last).expect("Failed to write to stdout");
}
