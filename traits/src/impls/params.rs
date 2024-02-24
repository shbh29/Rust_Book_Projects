use crate::sumarizer::summary::Summary;

pub fn broadcast<T: Summary>(t: &T) {
    println!("Breaking News: {}", t.summarize());
}

