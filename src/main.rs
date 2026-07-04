fn main() {

    #[derive(Debug)]
    enum Values {
        Bin,
        Oct,
        Dec,
        Hex,
    }

    let value = Values::Bin;
  
    println!("Value: {:?}", value);
  

}
