
pub struct Post {
    content: String,
    state: Option<Box<dyn State>>
}

impl Post {
    pub fn new() -> Post {
        Post {
            content: String::from(""),
            state: Some(Box::new(Draft {}))
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn review_request(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.review_request());
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.review_request())
        }
    }
}

trait State {
    fn review_request(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
struct PendingReview {}
struct Publish {}

impl State for Draft {
    fn review_request(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }
}
impl State for PendingReview {
    fn review_request(self: Box<Self>) -> Box<dyn State> {
        Box::new(Publish{})
    }
}

impl State for Publish {
    fn review_request(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    
}
