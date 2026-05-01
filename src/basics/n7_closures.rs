// Functions cont...

fn closures() {
    /*
        - Closures are functions that can capture variables from their surrounding environment
        - Closures can be bind to variables instead of having a fixed function identifier (anonymous)
        - Syntax: `|Type| -> ReturnType`
    */
    
    // Example 1
    let function_1 = || {};
    
    // same as
    fn function_1() {}
    
    let _function_2 = || -> () {};
    let _function_3 = || -> () { () };

    let _function_4 = || -> i32 { 32 };

    let _function_5 = || -> i32 {
        return 32;
    };

    let _function_6 = || {
        println!("Hello World");
    };

    let _function_7 = |name: String| {
        println!("Hello {}", name);
    };

    let function_8 = |num1: i32, num2: i32| -> i32 { num1 + num2 };

    /*
        - Function call
    */
    function_1();
    let _s = function_8(1, 2);
}

fn closure_capture() {
    /*
        - Closures can capture variables from their surrounding environment
        - Closure that capture varibles cannot directly fn() concrete type,
          Instead the type is defined as a trait objects or thorugh static dispatch that implements FnMut, FnOnce
        - Only variables that are used within the closure are captured
        - Closures automatically decide how to capture variables
            - &T       (immutable borrow)
            - &mut T   (mutable borrow)
            - T        (move / ownership)

        - The `move` keyword forces capture by value
        - Without `move` closure captures variables in the least restrictive way
    */

    // Example 1
    // If closure reads a variable, closure captures by immutable reference (&T)
    let x = 10;

    let capture = || {
        println!("x = {}", x); // x is captured by immutable reference (&x), because we only read it
    };

    capture();
    println!("Still usable: {}", x); // works

    // Example 2
    // If closure modifies a variable, closure captures by mutable reference (&mut T)
    let x = 10;
    let mut count = 0;

    let mut increment = || {
        count += x;
    };

    increment();

    println!("Still usable: {}", x); // works
    println!("Still usable: {}", count); // works

    // Example 3
    // If closure forces move with `move`, closure takes ownership of variables it uses
    let s = String::from("Hello");

    let capture = move || {
        println!("{}", s); // s is moved into closure.
    };

    capture();
    // println!("{}", s); // error

    // Example 4
    // If the captured type implements Copy trait, it is copied instead of moved
    let x = 5; // Copy
    let y = String::from("hello"); // no Copy

    let capture = move || {
        println!("{}", x);
        println!("{}", y);
    };

    capture();
    println!("Still usable: {}", x); // works
                                     // println!("{}", y); // error
}

fn closures_for_threads() {
    use std::thread;

    /*
        - Threads require `'static` closures
        - That means the closure must own everything it uses
        - Therefore, `move` is almost always required.
    */

    // Example
    let name = String::from("Alien");
    let age = 20;

    let handle = thread::spawn(move || {
        println!("{} is {}", name, age); // only 'name' is moved, because it doesn't implement Copy
    });

    // println!("Not usable: {}", name); // error
    println!("Still usable: {}", age); // works

    handle.join().unwrap();

    /*
        - In order to capture without moving, use a pointer like `Arc`
    */
}

fn type_annotations_for_closures() {
    /*
        - Fact:
            - Closures are actually structs with function pointers.
            - It internelly converted to something like this
                ```
                struct __Closure {
                    x: i32,
                }

                impl Fn<(i32), Output = i32> for __Closure {
                    fn call(&self, y: i32) -> i32 {
                        self.x + y
                    }
                }
                ```
        - There for the type annotations for closures cannot be used directly
        - Instead `Fn`, `FnMut`, `FnOnce` traits can be used to annotate closures with trait objects
    */

    // Example 1
    // Immutable borrow
    let x = 10;

    let _capture: &dyn Fn() = &|| {
        println!("x = {}", x);
    };

    // Example 2
    // Mutable borrow
    /*
        - `mut` is required here because the closure captures is a struct and
            struct fields for captured variables are mutable, so struct/closure itself should be mutable
    */
    let x = 10;
    let mut count = 0;

    let mut _increment: &mut dyn FnMut() = &mut || {
        count += x;
    };

    // Example 3
    // Move
    // Trait object cannot be a shared reference, because calling FnOnce takes ownership
    let s = String::from("Hello");

    let _capture_once: Box<dyn FnOnce()> = Box::new(move || {
        println!("{}", s);
    });
}
