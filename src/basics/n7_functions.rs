/*
    - Functions define behaviors
        - They encapsulate logic that can be executed
    - Functions cannot be overloaded
        - It would lead to ambiguity and confusion
        - Overloading is usually done through traits and generics
    - Functions are a callable type
        - Every function has a concrete type like `fn(i32) -> i32`
        - They aren’t structs, but they are first-class types
    - Functions can implement custom traits just like structs but under certain conditions
        - You cannot directly implement a trait on a built-in function pointer type due to Rust’s orphan rules
        - You can wrap the function in a struct and implement traits on that wrapper
    - Functions can be thought of as `callable` types
        - Rust provides `Fn`, `FnMut`, `FnOnce` traits that functions automatically implement (compiler magic)
    - Rust supports two types of functions
        1. Functions
        2. Closures (Anonymous functions)
*/

fn functions() {
    /*
        - Syntax: `fn(Type) -> ReturnType`
    */

    // Example 1
    // Functions evaluate to unit type by default (curly braces do)
    fn function_1() {}
    fn function_2() -> () {}
    fn function_3() {
        2;
    }
    fn function_4() -> () {
        2;
    }
    fn function_5() -> () {
        2;
        () // note no semicolon
    }
    fn function_6() -> () {
        return (); // explicit evalaution with return
    }

    // Example 2
    // Function with custom return type
    fn function_7() -> i32 {
        2 // note no semicolon
    }

    fn function_8() -> i32 {
        return 2;
    }

    // Example 3
    // Function with parameters
    fn function_9(a: i32, b: i32) -> i32 {
        return a + b;
    }

    fn function_10(a: i32, b: i32, name: String) -> i32 {
        println!("Hello {}", name);
        return a + b;
    }

    /*
        - Function call
    */
    function_1();
    let _s = function_9(2, 4);
}

fn function_pointers() {
    /*
        - Functions can be used as objects via pointers
        - Concreate type for a 
        - Actual function code is never duplicated or owned
    */

    // Function definition
    fn speak(name: String) -> () {
        println!("Hello {}", name);
    }

    let speak_ptr: fn(String) = speak; // here function doesn't get owned from `speak`
    speak_ptr(String::from("Name"));

    // Example 1
    fn print_name(name: String) -> () {
        println!("Hello {}", name);
    }

    let _print_name_ptr: fn(String) = print_name;

    // Example 2
    fn return_sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }

    let _return_sum_ptr: fn(i32, i32) -> i32 = return_sum;

    // Example 3
    fn return_clone<T: Clone>(t: T) -> T {
        t.clone()
    }

    let _return_clone_ptr: fn(String) -> String = return_clone; // function signature doesn't contain generic type

    // Example 4
    fn return_sum_ptr() -> fn(i32, i32) -> i32 {
        |num1: i32, num2: i32| num1 + num2
    } // this function returns a function pointer (don't get confused by the signature)

    /*
        - As functions by default implements Fn, FnMut, or FnOnce traits,
            functions can be passed via trait objects
        - Note: These are not function pointers
    */

    // Example
    fn return_sum_fn(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }

    let _return_sum_fn_ptr1: &dyn Fn(i32, i32) -> i32 = &return_sum_fn;
    let _return_sum_fn_ptr2: &dyn FnMut(i32, i32) -> i32 = &return_sum_fn;
    let _return_sum_fn_ptr3: &dyn FnOnce(i32, i32) -> i32 = &return_sum_fn;

    /*
        - `Fn(i32, i32)` this function like trait syntax is actually a syntactic sugar for `Fn<(i32, i32), Output = i32>`
    */
}
