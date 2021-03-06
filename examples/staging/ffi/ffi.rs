extern crate num;

use num::complex::{Complex,Complex32};

// this extern block links to the libm library
#[link(name = "m")]
extern {
    // this is a foreign function
    // that computes the square root of a single precision complex number
    fn csqrtf(z: Complex32) -> Complex32;
}

fn main() {
    // z = -1 + 0i
    let z = Complex::new(-1.0f32, 0.0);

    // calling a foreign function is an unsafe operation
    let z_sqrt = unsafe {
        csqrtf(z)
    };

    println!("the square root of {} is {}", z, z_sqrt);
}
