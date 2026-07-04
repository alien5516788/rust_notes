fn super_traits() {
    /*
       - Super traits are traits that other traits depend on
       - For a struct to implement a trait, it must first implement all its supertraits
       - Syntax: trait Sub: Super { ... }
    */

    // Super trait definition
    trait Super {
        fn basic_action(&self);
    }

    // Subtrait definition, requiring Super as a supertrait
    trait Sub: Super {
        fn advanced_action(&self);
    }

    // Struct definition
    struct MyStruct;

    impl Super for MyStruct {
        fn basic_action(&self) {
            println!("Basic action performed.");
        }
    }

    impl Sub for MyStruct {
        /*
           - Without the `Super` trait being first implemented, this would be an error
        */
        fn advanced_action(&self) {
            println!("Advanced action performed.");
        }
    }

    /*
        - You can require a type to implement multiple traits using the '+' syntax
    */
    trait Reader {
        fn read(&self);
    }

    trait Writer {
        fn write(&self);
    }

    // To implement 'SuperEditor', a type MUST implement Reader AND Writer.
    trait SuperEditor: Reader + Writer {
        fn atomic_edit(&self) {
            self.read();
            self.write();
            println!("Atomic edit complete.");
        }
    }

    struct Document;

    impl Reader for Document {
        fn read(&self) {
            println!("Reading document...");
        }
    }

    impl Writer for Document {
        fn write(&self) {
            println!("Writing document...");
        }
    }

    impl SuperEditor for Document {}

    /*
        - Super traits allow number of traits bounds to be written
    */
    fn perform_maintenance<T: SuperEditor>(tool: T) {
        /*
            - Instead of T: Reader + Writer + SuperEditor, just use T: SuperEditor
        */
        tool.read();
        tool.write();
        tool.atomic_edit();
    }

    // ===============TODO===============
    /*
        -Dyn compatibility: Rust currently does not allow creating a trait object (dyn)
        from a trait with multiple supertraits easily if you need to upcast back to the supertraits.
        Example of a valid trait object:
        let editor: &dyn SuperEditor = &Document;
    */
    // ===================================
}
