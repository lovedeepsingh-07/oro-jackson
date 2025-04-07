use askama::Template;
use bon;

#[derive(Template, bon::Builder)]
#[template(path = "page.html")]
pub struct PageTemplate {
    pub content: String,
}
