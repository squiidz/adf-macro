use adf_template_lib::adf_template;
use serde::Serialize;

#[derive(adf_template, Serialize)]
struct Comment {
    title: String,
    body: String,
}

fn main() {
    let comment = Comment::new("Hello".to_string(), "World".to_string());
    println!("{:?}", comment.generate());
}