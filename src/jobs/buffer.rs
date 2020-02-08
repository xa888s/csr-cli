use num_cpus as cpus;

pub fn get() -> Vec<[u8; 8192]> {
    let cpus = cpus::get();
    let mut bufs: Vec<[u8; 8192]> = Vec::with_capacity(cpus);

    for _ in 0..cpus {
        let buf: [u8; 8192] = [97; 8192];
        bufs.push(buf);
    }

    bufs
}
