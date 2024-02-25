// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//     fn can_contain(&self, other : &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }



// #[cfg(test)]
// #[ignore]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     #[should_panic(expected = "Fails the test")]
//     fn panics() {
//         panic!("Fails the test");
//     }

//     #[test]
//     fn bigger_rect_contains_smaller_rect() {
//         let bigger = Rectangle {
//             width: 30,
//             height: 50,
//         };
    
//         // can_contain 
//         // dbg! to print values
//         let smaller = Rectangle {
//             width: 25,
//             height: 45,
//         };

//         assert!(bigger.can_contain(&smaller));
//         assert!(!smaller.can_contain(&bigger));
//     }

//     #[test]
//     fn add_verify() {
//         assert_eq!(add(2, 2), 4);
//     }
// }
