pub mod simple_reference {
    pub fn main() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
        println!("done");
    }
}

pub mod custom_mybox {
    use std::ops::Deref;

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(v: T) -> MyBox<T> {
            MyBox(v)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    pub fn main() {
        let x = "String test";
        let y = MyBox::new(String::from(String::from("String test")));

        assert_eq!("String test", x);
        assert_eq!("String test", *y);
        println!("done deref coersion");
    }
}

pub mod node {
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: Option<Box<Node>>,
    }

    fn main() {
        let n = Node { value: 5, parent: Some(Box::new(Node{value: 1, parent: None})) };
        println!("Print: {:?}", n);
    }
}


