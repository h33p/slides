use std::thread::spawn;

#[inline(never)]
fn increment(shared_var: &mut u32) {
    for _ in 0..10000 {
        *shared_var += 1;
    }
}

fn main() {
    let mut shared_var = 0u32;
    let shared_var_addr = &mut shared_var as *mut u32 as usize;

    let thread1 = spawn(move || increment(unsafe { &mut *(shared_var_addr as *mut u32) }));
    let thread2 = spawn(move || increment(unsafe { &mut *(shared_var_addr as *mut u32) }));

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Expected: 20000, Actual: {}", shared_var);
}
