
pub mod summary;
pub mod news_article;

use crate::sumarizer::summary::Summary;
// use crate::sumarizer::news_article;

pub fn main() {
    let news_article = news_article::NewsArticle{
        name: String::from("Modiji"),
        content: String::from(
            "Paytm downfall for using nationalism and to sell more."
        ),
        author: String::from("Sudhanshu Gupta"),
        date: String::from("24th, Feb 2024")
    };

    println!("News Ariticle summary: {}", news_article.summarize());

}