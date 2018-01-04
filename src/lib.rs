#![no_std]

//! The pointer-sized floating-point type.
//! 
//! The size of this primitive is how many bytes it takes to reference any location in memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.
//!
//! As Rust only supports [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) and  [`f64`](https://doc.rust-lang.org/std/primitive.f64.html), `fsize` is only provided for 32 and 64 bit targets.

include!(concat!(env!("OUT_DIR"), "/fsize.rs"));