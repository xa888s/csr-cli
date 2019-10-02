use std::thread;
use std::io;

#[path = "crypt.rs"]
mod crypt;

pub fn run_jobs(message: String, mode: String, key: usize, threads: usize) {
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
    let length = message.len() - 1;

    match &mode[..] {
        "encrypt" => {
            for index in 0..jobs {
                children.push(thread::spawn(move || {
                    let chunk = &message[index*size..((index+1)*size)-1]; 
                    crypt::encrypt(&chunk, &key)
                }));
            }
            main_thread_result = crypt::encrypt(&message[size*jobs..length], &key);
        }
        "decrypt" => {
            for index in 0..jobs {
                children.push(thread::spawn(move || {
                    let chunk = &message[index*size..((index+1)*size)-1]; 
                    crypt::decrypt(&chunk, &key)
                }));
            }
            main_thread_result = crypt::decrypt(&message[size*jobs..length], &key);
        }
        _ => {
            eprintln!("Mode must be 'encrypt' or 'decrypt'");
            std::process::exit(1);
        }
    }

    let stdout = io::stdout();
    stdout.lock();
    for child in children {
        match child.join() {
            Ok(ans) => {
                print!("{}", &ans);
            }
            Err(_) => {
                eprintln!("Threads failed");
                std::process::exit(1);
            }
        }
    }
    println!("{}", &main_thread_result);
}
