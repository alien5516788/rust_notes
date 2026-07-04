//! Variance: How the subtyping of type arguments affects the subtyping of the generic type.
//! In Rust, subtyping is almost exclusively related to lifetimes ('a: 'b).

use std::cell::Cell;

/// 1. COVARIANCE (The Default)
/// If 'a outlives 'b, then Wrapper<'a> is a subtype of Wrapper<'b>.
/// Most immutable types are covariant.
struct Covariant<'a> {
    _data: &'a str, // References are covariant over 'a
}

/// 2. INVARIANCE (Mutable Access)
/// Invariance means there is NO subtyping relationship. Wrapper<'a> must match Wrapper<'b> exactly.
/// This is required for safety when you have "Interior Mutability."
struct Invariant<'a> {
    _data: Cell<&'a str>, // Cell/RefCell make the type invariant over 'a
}

/// 3. CONTRAVARIANCE (Function Arguments)
/// Reverses the relationship. Rare in Rust, seen in function pointers.
/// A function that handles a short lifetime can be used where one handling a long one is expected.
struct Contravariant<'a> {
    _func: fn(&'a str),
}

/*
    WHY INVARIANCE MATTERS:
    If &mut T were covariant, you could do this:

    let mut short_lived = String::from("short");
    let mut long_lived: &'static str = "static";

    let mut_ref: &mut &'static str = &mut long_lived;
    let covariant_ref: &mut &str = mut_ref; // If this were allowed (covariance)...
    *covariant_ref = &short_lived;          // ...you just stored a short life in a static variable!
    // Result: Use-after-free. Rust prevents this by making &mut T invariant.
*/
