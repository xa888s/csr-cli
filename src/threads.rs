use std::thread;
use std::io;

#[path = "crypt.rs"]
mod crypt;

pub fn split_jobs(threads: usize, mut message: String) -> Vec<String> {
    // create a vector with a capacity of the number of threads (each job will run on a thread)
    let mut jobs = Vec::with_capacity(threads);
    let size: usize = message.len() / threads;
    let remainder = message.len() % threads;

    // split message into equal sizes and add to a vector
    for _ in 0..threads {
        let next = message.split_off(size);
        jobs.push(message);
        message = next;
    }
    
    // assign remainder of the division as the last member of the vector
    // this can later be used by calling crypt::encrypt(jobs[jobs.len()-1], key) on the main thread
    if remainder > 0 {
        jobs.push(message);
    }
    jobs
}

pub fn run_jobs(mut jobs: Vec<String>, mode: String, key: usize) {
    let mut children = Vec::with_capacity(jobs.len());
    let main_thread_result;

    match &mode[..] {
        "encrypt" => {
            main_thread_result = crypt::encrypt(&jobs[jobs.len()-1], key);
            jobs.pop();
            for job in jobs {
                children.push(thread::spawn(move || {
                    crypt::encrypt(&job, key)
                }));
            }
        }
        "decrypt" => {
            main_thread_result = crypt::decrypt(&jobs[jobs.len()-1], key);
            jobs.pop();
            for job in jobs {
                children.push(thread::spawn(move || {
                    crypt::decrypt(&job, key)
                }));
            }
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
    print!("{}", &main_thread_result);
    println!("");
}
