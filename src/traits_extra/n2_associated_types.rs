/*
    - Associated types allow a trait to define a placeholder type that implementing structs can specify.
    - This is useful for abstraction without having to make the trait itself generic.
    - Primary purpose:
        - Simplify trait definitions when a type is closely tied to the trait.
        - Avoid repetitive generics in every function signature.
    - Associated types are only supported in traits.
*/

fn associated_types() {
    trait Container {
        type Item; // Placeholder type associated with this trait

        fn get(&self) -> Self::Item; // Uses the associated type
    }

    struct Boxed<T> {
        value: T,
    }

    impl<U> Container for Boxed<U>
    where
        U: Copy,
    {
        type Item = U; // Specify the associated type

        fn get(&self) -> Self::Item {
            self.value
        }
    }

    let b = Boxed { value: 10 };
    let _v = b.get(); // Returns i32, determined by Boxed<i32>
}

fn generic_associated_types() {
    /*
        - GATs extend associated types by allowing them to be generic over lifetimes or types
        - This is extremely useful for things like iterators or streams where the output type
            depends on a lifetime or other parameter.
        - Primary purpose:
            - Allow more flexible, lifetime-dependent associated types.
            - Enable advanced iterator patterns, streaming APIs.
        Use cases:
            - Iterator<Item<'a>>
            - Async streaming APIs
    */
    trait MyIterator {
        type Item<'a>
        where
            Self: 'a;

        fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
    }

    // Basic implementation example
    struct Numbers;

    impl MyIterator for Numbers {
        type Item<'a>
            = &'a i32
        where
            Self: 'a;

        fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
            None // Simplified, real impl would yield references to i32
        }
    }
}
