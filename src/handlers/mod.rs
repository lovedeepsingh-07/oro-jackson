// imports
use crate::{core, templates};
use askama_axum::Template;
use axum::{self, response::IntoResponse};
use std::sync::{Arc, RwLock};

// GET (/) home page route handler
pub async fn home_page() -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    // render HTML struct
    let html = match (templates::routes::HomePage {}.render()) {
        Ok(safe_html) => safe_html,
        Err(e) => {
            println!("Failed to render HTML, Error {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to render HTML"),
            ));
        }
    };

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        html,
    )
        .into_response());
}

// GET (/vault/*note_path) route handler
#[axum_macros::debug_handler]
pub async fn vault_page(
    axum::extract::State(app_state): axum::extract::State<Arc<RwLock<core::app::Application>>>,
    axum::extract::Path(note_path): axum::extract::Path<String>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    let app_state = match app_state.write() {
        Ok(safe_app_state) => safe_app_state,
        Err(e) => {
            println!("Failed to get application state, Error {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to get application state"),
            ));
        }
    };
    println!("{:#?}", note_path);

    let page_data = serde_json::to_string_pretty(&app_state.clone().vault_map).unwrap();

    let html = match (templates::routes::VaultPage { page_data }.render()) {
        Ok(safe_html) => safe_html,
        Err(e) => {
            println!("Failed to render HTML, Error {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to render HTML"),
            ));
        }
    };

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        html,
    )
        .into_response());
}
