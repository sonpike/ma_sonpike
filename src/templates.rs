use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct MyTemplate {}
