use state_oop_learning_rust::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Ich habe heute Mittag einen Salat gegessen");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("Ich habe heute Mittag einen Salat gegessen", post.content());

}
