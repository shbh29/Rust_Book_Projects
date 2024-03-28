use oop::avg_collection::AverageCollection;




fn main() {
    let mut ac = AverageCollection::new();

    ac.add(1);
    ac.add(5);
    ac.add(10);
    ac.add(15);

    println!("average of list {:?} is {}", ac.list(), ac.average());
}
