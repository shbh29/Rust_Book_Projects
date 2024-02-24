use super::summary::Summary;

pub struct Tweet {
    pub user: String,
    pub content: String,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("User {} has tweeted: {}", self.user, self.content)
    }
}