use super::summary::Summary;

pub struct Tweet {
    pub user: String,
    pub content: String,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summary_author(&self) -> String {
        format!(" by {}", self.user)
    }
    fn summarize(&self) -> String {
        format!("Tweet {} {}", self.content, self.summary_author())
    }
}