fn lifetime_bounds() {
    /*
        - Lifetime generics and lifetime bounds are related but different concepts
        - Lifetime generics:
            - Introduce named lifetimes (e.g. 'a, 'b)
            - Describe relationships between references
        - Lifetime bounds:
            - Restrict how long a type or reference must live
            - Express outlives relationships
        - All lifetime relationships are enforced at compile time
    */

    // Example 1
    // lifetime bounds on lifetime generics
    fn update_list<'a, 'b: 'a>(num: &'a i32, num_list: &'b mut Vec<&'a i32>) {
        /*
            - Describe outlives relationships.
            - Meaning:
                - Vector `num_list` lives as long as `'b`
                - Vector contains references to integers that live as long as `'a`
                - Integer reference `num` lives as long as `'a`, so integer can be safely pushed into vector
                - No dangling references occur because references outlives vector
        */
        num_list.push(num);
    }

    // Example 2
    // lifetime bounds on types generics
    fn make_tuple<'a, T: 'a>(num1: T, num2: T) -> (T, T) {
        /*
            Meaning `Num1` and `num2` must live at least as long as `'a` (same lifetime)
        */
        (num1, num2)
    }
}

fn lifetime_bounds_with_trait_bounds_and_trait_objects() {
    /*
        Lifetime bounds can be mixed with trait bounds and trait objects
    */

    // trait definition
    trait Describable {
        fn describe(&self) -> String;
    }

    // Example 1
    fn take_value1<'a>(num: &'a i32, obj: impl Describable + 'a) {
        /*
             - `num` and `obj` must live at least as long as `'a` (same duration)
             - `obj` must implement `Describable` trait
        */
        println!("{} {}", num, obj.describe());
    }

    // Example 2
    fn takes_value2(obj: Box<dyn Describable + 'static>) {
        /*
             - Boxed trait object must have static lifetime (entire program duration)
                 and must implement `Describable` trait
        */
        println!("{}", obj.describe());
    }
}
