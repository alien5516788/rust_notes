fn main() {

    struct Node {
        value: i32,
        twice: i32,
    }

    impl From<i32> for Node {
        fn from(value: i32) -> Self {
            Node { value, twice: value * 2 }
        }
    }

    impl From<Node> for i32 {
        fn from(node: Node) -> Self {
            node.value
        }
    }

    fn print_node(node: Node) {
        println!("Value: {}, Twice: {}", node.value, node.twice);
    }

    fn print_value(value: i32) {
        println!("Value: {}", value);
    }
    
    let node = Node::from(42);
    
    print_node(12.into());
    print_value(node.into());

    
}