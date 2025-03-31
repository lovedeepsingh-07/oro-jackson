use crate::server;
use axum::{self, response::IntoResponse};
use std::{
    fs, path,
    sync::{Arc, RwLock},
};

pub async fn home_route(
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

    let mark_file_content = fs::read_to_string(path::Path::new(&format!(
        "{}/05 - notes/writing-an-interpreter-in-go_throsten-ball_22-03-2025.md",
        web_state.content_path,
    )))
    .unwrap();

    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_MATH);
    options.insert(pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    options.insert(pulldown_cmark::Options::ENABLE_GFM);
    let parser = pulldown_cmark::Parser::new_ext(&mark_file_content, options);
    let mut output_html = String::new();

    pulldown_cmark::html::push_html(&mut output_html, parser);
    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        output_html,
    )
        .into_response());

    // let mut options = comrak::Options::default();
    // options.extension.math_dollars = true;
    // options.extension.front_matter_delimiter = Some("---".to_owned());
    // return Ok((
    //     [(
    //         axum::http::header::CONTENT_TYPE,
    //         String::from("text/html; charset=utf-8"),
    //     )],
    //     comrak::markdown_to_html(&mark_file_content, &options),
    // )
    //     .into_response());
}
