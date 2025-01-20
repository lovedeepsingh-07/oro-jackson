// imports
use askama_axum::Template;

// ----- `HomePage` template object
#[derive(Template)]
#[template(path = "routes/index.html")]
pub struct HomePage {}

// ----- `VaultPage` template object
#[derive(Template)]
#[template(path = "routes/vault.html")]
pub struct VaultPage {
    pub page_data: String,
}
