use crate::{error, server};
use axum::{self, response::IntoResponse};
use color_eyre::eyre::WrapErr;
use mime_guess;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn main_route(
    axum::extract::State(web_state): axum::extract::State<Arc<RwLock<server::WebState>>>,
    axum::extract::Path(filepath): axum::extract::Path<String>,
) -> error::HandlerResult<axum::response::Response> {
    let web_state = web_state.write().await;

    if filepath.starts_with("_static") {
        // serve a static file
        let file_content = web_state
            .output_path
            .join(&filepath)?
            .read_to_string()
            .wrap_err("failed to read contents of a static file")?;

        let file_type = mime_guess::from_path(&filepath).first_or_octet_stream();

        return Ok((
            [(axum::http::header::CONTENT_TYPE, file_type.to_string())],
            file_content,
        )
            .into_response());
    } else {
        let curr_file = web_state.output_path.join(format!("{}.html", filepath))?;
        let curr_index_file = web_state.output_path.join(&filepath)?.join("index.html")?;
        if curr_file.exists()? {
            // serve a page file
            let file_content = curr_file
                .read_to_string()
                .wrap_err("failed to read contents of a page file")?;
            return Ok((
                [(
                    axum::http::header::CONTENT_TYPE,
                    String::from("text/html; charset=utf-8"),
                )],
                file_content,
            )
                .into_response());
        } else if curr_index_file.exists()? {
            // serve a folder index file
            let file_content = curr_index_file
                .read_to_string()
                .wrap_err("failed to read contents of a folder index file")?;

            return Ok((
                [(
                    axum::http::header::CONTENT_TYPE,
                    String::from("text/html; charset=utf-8"),
                )],
                file_content,
            )
                .into_response());
        } else {
            // no such file
            return Err(error::Error::NotFound(format!(
                "no such file or directory, file: {:#?}",
                filepath
            )))?;
        }
    }
}

pub async fn index_route(
    axum::extract::State(web_state): axum::extract::State<Arc<RwLock<server::WebState>>>,
) -> error::HandlerResult<axum::response::Response> {
    let web_state = web_state.write().await;

    let curr_file = web_state.output_path.join("index.html")?;
    let file_content = curr_file
        .read_to_string()
        .wrap_err("failed to read contents of the index(home) file")?;
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        file_content,
    )
        .into_response());
}
