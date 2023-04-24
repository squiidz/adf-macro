use adf_template_lib::prelude::*;

#[derive(serde::Serialize)]
#[adf_template(path = "./template/comment.html")]
struct Comment {
    title: String,
    body: String,
}

#[derive(serde::Serialize)]
#[adf_template(path = "./template/issue.html")]
struct Issue {
    issue_id: String,
}

fn main() {
    let comment = Comment {
        title: "Hello".to_string(),
        body: "World".to_string(),
    };
    if let Ok(adf) = comment.to_adf() {
        println!("{:?}", adf);
    }

    let issue = Issue {
        issue_id: "CLOUD-123".to_string(),
    };
    if let Ok(adf) = issue.to_adf() {
        println!("{:?}", adf);
    }
}
