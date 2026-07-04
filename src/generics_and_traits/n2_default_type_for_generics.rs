pub mod default_type_for_generics {
    /*
        - Default types are supported for
            - Type generics
            - Const generics (nightly)
        - Generics can have default types for generic parameters
        - Default type for generic parameter is not supported for functions
        - Default type is mostly for compatibility with older code
             When new type was added, old type signatures don't need to be updated
    */

    // 1. Type generics
    fn type_generics() {
        // Example 1
        struct Item<T, U = i32> {
            t: T,
            l: U,
        }

        let item = Item { t: 42, l: 3.14 };
        let item2 = Item::<&str> { t: "hello", l: 34 };

        // Example 2
        enum Output<T, U = f32> {
            Item(T),
            Value(U),
        }

        let output = Output::<f32>::Item(3.14);
        let output = Output::<i32>::Value(3.14);

        // Example 3
        trait Work<T = i32> {
            fn do_something(&self, value: T);
        }

        struct Worker;

        impl Work for Worker {
            fn do_something(&self, value: i32) {
                println!("Working on {}", value);
            }
        }
    }

    // 2. Const generics (nightly)
    fn const_generics() {
        #![feature(const_generics_defaults)] // Enable const generics defaults

        // Example
        struct ArrayWrapper<T, const N: usize = 10> {
            data: [T; N],
        }

        let default_buffer = ArrayWrapper { data: [0; 10] };
        let custom_buffer = ArrayWrapper::<i32, 5> { data: [0; 5] };
    }
}
