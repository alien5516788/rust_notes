fn marker_traits() {
    /*
        - Marker traits are traits without methods. They mark types as having
           some property.
        - Nothing is special about marker traits them selves, they are traits without methods.
            - Just like structs, enums without fields (not unit structs)
            - Only usage is special.
        - Primary purposeis to encode metadata about a type without adding behavior.
        - Use cases:
           - To make trait bounds without having methods.
    */

    trait Marker {}

    struct MyType;

    // Marks 'MyType' with this trait
    impl Marker for MyType {}

    // Usage example
    fn do_nothing(arg: impl Marker) {
        println!("I did nothing with the argument.");
    }
}
