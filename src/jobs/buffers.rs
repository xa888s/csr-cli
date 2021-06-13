use crossbeam::{
    channel::{bounded, Sender},
    thread::{self, ScopedJoinHandle},
};
use csr::Caesar;
use num_cpus as cpus;
use std::io::{self, prelude::*, BufReader};
use std::mem::MaybeUninit;
use std::sync::Mutex;

const BUFFER_SIZE: usize = 32768;

pub fn run<R: Read>(
    translate: impl Fn(Caesar, &mut [u8]) + Send + Copy,
    caesar: Caesar,
    reader: &mut BufReader<R>,
) -> Result<(), io::Error> {
    let cpus = cpus::get();
    let numbered_bufs: Vec<Mutex<[MaybeUninit<u8>; BUFFER_SIZE]>> = (0..cpus)
        // Safety: refer to:
        // https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        .map(|_| Mutex::new(unsafe { MaybeUninit::uninit().assume_init() }))
        .collect();

    thread::scope(|s| {
        let (last_sender, mut prev_receiver) = bounded(0);

        let (txs, handles): (Vec<_>, Vec<_>) = numbered_bufs
            .iter()
            .enumerate()
            .map(|(i, buf)| {
                // this is the channel between this thread and the main thread
                let (reader_tx, reader_rx) = bounded(0);

                // this is the channel between this thread and the next thread in the chain (our
                // notifier)
                let (our_tx, prev_receiver) = if i != numbered_bufs.len() - 1 {
                    let (our_tx, next_rx) = bounded(0);
                    (our_tx, std::mem::replace(&mut prev_receiver, next_rx))
                } else {
                    // TODO: remove the second clone somehow
                    (last_sender.clone(), prev_receiver.clone())
                };

                let handle = s.spawn(move |_| {
                    let stdout = io::stdout();

                    while let Ok(bytes_read) = reader_rx.recv() {
                        let mut lock = buf
                            .lock()
                            .expect("no thread should poison the mutex (by panicking)");

                        // Safety: we are only reading parts that are promised to be initialized by
                        // the main thread, so this should be safe.
                        let buf = unsafe {
                            &mut std::slice::from_raw_parts_mut(
                                lock.as_mut_ptr() as *mut u8,
                                lock.len(),
                            )[..bytes_read]
                        };

                        // do our heavy work
                        translate(caesar, buf);

                        // wait until the previous thread signals us to start
                        if let Ok(()) = prev_receiver.recv() {
                            stdout.lock().write_all(buf).expect("write shouldn't fail");

                            // drop the mutex guard on the buffer so main can read into it, since after
                            // this we are done with it.
                            drop(lock);

                            // here we have finished our printing, so we will block on sending a
                            // message to the next thread in the chain so they can print their
                            // work if there is still work left (checked by the bytes_read).
                            if bytes_read == BUFFER_SIZE {
                                our_tx.send(()).unwrap();
                            } else {
                                break;
                            }
                        }
                    }
                });

                (reader_tx, handle)
            })
            .unzip();

        let mut iter = txs.iter().zip(numbered_bufs.iter());

        // we are special casing the first buffer to start our channel chain
        let (tx, buf) = iter.next().expect("There must be atleast 1 worker thread");

        let bytes_read = read_into_buf(tx, buf, reader);

        // starting the circle
        last_sender.send(()).unwrap();

        if bytes_read != BUFFER_SIZE {
            close_threads(txs, handles);
            return;
        }

        'outer: loop {
            for (tx, buf) in iter {
                let bytes_read = read_into_buf(tx, buf, reader);

                if bytes_read != BUFFER_SIZE {
                    break 'outer;
                }
            }

            iter = txs.iter().zip(numbered_bufs.iter());
        }

        close_threads(txs, handles);
    })
    .expect("thread scope shouldn't fail");

    Ok(())
}

fn read_into_buf<R: Read>(
    tx: &Sender<usize>,
    buf: &Mutex<[MaybeUninit<u8>; BUFFER_SIZE]>,
    reader: &mut BufReader<R>,
) -> usize {
    let lock = &mut *buf
        .lock()
        .expect("no thread should poison the mutex (by panicking)");

    // Safety: we are only writing into it, so this should be safe.
    let buf = unsafe { std::slice::from_raw_parts_mut(lock.as_mut_ptr() as *mut u8, lock.len()) };

    let bytes_read = reader.read(buf).expect("reader shouldn't panic");
    tx.send(bytes_read).expect("sender shouldn't panic");
    bytes_read
}

fn close_threads(txs: Vec<Sender<usize>>, handles: Vec<ScopedJoinHandle<()>>) {
    // we need to drop our channels (close them) here so the threads know to stop
    drop(txs);

    for handle in handles {
        // this should never fail
        handle.join().expect("no thread should panic");
    }
}
