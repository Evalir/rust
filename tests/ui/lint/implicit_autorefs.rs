//@ check-pass
//@ run-rustfix

#![allow(dead_code)] // For the rustfix-ed code.

use std::mem::ManuallyDrop;
use std::ops::Deref;

unsafe fn test_const(ptr: *const [u8]) {
    let _ = (*ptr)[..16];
    //~^ WARN implicit autoref
}

struct Test {
    field: [u8],
}

unsafe fn test_field(ptr: *const Test) -> *const [u8] {
    let l = (*ptr).field.len();
    //~^ WARN implicit autoref

    &raw const (*ptr).field[..l - 1]
    //~^ WARN implicit autoref
}

unsafe fn test_builtin_index(a: *mut [String]) {
    _ = (*a)[0].len();
    //~^ WARN implicit autoref

    _ = (*a)[..1][0].len();
    //~^ WARN implicit autoref
    //~^^ WARN implicit autoref
}

unsafe fn test_overloaded_deref_const(ptr: *const ManuallyDrop<Test>) {
    let _ = (*ptr).field;
    //~^ WARN implicit autoref
    let _ = &raw const (*ptr).field;
    //~^ WARN implicit autoref
}

unsafe fn test_overloaded_deref_mut(ptr: *mut ManuallyDrop<Test>) {
    let _ = (*ptr).field;
    //~^ WARN implicit autoref
}

unsafe fn test_double_overloaded_deref_const(ptr: *const ManuallyDrop<ManuallyDrop<Test>>) {
    let _ = (*ptr).field;
    //~^ WARN implicit autoref
}

unsafe fn test_manually_overloaded_deref() {
    struct W<T>(T);

    impl<T> Deref for W<T> {
        type Target = T;
        fn deref(&self) -> &T { &self.0 }
    }

    let w: W<i32> = W(5);
    let w = &raw const w;
    let _p: *const i32 = &raw const **w;
    //~^ WARN implicit autoref
}

struct Test2 {
    // Derefs to `[u8]`.
    field: &'static [u8]
}

fn test_more_manual_deref(ptr: *const Test2) -> usize {
    unsafe { (*ptr).field.len() }
    //~^ WARN implicit autoref
}

unsafe fn test_no_attr(ptr: *mut ManuallyDrop<u8>) {
    ptr.write(ManuallyDrop::new(1)); // Should not warn, as `ManuallyDrop::write` is not
                                     // annotated with `#[rustc_no_implicit_auto_ref]`
}

unsafe fn test_vec_get(ptr: *mut Vec<u8>) {
    let _ = (*ptr).get(0);
    //~^ WARN implicit autoref
    let _ = (*ptr).get_unchecked(0);
    //~^ WARN implicit autoref
    let _ = (*ptr).get_mut(0);
    //~^ WARN implicit autoref
    let _ = (*ptr).get_unchecked_mut(0);
    //~^ WARN implicit autoref
}

unsafe fn test_string(ptr: *mut String) {
    let _ = (*ptr).len();
    //~^ WARN implicit autoref
    let _ = (*ptr).is_empty();
    //~^ WARN implicit autoref
}

fn main() {}
