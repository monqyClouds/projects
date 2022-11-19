use state_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    // wont add text
    post.add_text("I ate a salad for lunch today, again!");

    // post.reject();
    // assert_eq!("", post.content());

    post.approve();
    post.approve(); // calling the second time
    assert_eq!("I ate a salad for lunch today", post.content());
}
