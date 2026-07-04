fn trait_objects() {
    /*
        - Trait objects allow for dynamic dispatch in Rust
        - They let you work with values whose concrete type is unknown at compile time,
            as long as they implement a specific trait
        - Key points:
            - Type is erased at compile time
            - Only the trait (behavior) is known
            - Uses dynamic dispatch via a vtable at runtime
            - Requires a pointer (`&`, `Box`, `Rc`, etc.) because size is unknown
            - Trait must be object-safe
    */

    // trait implementation
    struct User {
        name: String,
    }

    struct Product {
        id: u32,
    }

    trait Describable {
        fn describe(&self);
    }

    impl Describable for User {
        fn describe(&self) {
            println!("User: {}", self.name);
        }
    }

    impl Describable for Product {
        fn describe(&self) {
            println!("Product ID: {}", self.id);
        }
    }

    // Example
    // uses `dyn Trait` syntax
    fn print_description(item: &dyn Describable) {
        /*
           - `item` can be any type implementing trait `Describable`
           - Dispatch is dynamic (resolved at runtime via v-table)
        */
        item.describe();
    }

    let user = User {
        name: String::from("Alien"),
    };

    let product = Product { id: 42 };

    print_description(&user);
    print_description(&product);
}

fn multiple_trait_objects() {
    /*
        - Multiple trait objects can be added using `+` operator
    */

    // Example
    // `Send` and `Sync` traits from std lib
    fn print_send(_x: &(dyn Send + Sync)) {
        println!("Just look at parameters");
    }

    // =======TODO===========
    // However, multiple non-auto traits are NOT allowed directly
    trait Describable {
        fn describe(&self) -> String;
    }
    // You need a supertrait:
    trait Displayable: Describable {
        fn display(&self);
    }

    fn print_supertrait(item: &dyn Displayable) {
        item.display();
    }
    // ==============================
}
