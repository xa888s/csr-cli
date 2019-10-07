use std::io;
use std::io::Write;
use std::thread;

#[path = "crypt.rs"]
mod crypt;

pub fn run_jobs(message: String, mode: String, key: u8, threads: usize) {
    // index of last char in String
    let length = message.len();

    let size: usize = length / threads;

    // if the length of the message is less than the thread count, do the job on one thread only
    let jobs = if length < threads { 0 } else { threads };

    let main_thread_result;
    let mut children = Vec::with_capacity(jobs);

    let func = match &mode[..] {
        "encrypt" => crypt::encrypt,

        "decrypt" => crypt::decrypt,

        _ => {
            eprintln!("Mode must be 'encrypt' or 'decrypt'");
            std::process::exit(1);
        }
    };

    for index in 0..jobs {
        let chunk = String::from(&message[index * size..(index + 1) * size]);
        children.push(thread::spawn(move || func(chunk, key)));
    }
    main_thread_result = func(String::from(&message[size * jobs..length]), key);

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
