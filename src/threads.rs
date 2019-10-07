use std::io;
use std::io::Write;
use std::thread;

#[path = "message.rs"]
mod message;
pub use message::{Mode, Message};

pub fn run_jobs(message: Message, key: u8, threads: usize) {
    // index of last char in String
    let length = message.text.len();

    let size: usize = length / threads;

    // if the length of the message is less than the thread count, do the job on one thread only
    let jobs = if length < threads { 0 } else { threads };

    let main_thread_result;
    let mut children = Vec::with_capacity(jobs);

    match message.mode {
        Mode::Encrypt => {
            for index in 0..jobs {
                let chunk = Message::new(String::from(&message.text[index * size..(index + 1) * size]));
                children.push(thread::spawn(move || chunk.encrypt(key)));
            }
            let last = Message::new(String::from(&message.text[size * jobs..length]));
            main_thread_result = last.encrypt(key);
        }

        Mode::Decrypt => {
            for index in 0..jobs {
                let chunk = Message::new(String::from(&message.text[index * size..(index + 1) * size]));
                children.push(thread::spawn(move || chunk.decrypt(key)));
            }
            let last = Message::new(String::from(&message.text[size * jobs..length]));
            main_thread_result = last.decrypt(key);
        }
    }

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
