// ==========================================================
// NEGATIVE TRAIT IMPL (Nightly only)
// ==========================================================

// Negative trait implementations allow explicitly marking a type
// as NOT implementing an auto trait.
//
// Primary purpose:
// - Provide fine-grained control over auto traits
// Use cases:
// - Preventing Send/Sync on specific types
// Note: Nightly feature only
// #![feature(negative_impls)]
// auto trait MyAutoTrait {}
// impl !MyAutoTrait for i32 {}

// ==========================================================
// SPECIALIZATION (Nightly only)
// ==========================================================

// Specialization allows more specific implementations to override
// more general ones.
//
// Primary purpose:
// - Provide default behavior while allowing optimized overrides
// Use cases:
// - Library authors optimizing trait behavior
// Note: Nightly feature only
// #![feature(specialization)]
// trait Example {
//     fn value(&self) -> i32;
// }
// default impl<T> Example for T {
//     fn value(&self) -> i32 {
//         0
//     }
// }
// impl Example for i32 {
//     fn value(&self) -> i32 {
//         *self
//     }
// }
