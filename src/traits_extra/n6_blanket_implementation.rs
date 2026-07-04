// ==========================================================
// BLANKET IMPLEMENTATION
// ==========================================================

// Blanket implementations apply a trait to any type that satisfies
// a set of bounds.
// This is not a new trait type, just a way to implement traits for many types
//
// Primary purpose:
// - Reduce boilerplate
// - Automatically implement traits for many types
// Use cases:
// - std::fmt::Debug, Copy, Clone, etc.
trait MyTrait {
    fn do_it(&self);
}

// This implementation covers all types that implement std::fmt::Debug
// So kinda like a blanket that covers all Debug types
impl<T: std::fmt::Debug> MyTrait for T {
    fn do_it(&self) {
        println!("{:?}", self); // Implemented for all Debug types
    }
}
