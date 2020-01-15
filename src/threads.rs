use std::io;
use std::io::Write;
use std::thread;

pub use caesar::{Caesar, Kind};

pub fn run_jobs(message: Caesar, key: u8, threads: usize) {
    // index of last char in String
    let length = message.text.len();

    let size: usize = length / threads;

    // if the length of the message is less than the thread count, do the job on one thread only
    let jobs = if length < threads { 0 } else { threads };

    let mut children = Vec::with_capacity(jobs);

    // iterate over all threads and assign messages to each one
    for index in 0..jobs {
        let chunk = Caesar::new(
            String::from(&message.text[index * size..(index + 1) * size]),
            message.kind,
        );

        children.push(thread::spawn(move || chunk.translate(key)));
    }

    // last job is done on the main thread
    let last = Caesar::new(
        String::from(&message.text[size * jobs..length]),
        message.kind,
    );

    let main_thread_result = last.translate(key);

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
    writeln!(&mut handle, "{}", &main_thread_result).expect("Failed to write to stdout");
}
