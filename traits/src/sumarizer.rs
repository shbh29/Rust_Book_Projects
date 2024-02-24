

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub name: String,
    pub content: String,
    pub author: String,
    pub date: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} published on {}, \n {} \n by {}", self.name, self.date, self.content, self.author)
    }
}



pub fn main() {
    let news_article = NewsArticle{
        name: String::from("Modiji"),
        content: String::from(
            "Paytm downfall for using nationalism and to sell more."
        ),
        author: String::from("Sudhanshu Gupta"),
        date: String::from("24th, Feb 2024")
    };

    println!("News Ariticle summary: {}", news_article.summarize());

}