use crate::{context, oj_file};
use leptos::prelude::*;
use std::path;

pub mod file_page;
pub mod folder_page;

pub struct PageFrontmatter {
    title: String,
}

pub fn get_title_from_file(ctx: &context::Context, input_path: &str) -> String {
    let curr_file_path = path::Path::new(input_path);
    if let Some(curr_file_name) = curr_file_path.file_stem() {
        if curr_file_name == "index" {
            if let Some(parent_path) = curr_file_path.parent() {
                if parent_path.to_string_lossy().to_string() == ctx.build_args.content {
                    return ctx.config.title.clone();
                } else {
                    if let Some(parent_name) = parent_path.file_name() {
                        return format!("Folder: {}", parent_name.to_string_lossy().to_string());
                    }
                }
            }
        }
        return curr_file_name.to_string_lossy().to_string();
    }
    return "null".to_string();
}

impl PageFrontmatter {
    pub fn new(ctx: &context::Context, curr_file: &oj_file::OjFile) -> Self {
        let mut page_title = String::from("null");
        match &curr_file.frontmatter {
            oj_file::OjFrontmatter::Yaml(frontmatter) => {
                if frontmatter.is_null() {
                    page_title = get_title_from_file(ctx, &curr_file.input_path);
                } else if let Some(title) = frontmatter.get("title") {
                    if let Ok(ok_title) = serde_yaml::from_value::<String>(title.clone()) {
                        page_title = ok_title;
                    } else {
                        page_title = get_title_from_file(ctx, &curr_file.input_path);
                    }
                } else {
                    page_title = get_title_from_file(ctx, &curr_file.input_path);
                }
            }
            oj_file::OjFrontmatter::Toml(frontmatter) => {
                if let Some(title) = frontmatter.get("title") {
                    page_title = title.to_string();
                }
            }
        }
        return PageFrontmatter { title: page_title };
    }
}

#[component]
pub fn BaseHTML(children: Children, frontmatter: PageFrontmatter) -> impl IntoView {
    view! {
        <!doctype html>
        <html lang="en" data-theme="oj-dark">
            <head>
                <title>{frontmatter.title.clone()}</title>
                <meta name="viewport" content="width=device-width, initial-scale=1" />

                <link rel="stylesheet" href="/_static/theme.css" />
                <link rel="stylesheet" href="/_static/style.css" />

                <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.22/dist/katex.min.css" integrity="sha384-5TcZemv2l/9On385z///+d7MSYlvIEw9FuZTIdZ14vJLqWphw7e7ZPuOiCHJcFCP" crossorigin="anonymous"/>
                <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.22/dist/katex.min.js" integrity="sha384-cMkvdD8LoxVzGF/RPUKAcvmm49FQ0oxwDF3BGKtDXcEc+T1b2N+teh/OJfpU0jr6" crossorigin="anonymous"></script>
                <script src="/_static/scripts/katex.render.js"></script> // custom katex render script

                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/default.min.css" />
                <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/highlight.min.js"></script>
                <link rel="stylesheet" href="/_static/highlightjs.theme.css" />
                <script src="/_static/scripts/highlightjs.render.js"></script>

                <script type="module" src="/_static/scripts/mermaid.render.js"></script>
            </head>
            <body class="overflow-x-auto">
                <div class="fixed top-0 right-0 left-0 z-[80] flex items-center justify-end p-4">
                    <label class="swap swap-rotate btn btn-square btn-ghost hover:bg-base-300 stroke-base-content">
                        <input type="checkbox" id="oj-theme-toggle-checkbox" />

                        <svg
                            class="swap-off size-[1.5rem]"
                            viewBox="0 0 24 24"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                            ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
                                id="SVGRepo_tracerCarrier"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            ></g><g id="SVGRepo_iconCarrier">
                                <path
                                    d="M12 3V4M12 20V21M4 12H3M6.31412 6.31412L5.5 5.5M17.6859 6.31412L18.5 5.5M6.31412 17.69L5.5 18.5001M17.6859 17.69L18.5 18.5001M21 12H20M16 12C16 14.2091 14.2091 16 12 16C9.79086 16 8 14.2091 8 12C8 9.79086 9.79086 8 12 8C14.2091 8 16 9.79086 16 12Z"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                ></path>
                            </g></svg
                        >

                        <svg
                            class="swap-on size-[1.5rem]"
                            viewBox="0 0 24 24"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                            ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
                                id="SVGRepo_tracerCarrier"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            ></g><g id="SVGRepo_iconCarrier">
                                <path
                                    d="M3.32031 11.6835C3.32031 16.6541 7.34975 20.6835 12.3203 20.6835C16.1075 20.6835 19.3483 18.3443 20.6768 15.032C19.6402 15.4486 18.5059 15.6834 17.3203 15.6834C12.3497 15.6834 8.32031 11.654 8.32031 6.68342C8.32031 5.50338 8.55165 4.36259 8.96453 3.32996C5.65605 4.66028 3.32031 7.89912 3.32031 11.6835Z"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                ></path>
                            </g></svg
                        >
                    </label>
                </div>
                <div class="mt-[64px] mb-[24px] px-4">
                    <div class="mx-auto max-w-5xl">
                        <p class="text-4xl font-bold text-wrap">{frontmatter.title.clone()}</p>
                    </div>
                </div>
                {children()}
                <script src="/_static/scripts/theme.control.js"></script>
            </body>
        </html>
    }
}
