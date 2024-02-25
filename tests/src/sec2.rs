
pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
pub mod my_test_mod_sec2 {
    use super::*;

    //1. run specific test
    //2. ignore the specific test
    //3. run ignored test : -- --ignored-test
    //4. run 2 tests
    //5. run test in sequence --test-thread=1
    //6. --show-output
    //7. -- --

    // Practiced commands
    // cargo test
// cargo test -- --help
// cargo test -- --test-threads 1
// cargo test 
// cargo test -- --ignored
// cargo test hundred
// cargo test two
// cargo test two -- --show-output


    #[test]
    fn add_verify_two() {
        println!("test add two");
        assert_eq!(add(2, 2), 4);
    }


    #[test]
    fn add_verify_five() {
        println!("test add five");
        assert_eq!(add(2, 5), 7);
    }


    #[test]
    #[ignore]
    fn add_verify_hundred() {
        assert_eq!(add(2, 100), 102);
    }

}