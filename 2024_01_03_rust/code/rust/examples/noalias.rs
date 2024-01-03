#[inline(always)]
fn mat_fma(a: &mut [f32], b: &[f32], c: f32) {
    if (a.len() % 8) != 0 || b.len() != 4 {
        return;
    }

    for (i, a) in a.iter_mut().enumerate() {
        *a += b[i % 4] * c;
    }
}

#[inline(never)]
#[no_mangle]
pub fn noinline_mat_fma(a: &mut [f32], b: &[f32], c: f32) {
    mat_fma(a, b, c);
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn noinline_mat_fma2(a: &mut [f32; 8], b: &[f32; 4], c: f32) {
    mat_fma(a, b, c);
}

#[no_mangle]
pub static MAT_FMA: fn(&mut [f32], &[f32], c: f32) = noinline_mat_fma;

#[no_mangle]
pub static MAT_FMA2: extern "C" fn(&mut [f32; 8], &[f32; 4], c: f32) = noinline_mat_fma2;

#[inline(never)]
fn use_up(a: &mut [f32]) {
    core::hint::black_box(a);
}

fn main() {
    core::hint::black_box(MAT_FMA);
    core::hint::black_box(MAT_FMA2);

    let mut a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let mut b = [16.0, 14.0, 12.0, 10.0, 8.0, 6.0, 4.0, 2.0];

    // Mark the values as modified
    use_up(&mut a);
    use_up(&mut b);

    mat_fma(&mut a[..], &b[..4], 2.0);

    use_up(&mut a);
    use_up(&mut b);

    for i in 0..8 {
        println!("{} {}", a[i], b[i]);
    }
}
