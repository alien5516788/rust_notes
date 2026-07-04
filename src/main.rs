fn main() -> Result<(), String> {

    struct MyType {
        value: i32,
        c: i32,
    }

    impl MyType {
        fn new(value: i32) -> Self {
            Self {
                value,
                c: 0,
            }
        }
    }

    impl Iterator for MyType {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.c > 10 {
                None
            } else {
                self.c += 1;
                Some(self.value)
            }
        }
    }

    let m = MyType::new(4);

    for i in m {
        println!("{}", i);
    }

    // for i in m {
    //     println!("{}", i);
    // }

    

    // let mut v = Vec::from([2, 4, 5, 6]);

    // println!("{:?}", v);

    // for i in &mut v {
    //     println!("Multiplying '{}' by two", i);
    //     *i = *i * 2
    // }



    // println!("{:?}", v);



    Ok(())

}
