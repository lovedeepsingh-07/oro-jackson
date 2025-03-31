// imports
use crate::error;
use bon;

// modules
#[cfg(test)]
mod tests;

// ----- `WebState` object
#[derive(Debug, Clone)]
pub struct WebState {
    pub content_path: String,
}
#[bon::bon]
impl WebState {
    #[builder]
    pub fn new(content_path: String) -> Result<WebState, error::WebStateError> {
        return Ok(WebState { content_path });
    }
}
