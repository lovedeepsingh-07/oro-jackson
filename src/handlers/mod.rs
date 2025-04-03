use crate::server;
use axum::{self, response::IntoResponse};
use std::{
    fs,
    sync::{Arc, RwLock},
};

pub async fn main_route(
    axum::extract::State(web_state): axum::extract::State<Arc<RwLock<server::WebState>>>,
    axum::extract::Path(filepath): axum::extract::Path<String>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    let web_state = match web_state.write() {
        Ok(safe_web_state) => safe_web_state,
        Err(e) => {
            println!("Failed to get web state, Error {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to get web state"),
            ));
        }
    };

    let file_content =
        match fs::read_to_string(format!("{}/{}.html", web_state.output_path, filepath)) {
            Ok(safe_contents) => safe_contents,
            Err(e) => {
                println!(
                    "Failed to failed to read html file contents, Error: {:#?}",
                    e
                );
                return Err((
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Failed to failed to read html file contents"),
                ));
            }
        };
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        file_content,
    )
        .into_response());
}

pub async fn index_route(
    axum::extract::State(web_state): axum::extract::State<Arc<RwLock<server::WebState>>>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    let web_state = match web_state.write() {
        Ok(safe_web_state) => safe_web_state,
        Err(e) => {
            println!("Failed to get web state, Error {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to get web state"),
            ));
        }
    };

    let file_content = match fs::read_to_string(format!("{}/index.html", web_state.output_path)) {
        Ok(safe_contents) => safe_contents,
        Err(e) => {
            println!(
                "Failed to failed to read html file contents, Error: {:#?}",
                e
            );
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to failed to read html file contents"),
            ));
        }
    };
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        file_content,
    )
        .into_response());
}
