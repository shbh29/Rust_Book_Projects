use super::summary::Summary;

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
