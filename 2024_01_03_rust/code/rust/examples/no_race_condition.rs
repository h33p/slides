use std::sync::atomic::{AtomicU32, Ordering};
use std::thread::spawn;

static SHARED_VARIABLE: AtomicU32 = AtomicU32::new(0);

fn increment() {
    for _ in 0..10000 {
        SHARED_VARIABLE.fetch_add(1, Ordering::Relaxed);
    }
}

fn main() {
    let thread1 = spawn(increment);
    let thread2 = spawn(increment);

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!(
        "Expected: 20000, Actual: {}",
        SHARED_VARIABLE.load(Ordering::Relaxed)
    );
}
