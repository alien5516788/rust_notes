

fn main() {

    static mut VALUE: &'static i32 = &89;


    let x = 78;

    let y: &i32 = &56;

    // unsafe {
    //     {
    //         static V: i32 = 42;
    //         VALUE = &V;
    //     }
    // }

    fn test(v: &'static i32) {
        println!("{:?}", v);
    }

    unsafe {
        test(y);
    }

}
