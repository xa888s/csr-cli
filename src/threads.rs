use std::io;
use std::io::Write;
use std::thread;

#[path = "crypt.rs"]
mod crypt;

pub fn run_jobs(message: String, mode: String, key: u8, threads: usize) {
    let size: usize = message.len() / threads;
    let jobs;

    // if the length of the message is less than the thread count, do the job on one thread only
    if message.len() < threads {
        jobs = 0;
    } else {
        jobs = threads;
    }

    let main_thread_result;
    let mut children = Vec::with_capacity(jobs);

    // index of last char in String
    let length = message.len();

    match &mode[..] {
        "encrypt" => {
            for index in 0..jobs {
                let chunk = String::from(&message[index * size..(index + 1) * size]);
                children.push(thread::spawn(move || crypt::encrypt(chunk, key)));
            }
            main_thread_result = crypt::encrypt(String::from(&message[size * jobs..length]), key);
        }
        "decrypt" => {
            for index in 0..jobs {
                let chunk = String::from(&message[index * size..(index + 1) * size]);
                children.push(thread::spawn(move || crypt::decrypt(chunk, key)));
            }
            main_thread_result = crypt::decrypt(String::from(&message[size * jobs..length]), key);
        }
        _ => {
            eprintln!("Mode must be 'encrypt' or 'decrypt'");
            std::process::exit(1);
        }
    }

    let mut stdout = io::stdout();
    stdout.lock();
    for child in children {
        match child.join() {
            Ok(ans) => {
                write!(&mut stdout, "{}", &ans).expect("Failed to write to stdout");
            }
            Err(_) => {
                eprintln!("Threads failed");
                std::process::exit(1);
            }
        }
    }
    writeln!(&mut stdout, "{}", &main_thread_result).expect("Failed to write to stdout");
}
