use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {}

#[derive(Template)]
#[template(path = "resume.html")]
pub struct ResumeTemplate {}
