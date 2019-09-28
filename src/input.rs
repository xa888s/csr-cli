use std::io::{self, Read};

pub fn get_input() -> String {
    let mut message = String::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();

    while let Ok(n_bytes) = stdin.read_to_string(&mut line) {
        if n_bytes == 0 { break }
        message = message + &line;
        line.clear();
    }

    message
}
