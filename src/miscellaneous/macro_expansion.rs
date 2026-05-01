// ============================
// Rust Macros Demonstration
// ============================
//
// This file illustrates everything we discussed about Rust macros:
// - macro_rules! basics
// - macro expansion vs macro definition
// - token trees
// - delimiters (), {}, []
// - macro scope and variable hygiene
// - how macros can generate arbitrary code
// - compiler checks happening after expansion
// - built-in macros like format_args! and vec!
// ============================

fn main() {
    println!("=== Basic macro expansion ===");

    // -----------------------------
    // 1. Simple macro defining a struct
    // -----------------------------
    macro_rules! make_struct {
        ($name:ident) => {
            struct $name {
                value: i32,
            }
        };
    }

    // Macro call expands into a real struct definition here:
    make_struct!(Person);

    // Now we can use the generated struct
    let x = Person { value: 23 };
    println!("Person.value = {}", x.value);

    // -----------------------------
    // 2. Macro injecting a let binding
    // -----------------------------
    macro_rules! make_variable {
        ($name:ident) => {
            let $name: i32 = 42; // variable exists in this scope
        };
    }

    make_variable!(my_var);
    println!("my_var = {}", my_var); // ✅ works

    // Important: if variable is unused, compiler may optimize it away
    // Macros do not create "hidden variables" — they expand in-place

    // -----------------------------
    // 3. Macro with arbitrary tokens
    // -----------------------------
    macro_rules! arbitrary_tokens {
        ($tokens:tt) => {
            println!("Received tokens!");
        };
    }

    // arbitrary_tokens!(asdf 123 + foo_bar); // But linter yells, so I commented it
    // ✅ Allowed: tokens don't need to make sense until expansion

    // -----------------------------
    // 4. Smallest macro
    // -----------------------------
    macro_rules! minimal {
        () => {};
    } // minimal, does nothing
    minimal!(); // expands to ()

    // -----------------------------
    // 5. Scope and expansion errors
    // -----------------------------
    macro_rules! invalid_macro {
        () => {
            let ""; // ❌ invalid Rust code
        };
    }
    // invalid_macro!();
    // ⚠ If uncommented, this will error at the call site
    // The error belongs to the expanded code, not the definition

    // -----------------------------
    // 6. Delimiters: (), {}, []
    // -----------------------------
    macro_rules! paren_macro {
        ($x:expr) => {
            println!("Paren: {}", $x);
        };
    }
    macro_rules! brace_macro {
        { $x:expr } => { println!("Brace: {}", $x); };
    }
    macro_rules! bracket_macro {
        [ $x:expr ] => { println!("Bracket: {}", $x); };
    }

    paren_macro!(10); // ✅ ()
    brace_macro! {20}; // ✅ {}
    bracket_macro![30]; // ✅ []

    // Delimiters must match macro definition exactly
    // paren_macro![10]; // ❌ would fail

    // -----------------------------
    // 7. Macros generating almost anything
    // -----------------------------
    macro_rules! complex_macro {
        ($name:ident) => {
            struct $name {
                value: i32,
            }
            impl $name {
                fn new(v: i32) -> Self {
                    $name { value: v }
                }
            }
        };
    }

    complex_macro!(Thing);
    let thing = Thing::new(99);
    println!("Thing.value = {}", thing.value);

    // -----------------------------
    // 8. Built-in macros and compiler magic
    // -----------------------------
    // `format_args!` is a compiler-built-in macro
    // ::std::io::_print(format_args!("Hello built-in macro\n")); // Linter yells, so I commented it again
    // cargo expand does not fully expand `format_args!` because it's compiler intrinsic
    // It produces a fmt::Arguments struct internally

    // -----------------------------
    // 9. vec![] macro uses brackets
    // -----------------------------
    let v = vec![1, 2, 3]; // vec! macro is defined with `[]`
    println!("v = {:?}", v);

    // Why brackets? Because the macro pattern is `[ $( $x:expr ),* ]`
    // Macros match token trees grouped by delimiters: (), {}, []
}

// ============================
// Notes / Summary
// ============================
//
// - Macros are like **code generators**: they paste code at the call site
// - Errors are reported
