pub mod avg_collection {

    pub struct AverageCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AverageCollection {
        pub fn new() -> AverageCollection {
            AverageCollection {
                list: vec![],
                average: 0.0,
            }
        }

        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        fn update_average(&mut self) {
            let sum: i32 = self.list.iter().sum();
            self.average = sum as f64 / self.list.len() as f64;
        }

        pub fn remove(&mut self) {
            let res = self.list.pop();
            match res {
                Some(val) => println!("removed {}", val),
                None => println!("cannot remove anything"),
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        pub fn list(&self) -> Vec<i32> {
            self.list.clone()
        }
    }
}

pub mod use_avg_collection {
    use super::avg_collection::AverageCollection;

    pub fn main() {
        let mut ac = AverageCollection::new();

        ac.add(1);
        ac.add(5);
        ac.add(10);
        ac.add(15);

        println!("average of list {:?} is {}", ac.list(), ac.average());
    }
}

pub mod blog_post;
