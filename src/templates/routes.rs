// imports
use crate::utils;
use askama_axum::Template;

// ----- `HomePage` template object
#[derive(Template)]
#[template(path = "routes/index.html")]
pub struct HomePage {}

// ----- `VaultNotePage` template object
#[derive(Template)]
#[template(path = "routes/vault.html")]
pub struct VaultPage {
    pub page_data: Vec<utils::VaultObject>,
}

// ----- `VaultNotePage` template object
#[derive(Template)]
#[template(path = "routes/vault_note.html", escape = "none")]
pub struct VaultNotePage {
    pub page_data: String,
}
