const BUFFER_SIZE: usize = 32768;

pub fn get(cpus: usize) -> Vec<[u8; BUFFER_SIZE]> {
    let mut bufs: Vec<[u8; BUFFER_SIZE]> = Vec::with_capacity(cpus);

    for _ in 0..cpus {
        let buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        bufs.push(buf);
    }

    bufs
}
