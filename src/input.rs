use std::io::{self, Write};

pub fn get_input(message: &str) -> String {
    let mut buf = String::new();
    print!("{} ", message);
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut buf).expect("Couldn't read stdin!");
    buf
}
