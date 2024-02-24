use crate::sumarizer::summary::Summary;

pub fn broadcast(t: &impl Summary) {
    println!("Breaking News: {}", t.summarize());
}