//@aux-build:macro_rules.rs

#![warn(clippy::missing_transmute_annotations)]
#![allow(clippy::let_with_type_underscore)]

#[macro_use]
extern crate macro_rules;

macro_rules! local_bad_transmute {
    ($e:expr) => {
        std::mem::transmute($e)
        //~^ ERROR: transmute used without annotations
    };
}

fn bar(x: i32) -> i32 {
    x
}

unsafe fn foo1() -> i32 {
    unsafe {
        // Should not warn!
        std::mem::transmute([1u16, 2u16])
    }
}

// Should not warn!
const _: i32 = unsafe { std::mem::transmute([1u16, 2u16]) };

#[repr(i32)]
enum Foo {
    A = 0,
}

unsafe fn foo2() -> i32 {
    unsafe {
        let mut i: i32 = 0;
        i = std::mem::transmute([1u16, 2u16]);
        //~^ ERROR: transmute used without annotations
        i = std::mem::transmute::<_, _>([1u16, 2u16]);
        //~^ ERROR: transmute used without annotations
        i = std::mem::transmute::<_, i32>([1u16, 2u16]);
        //~^ ERROR: transmute used without annotations
        i = std::mem::transmute::<[u16; 2], _>([1u16, 2u16]);
        //~^ ERROR: transmute used without annotations

        let x: i32 = bar(std::mem::transmute::<[u16; 2], _>([1u16, 2u16]));
        //~^ ERROR: transmute used without annotations
        bar(std::mem::transmute::<[u16; 2], _>([1u16, 2u16]));
        //~^ ERROR: transmute used without annotations

        i = local_bad_transmute!([1u16, 2u16]);

        // Should not warn.
        i = bad_transmute!([1u16, 2u16]);

        i = std::mem::transmute([0i16, 0i16]);
        //~^ ERROR: transmute used without annotations

        i = std::mem::transmute(Foo::A);
        //~^ ERROR: transmute used without annotations

        i
    }
}

fn main() {
    let x: _ = unsafe { std::mem::transmute::<_, i32>([1u16, 2u16]) };
    //~^ ERROR: transmute used without annotations
    unsafe {
        let x: _ = std::mem::transmute::<_, i32>([1u16, 2u16]);
        //~^ ERROR: transmute used without annotations

        // Should not warn.
        std::mem::transmute::<[u16; 2], i32>([1u16, 2u16]);
        let x = std::mem::transmute::<[u16; 2], i32>([1u16, 2u16]);
        let x: i32 = std::mem::transmute::<[u16; 2], _>([1u16, 2u16]);
        let x: i32 = std::mem::transmute::<_, i32>([1u16, 2u16]);
        let x: i32 = std::mem::transmute([1u16, 2u16]);
    }
    let x: i32 = unsafe { std::mem::transmute([1u16, 2u16]) };
}
