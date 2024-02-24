pub trait Summary {
    // fn new() -> impl Summary;
    fn summary_author(&self) -> String{"".to_string()}
    fn summarize(&self) -> String {
        format!("Read along...")
    }
}

//todo: 1. default implementation in a trait can be used by another function, right?

//todo: 2. if a function returns a summary type, can a method of that struct be called?
//todo: 3. create an error wala function that returns two objects that implements summary trait
//todo: 4. Implement Display trait for Summary Struct.
//todo: 5. Implement PartialOrd trait for summary to compare between two objects of trait.
//todo: 6. Implement method taht uses both the functionalities of Display and partical order by comparing the two.


