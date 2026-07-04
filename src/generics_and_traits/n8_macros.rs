// ===============================================================
// RUST MACRO SYSTEM CHEAT SHEET
// ===============================================================
// Metaprogramming: Writing code that generates other code.
// Built into the Abstract Syntax Tree (AST) -> Safer than C macros.
// ===============================================================

// ---------------------------------------------------------------
// 1. DECLARATIVE MACROS (macro_rules!)
// ---------------------------------------------------------------
// - Uses: Pattern matching on code tokens.
// - Best for: Variadic functions, repetitive boilerplate, simple DSLs.
// - Hygiene: Variables inside the macro don't leak out (mostly).

macro_rules! say_hello {
    // () indicates no arguments
    () => {
        println!("Hello!");
    };
    // ($name:expr) matches a single expression
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    // $(...)* matches zero or more repetitions
    ($($x:expr),*) => {
        $(
            println!("Hello, {}!", $x);
        )*
    };
}

// ---------------------------------------------------------------
// 2. PROCEDURAL MACROS (The "Function-like" Macros)
// ---------------------------------------------------------------
// - Requirement: Must be defined in a separate crate of type `proc-macro`.
// - Logic: Functions that take `TokenStream` and return `TokenStream`.
// - Power: Can execute any Rust code during compilation.

// A. CUSTOM DERIVE
// Usage: #[derive(MyTrait)]
// Purpose: Automatically implements a trait for a struct or enum.
// Example: #[derive(Serialize, Deserialize)] from Serde.

// B. ATTRIBUTE-LIKE MACROS
// Usage: #[my_attribute]
// Purpose: Creates custom attributes for items (functions, structs).
// Example: #[tokio::main] or #[get("/")] in Rocket/Actix.

// C. FUNCTION-LIKE MACROS
// Usage: my_macro!(...)
// Purpose: Looks like declarative, but uses complex logic/parsing.
// Example: sqlx::query!("SELECT...") or format_args!(...).

// ---------------------------------------------------------------
// QUICK COMPARISON TABLE
// ---------------------------------------------------------------
/*
 | Feature         | Declarative (macro_rules!) | Procedural               |
 |-----------------|----------------------------|--------------------------|
 | Engine          | Pattern Matching           | Imperative Rust Code     |
 | Input           | Token fragments            | TokenStream              |
 | Definition      | Same crate/module          | Must be a separate crate |
 | Use Case        | Simple repetitions         | Complex logic/Derives    |
*/

// ---------------------------------------------------------------
// WHY USE THEM?
// ---------------------------------------------------------------
// 1. DRY: Eliminate structural boilerplate.
// 2. DSL: Create domain-specific languages (like HTML inside Rust).
// 3. Variadics: Handle unknown number of arguments (e.g., vec![1, 2, 3]).

fn main() {
    // Calling our declarative macro examples:
    say_hello!(); // Match 1
    say_hello!("Alice"); // Match 2
    say_hello!("Bob", "Eve"); // Match 3 (variadic)
}

// [Inference] Based on the Rust compiler's architecture, Procedural macros
// are significantly more resource-intensive at compile time than Declarative ones.

// ============================
// Rust Macros Summary Demo
// ============================
//
// This file summarizes all major macro types in Rust:
// 1. Declarative macros (`macro_rules!`)
// 2. Procedural macros:
//    a) Function-like (`#[proc_macro]`)
//    b) Derive (`#[proc_macro_derive]`)
//    c) Attribute (`#[proc_macro_attribute]`)
//
// Shows syntax differences in definition and expansion
// ============================

fn main2() {
    println!("=== 1. Declarative macro (macro_rules!) ===");

    // -----------------------------
    // Declarative macro (macro_rules!)
    // -----------------------------
    // Definition syntax: `macro_rules! name { patterns }`
    macro_rules! say_hello {
        ($name:ident) => {
            println!("Hello, {}!", stringify!($name));
        };
    }

    // Expansion / usage syntax: macro_name!(...) with matching delimiters
    say_hello!(Alice); // ✅ uses ()
                       // say_hello![Alice]; // ❌ would fail: delimiters must match

    // Can generate almost any valid Rust code
    macro_rules! generate_struct {
        ($name:ident) => {
            struct $name {
                value: i32,
            }
        };
    }
    generate_struct!(Person);
    let x = Person { value: 10 };
    println!("Person.value = {}", x.value);

    // Token trees ($tt) allow arbitrary sequences
    macro_rules! arbitrary_tokens {
        ($tokens:tt) => {
            println!("Got arbitrary tokens!");
        };
    }
    // arbitrary_tokens!(foo + bar * 42); // ✅ works, validity checked after expansion // linter yells

    println!("\n=== 2. Procedural macros (overview) ===");
    println!("Procedural macros require a separate crate with `proc-macro = true`.");
    println!(
        "They receive TokenStreams as input, unlike macro_rules! which only sees token trees."
    );

    // -----------------------------
    // Function-like procedural macro
    // -----------------------------
    // Definition syntax (in a proc-macro crate):
    // #[proc_macro]
    // pub fn make_42(input: TokenStream) -> TokenStream { ... }
    //
    // Usage / expansion syntax: make_42!(...)
    println!("Function-like proc macro: foo!(...)");

    // -----------------------------
    // Derive procedural macro
    // -----------------------------
    // Definition syntax:
    // #[proc_macro_derive(TraitName)]
    // pub fn derive_trait(input: TokenStream) -> TokenStream { ... }
    //
    // Usage / expansion syntax:
    // #[derive(TraitName)]
    // struct MyStruct;
    println!("Derive proc macro: #[derive(TraitName)] on struct/enum");

    // -----------------------------
    // Attribute procedural macro
    // -----------------------------
    // Definition syntax:
    // #[proc_macro_attribute]
    // pub fn attr_macro(attr: TokenStream, item: TokenStream) -> TokenStream { ... }
    //
    // Usage / expansion syntax:
    // #[attr_macro(...)]
    // fn foo() { ... }
    println!("Attribute proc macro: #[attr_macro] applied to item (function, struct, etc.)");

    println!("\n=== 3. Delimiters for macros ===");
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

    println!("\n=== 4. Built-in / compiler macros ===");
    // Some macros like `format_args!` or `println!` are compiler built-ins
    // cargo expand may not show full expansion because they are handled internally
    // ::std::io::_print(format_args!("Hello from format_args! macro\n")); // linter yells

    // vec! uses [] because the macro pattern is defined with brackets
    let v = vec![1, 2, 3];
    println!("v = {:?}", v);

    println!("\n=== Summary Table ===");
    println!("Type               | Definition Syntax                   | Usage Syntax");
    println!("-------------------+------------------------------------+-------------------------");
    println!("Declarative        | macro_rules! name {{ patterns }}    | name!(...)");
    println!("Function-like      | #[proc_macro] fn name(TokenStream) | name!(...)");
    println!("Derive             | #[proc_macro_derive(Name)]          | #[derive(Name)]");
    println!("Attribute          | #[proc_macro_attribute] fn name(...) | #[name(...)] on item");

    println!("\nKey points:");
    println!("- macro_rules! sees token trees (TT) only, no AST.");
    println!("- Procedural macros receive TokenStream, can parse to AST using syn crate.");
    // println!("- Delimiters in macro calls must match definition: (), {}, []"); // linter yells
    println!(
        "- Built-in macros (format_args!, vec!, println!) may hide expansions from cargo expand."
    );
}
