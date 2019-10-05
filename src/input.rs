use std::io::{self, Read};

pub fn get_input() -> String {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buf).expect("Failed to read stdin");
    buf
}
