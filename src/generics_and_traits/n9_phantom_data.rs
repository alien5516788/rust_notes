fn phantom_data() {
    /*
        - PhantomData<T> is a zero-sized marker used to "fake" the usage of a type parameter T
        - The compiler requires all generic parameters to be used in a struct field
        - Use cases:
            - Informing the compiler about ownership/lifetimes in low-level code (pointers)
            - Controlling Variance (how lifetimes/types subtype)
            - Implementing the "Type-State" pattern (compile-time state machines)
        - Key Characteristic: It occupies 0 bytes at runtime (it's purely for compile-time)
        - Note: Compiler removes the PhantomData field after lifetime verification at compile time
    */

    use std::marker::PhantomData;

    // Simple idea is compiler strips any field with `PhantomData` at compile time (after any verification is done)

    struct Person<T> {
        tempField: PhantomData<T>,
    }

    let p = Person::<i32> {
        tempField: PhantomData,
    };

    // println!("{:?}", p);

    // Example 1: Handling Unused Type Parameters
    /*
        - Error E0392 occurs if you define `struct MyVec<T> { ptr: *const T }`
          because the compiler doesn't know how T is used.
        - PhantomData tells the compiler: "Pretend this struct owns T."
    */
    struct MyVec<T> {
        ptr: *const T,
        len: usize,
        marker: PhantomData<T>, // Satisfies compiler that T is "used"
    }

    // Example 2: Tying to a Lifetime
    /*
        - Useful for FFI or raw pointers where you want to ensure the struct
          doesn't outlive a specific reference, even if you don't store the reference.
    */
    struct ScopedPointer<'a, T> {
        ptr: *const T,
        lifetime: PhantomData<&'a T>, // Ties struct to lifetime 'a
    }

    // Example 3: Type-State Pattern
    /*
        - Use different marker structs to represent states.
        - No memory is used for these states at runtime.
    */
    struct Unpaid;
    struct Paid;

    struct Invoice<Status> {
        id: u32,
        state: PhantomData<Status>,
    }

    impl Invoice<Unpaid> {
        fn new(id: u32) -> Invoice<Unpaid> {
            Invoice {
                id,
                state: PhantomData,
            }
        }

        fn pay(self) -> Invoice<Paid> {
            Invoice {
                id: self.id,
                state: PhantomData,
            }
        }
    }

    let inv = Invoice::new(101);
    let paid_inv = inv.pay(); // Transition state at compile-time

    // Example 4: Controlling Variance
    /*
        - Variance determines if you can swap a subtype (long lifetime) for a supertype (short lifetime).
        - PhantomData<T> is Covariant (flexible).
        - PhantomData<fn(T) -> T> is Invariant (strict/exact match).
    */
    struct InvariantType<T> {
        marker: PhantomData<fn(T) -> T>,
    }

    // Example 5: Drop Checking
    /*
        - If a struct manages raw memory, PhantomData<T> tells the "Drop Checker"
          that when this struct is dropped, it might also drop instances of T.
    */
    struct CustomBox<T> {
        ptr: *mut T,
        marker: PhantomData<T>,
    }
}

fn why_phantom_data_is_useful() {
    /*
        - Problem: A generic <T> is more than just a label; it defines a "Safety Contract."
        - If Rust allowed unused generics, it wouldn't know how to handle:
            1. Drop Checking (Ownership)
            2. Thread Safety (Send/Sync)
            3. Variance (Lifetime flexibility)
        - PhantomData is the "Explicit Answer" to these three safety questions.
    */

    use std::marker::PhantomData;

    // Reason 1: The "Drop" Question (Ownership)
    /*
        - Does this struct "own" T? If so, the Drop Checker must ensure T
          is valid when the struct is destroyed.
        - PhantomData<T> tells the compiler: "I own a T, so check its validity for dropping."
    */
    struct MyBox<T> {
        ptr: *mut T,
        _marker: PhantomData<T>,
    }

    // Reason 2: The "Thread Safety" Question (Send/Sync)
    /*
        - Rust automatically implements Send/Sync based on a struct's fields.
        - If <T> is unused, the compiler doesn't know if the struct is thread-safe.
        - If a user puts a non-Send type (like Rc) into <T>, PhantomData
          ensures your struct also becomes non-Send, preventing data races.
    */
    struct ThreadSafeWrapper<T> {
        id: u64,
        _marker: PhantomData<T>,
    }

    // Reason 3: The "Variance" Question (Lifetime Flexibility)
    /*
        - This determines if you can swap a long lifetime for a short one.
        - Reference (&T) is Covariant (flexible).
        - Mutable Reference (&mut T) is Invariant (strict).
        - PhantomData allows you to choose which behavior you want for your generic.
    */
    struct Flexible<'a> {
        _marker: PhantomData<&'a i32>, // Covariant: Can swap 'static for 'a
    }

    struct Strict<'a> {
        _marker: PhantomData<*mut &'a i32>, // Invariant: Must be exactly 'a
    }

    // Reason 4: Zero-Cost Abstractions (Logic without Memory)
    /*
        - Allows "Type-States" or "Ghost Units" (like USD vs EUR)
          without the runtime overhead of wrapping values.
    */
    struct SecretLabel;
    struct Data<Label> {
        value: i32,
        _marker: PhantomData<Label>, // 0 bytes at runtime
    }
}

fn an_example() {
    struct Draft;
    struct Signed;

    struct Email<State> {
        content: String,
        _state: std::marker::PhantomData<State>,
    }

    impl Email<Draft> {
        fn sign(self) -> Email<Signed> {
            Email {
                content: self.content,
                _state: std::marker::PhantomData,
            }
        }
    }

    impl Email<Signed> {
        fn send(self) { /* ... */
        }
    }

    // This works:
    // email.sign().send();

    // This fails at COMPILE time:
    // email.send(); // ❌ Method 'send' not found for Email<Draft>
}
