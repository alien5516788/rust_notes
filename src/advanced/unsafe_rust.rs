//! Unsafe Rust: A sub-language that lets you bypass the borrow checker.
//! 'unsafe' does not turn off the borrow checker; it only gives you 5 specific superpowers.

fn main() {
    // SUPERPOWER 1: Dereference a raw pointer
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        *r2 = 10;
    }

    // SUPERPOWER 2: Call an unsafe function or method
    unsafe {
        dangerous_function();
    }

    // SUPERPOWER 3: Access or modify a mutable static variable
    unsafe {
        println!("Global: {}", GLOBAL_VAR);
    }

    // SUPERPOWER 4: Implement an unsafe trait
    unsafe impl MyUnsafeTrait for i32 {}

    // SUPERPOWER 5: Access fields of a union (primarily for FFI)
}

static mut GLOBAL_VAR: i32 = 42;

unsafe fn dangerous_function() {
    // The caller must ensure memory safety requirements are met
}

unsafe trait MyUnsafeTrait {}

/*
    THE UNSAFE CONTRACT:
    When you use 'unsafe', you are promising the compiler:
    - No Null pointers (unless ignored).
    - No Data Races.
    - No Use-after-free.
    - No Out-of-bounds access.
    The compiler cannot help you here; you must verify these yourself.
*/
