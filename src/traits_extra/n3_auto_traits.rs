// ==========================================================
// RUST AUTO TRAITS (OIBITs) COMPREHENSIVE GUIDE
// ==========================================================

// Not strictly a trait type but a compiler feature

use std::marker::PhantomData;
use std::rc::Rc;

/// 1. THE BASICS
/// All Auto Traits are Marker Traits (no methods), but not all
/// Marker Traits are Auto Traits (e.g., `Copy` is NOT auto).
///
/// Auto traits are "recursive":
/// - If ALL fields of a struct are `Send`, the struct is `Send`.
/// - If ONE field is NOT `Send`, the struct is NOT `Send`.
fn is_send<T: Send>() {
    println!("This type is Send!");
}
fn is_sync<T: Sync>() {
    println!("This type is Sync!");
}

/// 2. PRIMITIVES & BASE CASES
/// The compiler has hardcoded rules for primitives.
fn primitives_example() {
    // i32, bool, f64, etc., are the "building blocks" that are Send/Sync.
    is_send::<i32>();
    is_sync::<bool>();

    // RAW POINTERS are the "poison". They are NOT Send or Sync by default.
    // This is because the compiler cannot guarantee what they point to.
    // is_send::<*const i32>(); // <-- This would fail to compile!
}

/// 3. AUTOMATIC PROPAGATION (The "Detective" Work)
/// The compiler looks inside your types automatically.
struct SafeContainer {
    id: i32,      // i32 is Send
    name: String, // String is Send (internally handles its pointers safely)
}

struct UnsafeContainer {
    data: i32,
    pointer: *const i32, // This "poisons" the whole struct
}

fn propagation_example() {
    is_send::<SafeContainer>(); // Works
}
