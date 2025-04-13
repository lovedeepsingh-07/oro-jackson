use crate::{error, server};
use axum::{self, response::IntoResponse};
use color_eyre::eyre::WrapErr;
use mime_guess;
use std::{fs, path, sync::Arc};
use tokio::sync::RwLock;

pub async fn main_route(
    axum::extract::State(web_state): axum::extract::State<Arc<RwLock<server::WebState>>>,
    axum::extract::Path(filepath): axum::extract::Path<String>,
) -> error::HandlerResult<axum::response::Response> {
    let web_state = web_state.write().await;

    if filepath.starts_with("_static") {
        // serve a static file
        let file_contents = fs::read_to_string(format!("{}/{}", web_state.output_path, filepath))
            .wrap_err("failed to read contents of a static file")?;

        let file_type = mime_guess::from_path(filepath.to_string()).first_or_octet_stream();

        return Ok((
            [(axum::http::header::CONTENT_TYPE, file_type.to_string())],
            file_contents,
        )
            .into_response());
    } else {
        let curr_file_string = format!("{}/{}.html", web_state.output_path, filepath);
        let curr_file_path = path::Path::new(&curr_file_string);
        let curr_file_index_string = format!("{}/{}/index.html", web_state.output_path, filepath);
        let curr_file_index_path = path::Path::new(&curr_file_index_string);
        if curr_file_path.exists() {
            // serve a page file
            let file_contents = fs::read_to_string(curr_file_string)
                .wrap_err("failed to read contents of a page file")?;
            return Ok((
                [(
                    axum::http::header::CONTENT_TYPE,
                    String::from("text/html; charset=utf-8"),
                )],
                file_contents,
            )
                .into_response());
        } else if curr_file_index_path.exists() {
            // serve a folder index file
            let file_contents = fs::read_to_string(curr_file_index_string)
                .wrap_err("failed to read contents of a folder index file")?;

            return Ok((
                [(
                    axum::http::header::CONTENT_TYPE,
                    String::from("text/html; charset=utf-8"),
                )],
                file_contents,
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

    let file_content = fs::read_to_string(format!("{}/index.html", web_state.output_path))
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
