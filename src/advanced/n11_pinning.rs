//! Pinning: Ensuring a value does not move in memory.
//! This is required for self-referential structs where a pointer points to another field in the same struct.

use std::marker::PhantomPinned;
use std::pin::Pin;

/// A self-referential struct
struct SelfReferential {
    data: String,
    pointer_to_data: *const String,
    _marker: PhantomPinned, // This makes the type !Unpin
}

impl SelfReferential {
    fn new(text: &str) -> Self {
        Self {
            data: String::from(text),
            pointer_to_data: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.data;
        let this = unsafe { self.get_unchecked_mut() };
        this.pointer_to_data = self_ptr;
    }
}

/*
    THE PINNING RULES:
    1. Unpin (Default): Most types (i32, String) can be moved safely. Pin<P> does nothing for them.
    2. !Unpin: Types that are unsafe to move (like Futures or our struct above).
    3. Stack Pinning: Uses `pin_utils::pin_mut!` (unsafe if done manually).
    4. Heap Pinning: `Box::pin(value)` is the easiest way to pin a !Unpin type.
*/
