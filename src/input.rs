use std::io::{self, Read};

pub fn get() -> io::Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buf)?;
    Ok(buf)
}
