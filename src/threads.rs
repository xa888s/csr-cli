use std::thread;
mod crypt;

pub fn split_jobs(threads: usize, mut message: String) -> Vec<String> {
    // create a vector with a capacity of the number of threads (each job will run on a thread)
    let mut jobs = Vec::with_capacity(threads);
    let size: usize = message.len() / threads;
    let remainder = message.len() % threads;

    // assign remainder of the division as the first member of the vector
    // this can later be used by calling crypt::encrypt(jobs[0], key) on the main thread
    if remainder > 0 {
        let message_initial = message.split_off(remainder);
        jobs.push(message);
        message = message_initial;
    }

    // split message into equal sizes and add to a vector
    for _ in 0..threads {
        let next = message.split_off(size);
        jobs.push(message);
        message = next;
    }
    jobs
}

pub fn run_jobs(mut jobs: Vec<String>, mode: String, key: usize, length: usize) {
    // run first job on the main thread and remove it from the vector
    let main_thread_result = crypt::encrypt(&jobs[0], key);
    jobs.remove(0);
    let mut children = Vec::with_capacity(jobs.len());

    if mode == "encrypt" {
        for job in jobs {
            children.push(thread::spawn(move || {
                crypt::encrypt(&job, key)
            }));
        }
    } else if mode == "decrypt" {
        for job in jobs {
            children.push(thread::spawn(move || {
                crypt::decrypt(&job, key)
            }));
        }
    } else {
        eprintln!("Mode must be 'encrypt' or 'decrypt'");
        std::process::exit(1);
    }
    let mut result = String::with_capacity(length);
    result += &main_thread_result;
    for child in children {
        match child.join() {
            Ok(ans) => {
                result += &ans;
            }
            Err(_) => {
                eprintln!("Threads failed");
                std::process::exit(1);
            }
        }
    }
    println!("{}", result);
}
