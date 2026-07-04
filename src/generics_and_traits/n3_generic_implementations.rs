pub mod implementing_generics_traits_and_structs {
    fn generic_struct_implementation() {
        /*
            - Generic struct allows multiple implementations
                for different concrete types of same struct.
            - There is a thing to watchout.
                when there is a universal implementation, functions or methods cannot be
                redundant in concrete implementations as they would overlap with the universal implementation.
            - Structs still can have multiple implementation blocks for the same concrete type,
                as long as they are not redundant.
        */

        // Example
        struct Container<T> {
            item: T,
        }

        // generic implementation for all types U
        impl<U> Container<U> {
            fn new(item: U) -> Self {
                Self { item }
            }

            fn get_item(&self) -> &U {
                &self.item
            }
        }

        // concrete implementation for i32 type
        impl Container<i32> {
            fn double(&self) -> i32 {
                self.item * 2
            }
        }
    }

    fn generic_trait_implementation() {
        /*
            - Generic trait allows multiple implementations
                for the same type but with different type parameter.
            - There is also a thing to watchout.
                When there is a universal implementation, there cannot be implementations
                for any concrete types and vice versa. Becuase they would overlap with the universal implementation.
            - Traits still have to be fully implemented in one block.
        */

        // Example
        trait Interact<T> {
            fn interact(&self, value: T);
        }

        struct Robot {
            id: i32,
        }

        // implementing generic trait for concrete type
        // either universal implementation or concrete implementations can present, not both
        /*
            impl<U> Interact<U> for Robot {
                fn interact(&self, value: U) {
                    println!("Robot {} received a message", self.id);
                }
            }
        */

        // implementing generic trait for concrete type
        impl Interact<String> for Robot {
            fn interact(&self, value: String) {
                println!("Robot {} received message: {}", self.id, value);
            }
        }

        // implementing same trait but different type parameter
        impl Interact<i32> for Robot {
            fn interact(&self, value: i32) {
                println!("Robot {} received number: {}", self.id, value);
            }
        }
    }
}
