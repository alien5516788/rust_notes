pub mod types_of_generics {
    /*
        - Rust have 3 main kinds or generics
            1. Type generics
            2. Const generics
            3. Life time generics
        - Generics allows mutiple concrete types to be created out of generic types
        - By default generics use static dispatch
    */

    /*
        - Generic bounds support
            |Generic Kind|	 |Trait Bounds|	 |Trait Objects|	 |Lifetime Bounds|
            Type (T)	         Yes	          Yes	            Yes
            Const (N)	         No	              No	            No
            Lifetime ('a)	     No	              No	            Yes
    */

    // 1. Type generics
    fn type_generics() {
        /*
            - Type generics to specify the concrete type for generic parameters
            - Allows us to write generic code that can work with multiple concrete types
        */

        // Example 1
        /*
           - The concrete type for `T` and `U` is determined at compile time by the caller
        */
        fn return_tup<T, U>(arg1: T, arg2: U) -> (T, U) {
            return (arg1, arg2);
        }

        return_tup(3.4, 56);
        return_tup::<f32, i32>(3.4, 56);
        return_tup::<f32, &str>(3.5, "world");

        // Example 2
        /*
            - The concrete type for `T` and `U` is determined at compile time by struct instantiation
        */
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let point = Point { x: 3.4, y: 56 };
        let point = Point::<f32, i32> { x: 3.5, y: 5 };

        // Example 3
        /*
            - The concrete type for `T` is determined at compile time by enum instantiation
        */
        enum Output<T> {
            Value(T),
            None,
        }

        let output = Output::Value(34);
        let output = Output::<&str>::Value("hello");

        // Example 4
        /*
            - The concrete type for `T` is determined at compile time by trait implementation
        */
        trait Communicate<T> {
            fn speak(&self, arg: T);
        }

        struct Person;

        impl Communicate<i32> for Person {
            fn speak(&self, arg: i32) {
                println!("I yelled {}", arg);
            }
        }

        // Example 5
        /*
            - Trait implementation for `Human` that works with any type `U`
        */
        struct Human;

        impl<U> Communicate<U> for Human {
            fn speak(&self, arg: U) {
                println!("I say nothing, becuase I don't know what to say with T");
            }
        }
    }

    // 2. Const generics
    fn const_generics() {
        /*
            - Const generics allow specifying constant values as generic parameters instead of types
            - Const generics support only integer types, bool, char, and references to them
            - Unlike type generics, the constant value cannot be inferred from the context and must be explicitly
                provided inside the turbofish syntax
        */

        // Example 1
        /*
            - The constant value for `U` is determined by the caller at compile time
        */
        fn return_value<T, const U: i32>(arg1: T) -> i32 {
            return 34 + U;
        }

        return_value::<i32, 42>(34);

        // Example 2
        /*
            - The constant value for `N` is determined by struct instantiation
        */
        struct List<T, const N: usize> {
            t: T,
            l: [i32; N],
        }

        let list = List { t: 23, l: [0; 7] };
        let list = List::<i32, 4> { t: 23, l: [0; 4] };

        // Example 3
        /*
            - The constant value for `N` is determined by enum instantiation
        */
        enum Output<T, const N: usize> {
            Item([i32; N]),
            Value(T),
        }

        let output = Output::<i32, 6>::Value(34);

        // Example 4
        /*
            - Constant value for the `A` parameter is determined by trait implementation
        */
        trait FindLocation<const A: i32> {
            fn is_existing(&self, location: i32) -> bool {
                return location + A > 10;
            }
        }

        struct Rover;

        impl FindLocation<100> for Rover {} // Default implementation automatically uses constant value 100
    }

    // 3. Life time generics
    fn lifetime_generics() {
        /*
            - Describe the relationship between the lifetimes of references
            - Life time generics on functions define the lifetime of the arguments and return value
            - Life time generics on other items define the lifetime of attributes and items themselves
            - Always applied on references
            - Note: There is also an special lifetime parameter `'static` apart from custom
                lifetime parameters like `'a`, `'b`, `'c`, etc
            - `'static` indicates that the value has a lifetime that is the entire duration of the program
            - Variables initialized with static keyword will have a `'static` lifetime
        */

        // Example 1
        /*
            - 'a ties input and output references together
            - Both `x` and `y` must live at least as long as both input references
            - Without `'a`, compiler cannot guarantee that the lifetime of return value is consistent
                and will result in a compile error
        */
        let x = String::from("hello");
        let y = String::from("world");

        fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        longest(&x, &y);

        // Example 2
        /*
            - Here `x` and `y` have different lifetimes
            - `y` has a shorter lifetime than `x`
            - If the `result` is used later in program, `y` will have already gone out of scope
                and compiler will throw an error
        */
        let x = String::from("hello");

        fn shortest<'a>(x: &'a String, y: &'a String) -> &'a String {
            if x.len() < y.len() {
                x
            } else {
                y
            }
        }

        let result;

        {
            let y = String::from("world");
            result = shortest(&x, &y);
            // Becuase `y` goes out of scope before `result` is used
        }

        // println!("{}", result); // error

        // Example 3
        /*
            - The struct `Holder` cannot outlive the references it contains
            - If `name` or `title` are dropped, the `Holder` instance becomes invalid
            - Both `name` and `title` are tied to the same lifetime 'a, meaning
                the struct's lifetime is limited by whichever is shorter
        */
        struct Holder<'a> {
            name: &'a str,
            title: &'a str,
            age: i32,
        }

        let name = String::from("Alice");
        let holder;

        {
            let title = String::from("Manager");
            holder = Holder {
                name: &name,
                title: &title,
                age: 30,
            };
            // title goes out of scope here
        }

        // println!("{}", holder.title); // error

        // Example 4
        /*
            - The lifetime 'a ensures that if a `Message` variant holds a reference,
                that reference must remain valid for the life of the enum instance
            - If you try to move a `Message::Text` into a scope where the original
                string is gone, the compiler will block it
        */
        enum Message<'a> {
            Text(&'a str),
            Number(&'a i32),
            Flag(bool),
        }

        let msg;
        let text = String::from("Hello");
        msg = Message::Text(&text);

        // Example 5
        /*
            - Here, 'a specifies that the string slice `s` passed to `print`
                must live at least as long as the lifetime parameter defined by the trait
            - This is often used when the implementation of the trait needs to
                store that reference or use it for a specific duration
        */
        trait Printer<'a> {
            fn print(&self, s: &'a str);
        }

        struct MyPrinter;

        impl<'a> Printer<'a> for MyPrinter {
            fn print(&self, s: &'a str) {
                println!("{}", s);
            }
        }

        // Example 6
        /*
            - `'static` data is usually stored directly in the program's binary
            - Unlike 'a, it never goes out of scope
            - A reference to a string literal (e.g., "hello") is implicitly `'static`
            - `shout` function only accepts references that live for the whole program
            - The variable name `z` cannot be used as a local variable or anywhere in the program
                because it shadows the static `z`
        */
        fn shout(s: &'static i32) {
            println!("Number is {}", s);
        }

        let x = 45;
        static z: i32 = 78;

        shout(&z);
        // shout(&x); // error
        // 'x' is a local String and will be dropped eventually
    }
}
