#[inline(never)]
fn example(a: &mut usize, b: &mut usize, c: usize) {
    *a += c;
    *b *= c;
    *a += c;
}

fn main() {
    let mut a = 5;
    let mut b = 5;
    // Make sure example is not optimized based on the context
    let val = core::hint::black_box(10);

    example(&mut a, &mut b, val);
}
