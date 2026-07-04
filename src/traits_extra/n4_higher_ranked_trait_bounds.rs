fn higher_ranked_trait_bounds() {
    /*
        - Higher-Ranked Trait Bounds (HRTBs) use the `for<'a>` syntax.
        - Problem: Standard generics `T: Trait<'a>` require the lifetime `'a` to be
          fixed *before* the function is called.
        - Solution: HRTBs allow a trait bound to be generic over "all possible lifetimes."
        - Most common use: Passing closures that take references as arguments.
    */

    // --- The Problem: The "Too Short" Lifetime ---
    /*
        If we try to write a function that takes a closure, and that closure
        takes a reference, the compiler needs to know how long that reference lives.
    */

    // This looks correct but often fails in complex scenarios:
    // fn call_with_ref<'a, F>(closure: F) where F: Fn(&'a i32) { ... }

    // --- The Solution: HRTB (`for<'a>`) ---

    // Example 1: Basic HRTB Syntax
    /*
        - `for<'a> F: Fn(&'a i32)`
        - This reads: "For all lifetimes 'a that might exist, F must implement Fn(&'a i32)"
    */
    // This function only accepts clousures that can work with a parameter of any lifetime
    fn apply_to_local<F>(func: F)
    where
        for<'a> F: Fn(&'a i32),
    {
        let local_val = 10;
        func(&local_val); // The closure works even on this very short local lifetime
    }

    let my_closure = |x: &i32| println!("Value is: {}", x);
    apply_to_local(my_closure);

    let my_closure2 = |x: &'static i32| println!("Value is: {}", x);
    // apply_to_local(my_closure2);
    // This will not compile becuase closure only works for 'static lifetimes, not for all lifetimes

    // Example 2: Why we can't use standard generics here
    /*
        If we defined it as: `fn apply<'a, F: Fn(&'a i32)>(...)`
        The caller decides what 'a is. But if the caller decides 'a is 'static,
        we can't pass a reference to `local_val` because `local_val` doesn't live forever.
        HRTB solves this by letting the function itself provide the lifetime.
    */

    // Example 3: Complex HRTBs with Traits
    /*
        You can use HRTBs for your own traits, not just closures.
    */
    trait Checker<T> {
        fn check(&self, item: T);
    }

    struct Inspector;
    impl<'a> Checker<&'a str> for Inspector {
        fn check(&self, item: &'a str) {
            println!("Inspecting: {}", item);
        }
    }

    fn validate_data<C>(checker: C)
    where
        for<'a> C: Checker<&'a str>,
    {
        let s = String::from("Internal Data");
        checker.check(&s);
    }

    validate_data(Inspector);

    // Example 4: Implicit HRTBs
    /*
        Rust has "Lifetime Elision" for function signatures.
        `fn foo(f: impl Fn(&i32))` is actually sugar for an HRTB.
    */
    fn implicit_hrtb(f: impl Fn(&i32)) {
        let x = 5;
        f(&x);
    }
}
fn multiple_hrtbs() {
    /*
        - You can define multiple lifetimes in a single HRTB: for<'a, 'b>
        - This is necessary when a closure/trait takes multiple references.
        - The compiler ensures the bound holds for ANY combination of 'a and 'b.
    */

    // Example 1: Multi-lifetime Closure
    /*
        - Here, the closure must be able to accept two references of
          potentially different lifetimes.
    */
    fn compare_lengths<F>(func: F)
    where
        for<'a, 'b> F: Fn(&'a str, &'b str) -> bool,
    {
        let s1 = String::from("Rust");
        {
            let s2 = String::from("Programming");
            // s1 and s2 have different lifetimes, but the HRTB handles it
            if func(&s1, &s2) {
                println!("First is longer or equal");
            }
        }
    }

    compare_lengths(|a, b| a.len() >= b.len());

    // Example 2: Mixing HRTBs with Standard Generics
    /*
        - You can have a fixed lifetime 'a (chosen by caller)
        - AND a higher-ranked lifetime 'b (chosen by the function)
    */
    fn mix_lifetimes<'a, F>(external_ref: &'a str, func: F)
    where
        for<'b> F: Fn(&'a str, &'b str),
    {
        let internal_string = String::from("Local");
        func(external_ref, &internal_string);
    }

    // Example 3: Multiple Traits with HRTBs
    /*
        - You can apply HRTBs to multiple bounds using the `+` operator.
    */
    fn execute<F>(task: F)
    where
        for<'a> F: Fn(&'a i32) + Copy,
    {
        let val = 42;
        task(&val);
        let task_copy = task; // Works because of the Copy bound
        task_copy(&val);
    }
}
