fn variables() {
    /*
        - Syntax: `let name: Type = value;`
    */

    /*
        - Immutable variable
        - By default all variables are immutable
    */
    let _var: i32;
    _var = 4;

    let _var = 4; // type inference

    let _var: i32 = 4;

    /*
        - Mutable variable
    */
    let mut _var: i32;
    _var = 4;

    let mut _var = 4;

    let mut _var: i32 = 4;

    /*
        - Constant:
            - Must have explicit type
            - Value must be known at compile time
            - Inlined everywhere it's used, therefore has no fixed memory address
    */
    const CNST_VAR: i32 = 4;

    /*
        - Static:
            - Must have explicit type
            - Lives for entire program lifetime, therefore has fixed memory address
            - Can be mutated, but requires `unsafe` block to modify
    */
    static STC_VAR: i32 = 67;

    static mut MUT_STC_VAR: i32 = 67;
    unsafe {
        MUT_STC_VAR = 34;
    }

    /*
        - Empty variable
            - `_` Just takes out a value and discard it
        - Variable names can be prefixed with `_` to indicate they are unused (e.g. `_var`)
    */
    let _ = 34;
    _ = 64;

    let _var = 33;
}

fn type_aliases() {
    /*
        - Type aliases define an aliase for type
        - Syntax: type AliasName = TypeName;
    */

    // Example 1
    type Int32 = i32;
    let num: Int32 = 78;

    // Example 2
    type Data = Option<Box<String>>;
    let d: Data = Some(Box::from("hello".to_string()));

    // Example 2
    struct Person<'a> {
        name: &'a str,
        age: i32,
    }
    type PersonData = Option<Box<Person<'static>>>;
    let d = Some(Box::from(Person {
        name: "John",
        age: 34,
    }));
}

fn statements_and_expressions() {
    /*
        - Expressions evaluate to a value and can be used in statements
    */
    7 + 8   /* Evaluates to 15 . */ ; // ignore the semicolon
    "hello" /* Evaluates to "hello" . */ ; // ignore the semicolon
    5.6     /* Evaluates to just 5.6 . */ ; // ignore the semicolon

    /*
        - Statements are used to execute side effects and evaluate to unit type (`()`)
        - Semicolon is used to terminate the expression
    */
    7 + 8; // Evaluates to `()`

    /*
        - Block expression creates a new scope
        - Can be used to group expressions together
        - Block expressions evaluate to the value of their last expression
        - Block without any expression evaluates to unit type by default
    */
    {} // evaluates to `()`

    {
        ()
    } // evaluates to `()`

    {
        8 + 9;
    } /* evaluates to `17` */ ; // ignore the semicolon

    {
        8 + 9;
    } // evaluates to `()`

    {
        8 + 9
    }; // evaluates to `()`
}

fn assignments() {
    /*
        - Variable can be used to assign final values evaluated from statements or expressions
    */

    // Example 1
    // `x` is assigned the value of `42`
    let x = 42;

    // Example 2
    // `y` is assigned the value of `x + 1`, which is `43`
    let y = x + 1;

    // Example 3
    // `_z` is assigned the value evaluated from the block expression
    let _z = {
        let a = 4;
        let b = 5;
        a + b + y // note: no semicolon, so it evaluates to `52`
    };

    // Example 4
    let _z = {
        let a = 4;
        let b = 5;
        a + b + y; // note: semicolon, so it evaluates to `()`
    };
}
