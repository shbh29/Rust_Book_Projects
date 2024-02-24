use crate::sumarizer::summary::Summary;

pub fn broadcast<T: Summary, U: Summary, X: Summary>(t: &T, u: &U, x: &X) {
    println!("Breaking News: {}", t.summarize());
    println!("Breaking News: {}", u.summarize());
    println!("Breaking News: {}", x.summarize());
}

