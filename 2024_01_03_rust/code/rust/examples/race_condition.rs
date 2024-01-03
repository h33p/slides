use std::thread::spawn;

static mut SHARED_VARIABLE: u32 = 0;

fn increment() {
    for _ in 0..10000 {
        unsafe { SHARED_VARIABLE += 1 };
    }
}

fn main() {
    let thread1 = spawn(increment);
    let thread2 = spawn(increment);

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Expected: 20000, Actual: {}", unsafe { SHARED_VARIABLE });
}
