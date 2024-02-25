
pub fn external_add_two(a: i32) -> i32 {
    internal_add(a, 2)
}

fn internal_add(a: i32, b: i32) -> i32 {
    a+b
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_add() {
        let result = internal_add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_external_add() {
        let result = external_add_two(5);
        assert_eq!(7, result);
    }
}