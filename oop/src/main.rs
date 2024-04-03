use oop::blog_post::Post;



fn main() {
    let mut post = Post::new();

    post.add_text("This is some added text");

    post.review_request();

    assert_eq!("", post.content());

    post.approve();
    assert_eq!("This is some added text", post.content());
}
