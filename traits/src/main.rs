use traits::sumarizer::{news_article, tweet, tv_report};
use traits::impls::params;
use traits::sumarizer::summary::Summary;

fn main() {
    let news_article = news_article::NewsArticle{
        name: String::from("Modiji"),
        content: String::from(
            "Paytm downfall for using nationalism and to sell more."
        ),
        author: String::from("Sudhanshu Gupta"),
        date: String::from("24th, Feb 2024")
    };

    let tweet = tweet::Tweet {
        user: String::from("Ravichandran Ashwin"),
        content: String::from(
            "Ravindra Jadega is a fun guy. Remember to ride horse in Test Match as well."
        ),
        retweet: false,
    };

    let tv_report = tv_report::TvReport {
        news_anchor: String::from("Sudhir Chaudhary"),
        tv_channel: String::from("Aaj Tak"),
        topic: String::from("Chhatra Copy"),
    };


    // println!("News Ariticle summary: {}", news_article.summarize());
    // println!("Tweet summary: {}", tweet.summarize());
    // println!("Tweet summary: {}", tv_report.summarize());
    

    params::broadcast(&news_article);
    params::broadcast(&tv_report);
    params::broadcast(&tweet);
}
