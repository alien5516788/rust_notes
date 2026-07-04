fn unit_type() {
    /*
        - Same as tuple with no values
        - Used as void
        - Only the value can be assign to is also `()`
    */
    let _v: () = ();

    // Note:
    // Tuple with multiple values
    let _v: (i32, f32) = (1, 3.4);
}

fn never_type() {
    /*
        - A type that indicates that no value produces
        - Cannot be assigned directly
        - Basically the program will not continue to execute, after encountering `!`
    */
    fn infinite_loop() -> ! {
        loop {}
    }

    fn panic_example() -> ! {
        panic!("This function never returns!");
    }
}

fn boolean_type() {
    /*
        - Indicate true or false states
    */
    let _t: bool = true;
    let _f: bool = false;
}

fn integer_types() {
    /*
        - Signed integers can hold both negative and positive values
    */
    let _i8_max: i8 = 127;
    let _i16_max: i16 = 32_767;
    let _i32_max: i32 = 2_147_483_647;
    let _i64_max: i64 = 9_223_372_036_854_775_807;
    let _i128_max: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    let _isize_max: isize = isize::MAX; // platform-dependent

    /*
        - Unsigned integers can hold only positive values
    */
    let _u8_max: u8 = 255;
    let _u16_max: u16 = 65_535;
    let _u32_max: u32 = 4_294_967_295;
    let _u64_max: u64 = 18_446_744_073_709_551_615;
    let _u128_max: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let _usize_max: usize = usize::MAX; // platform-dependent

    /*
        - Prefixes:
            - `0x` or `0X` for hexadecimal
            - `0o` or `0O` for octal
            - `0b` or `0B` for binary
            - No prefix for decimal
    */
    let _dec_i32: i32 = 2_147_483_647i32; // decimal
    let _hex_i32: i32 = 0x7FFF_FFFF; // hexadecimal
    let _bin_i32: i32 = 0b0111_1111_1111_1111_1111_1111_1111_1111; // binary
    let _oct_i32: i32 = 0o17_777_777_777; // octal

    /*
        -Suffixes:
            - Used for type conversion
            - Type name is added as a suffix
    */
    let _u32_val: u32 = 4_294_967_295;
    let _f32_large: f32 = 340_282_350_000_000_000_000_000_000_000_000_000_000f32;
    let _f64_large: f64 = 179_769_313_486_231_570_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000f64;
    let _hex_u32: u32 = 0xFFFF_FFFFu32;

    /*
       - Scientific notation
       - By default scientific notation evaluates to a float
    */
    let _scn: i32 = 3e8 as i32;
}

fn float_type() {
    /*
        - Represents numeric values with a floating point
    */
    let _f32_large: f32 = 340_282_350_000_000_000_000_000_000_000_000_000_000f32;
    let _f64_large: f64 = 179_769_313_486_231_570_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000f64;

    /*
       - Scientific notation
       - As you can see scientific notation is your friend for large numbers
    */
    let _f32_max: f32 = 3.402_823_47e38f32;
    let _f64_max: f64 = 1.797_693_134_862_315_7e308f64;
}

fn char_type() {
    /*
        - Represents a Unicode scalar value
        - Single quotes required
    */
    let _c: char = 'a';
    let _emoji: char = '😊';
}

fn str_type() {
    /*
        - Dynamically sized string slice type
        - Almost always used behind a pointer type (e.g. `&str`, Box<&str>, String)
    */
    let _s1: &str = "hello";
    let _s2: Box<str> = String::from("hello").into_boxed_str();
    let _s3: String = String::from("rust");
}
