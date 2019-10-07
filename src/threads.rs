use std::io;
use std::io::Write;
use std::thread;

#[path = "message.rs"]
mod message;
pub use message::{Message, Mode};

pub fn run_jobs(message: Message, mode: Mode, key: u8, threads: usize) {
    // index of last char in String
    let length = message.text.len();

    let size: usize = length / threads;

    // if the length of the message is less than the thread count, do the job on one thread only
    let jobs = if length < threads { 0 } else { threads };

    let main_thread_result;
    let mut children = Vec::with_capacity(jobs);

    // choose which function to use
    let func: fn(Message, u8) -> String = match mode {
        Mode::Encrypt => Message::encrypt,

        Mode::Decrypt => Message::decrypt,
    };

    // iterate over all threads and assign messages to each one
    for index in 0..jobs {
        let chunk = Message::new(String::from(
            &message.text[index * size..(index + 1) * size],
        ));
        children.push(thread::spawn(move || func(chunk, key)));
    }

    // last job is done on the main thread
    let last = Message::new(String::from(&message.text[size * jobs..length]));
    main_thread_result = func(last, key);

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
