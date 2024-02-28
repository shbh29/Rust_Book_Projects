#[derive(Debug, PartialEq)]
struct Shoe {
    size: i32,
    shoe_type: String,
}

fn match_size(shoes: Vec<Shoe>, size: i32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == size).collect()
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn get_my_size() {
        let shoes_vec = vec![
            Shoe{size: 13, shoe_type: String::from("Sneakers")},
            Shoe{size: 10, shoe_type: String::from("Sandal")},
            Shoe{size: 13, shoe_type: String::from("Formals")},
        ];

        let matched_size = match_size(shoes_vec, 13);

        assert_eq!(
            matched_size,
            vec![
                Shoe{size: 13, shoe_type: String::from("Sneakers")},    
                Shoe{size: 13, shoe_type: String::from("Formals")},
            ]
        );
        // assert_eq!
    }
}