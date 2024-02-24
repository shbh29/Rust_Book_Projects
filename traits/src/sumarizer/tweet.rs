use super::summary::Summary;
use std::fmt::{Display, Formatter};


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

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "My Tweet: {} by {}", self.content, self.user)
    }
}