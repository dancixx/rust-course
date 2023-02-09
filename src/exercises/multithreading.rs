use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

pub fn run_async_multithread() {
    let mut handles = vec![];
    let count = 1_000_000usize;
    let data = Arc::new(Mutex::new(0));

    let start = std::time::Instant::now();
    for _ in 0..10 {
        let data = Arc::clone(&data);
        let handle = std::thread::spawn(move || {
            for _ in 0..(count / 10) {
                let mut data = data.lock().unwrap();
                *data += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let end = std::time::Instant::now();
    println!("Time taken: {:?}", end - start);
    println!("{}", data.lock().unwrap());
}

pub fn run_sync() {
    let count = 1_000_000_000usize;
    let mut data = 0;

    let start = std::time::Instant::now();
    for _ in 0..count {
        data += 1;
    }
    let end = std::time::Instant::now();
    println!("Time taken: {:?}", end - start);
    println!("{}", data);
}

pub fn run_local_storage() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let count = 1_000_000_000usize;

    let start = std::time::Instant::now();
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut local_counter = 0;
            for _ in 0..(count / 10) {
                local_counter += 1;
            }
            let mut num = counter.lock().unwrap();
            *num += local_counter;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let end = std::time::Instant::now();
    println!("Time taken: {:?}", end - start);

    println!("Result: {}", *counter.lock().unwrap());
}

pub fn run_atomic() {
    let counter = Arc::new(AtomicUsize::new(0));
    let count = 1_000_000_000usize;
    let mut handles = vec![];

    let start = std::time::Instant::now();
    for _ in 0..10 {
        let counter = counter.clone();
        let handle = std::thread::spawn(move || {
            for _ in 0..(count / 10) {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let end = std::time::Instant::now();
    println!("Time taken: {:?}", end - start);

    println!("Result: {}", counter.load(Ordering::SeqCst));
}

// Declare a thread-local key for our counter
thread_local! {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
}

pub fn run_tls_atomic() {
    let mut handles = vec![];
    let count = 1_000_000_000usize;

    let start = std::time::Instant::now();
    for _ in 0..10 {
        let handle = std::thread::spawn(move || {
            // Each thread gets its own copy of the counter
            for _ in 0..(count / 10) {
                COUNTER.with(|c| c.fetch_add(1, Ordering::SeqCst));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = COUNTER.with(|c| c.load(Ordering::SeqCst));
    let end = std::time::Instant::now();
    println!("Time taken: {:?}", end - start);
    println!("Result: {}", result);
}
