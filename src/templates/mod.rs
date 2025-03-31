use askama::Template;

#[derive(Template)]
#[template(path = "page.html")]
pub struct PageTemplate {
    pub content: String,
}
