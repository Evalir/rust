//@ build-fail

#![feature(repr_simd)]

struct E;

#[repr(simd)]
struct S<T>([T; 4]);

fn main() {
    let _v: Option<S<E>> = None;
}

//~? ERROR monomorphising SIMD type `S<E>` with a non-primitive-scalar (integer/float/pointer) element type `E`
