#[derive(Debug)]
struct Node {
    value: i32,
    parent: Option<Box<Node>>,
}

fn main() {
    let n = Node { value: 5, parent: Some(Box::new(Node{value: 1, parent: None})) };
    println!("Print: {:?}", n);
}