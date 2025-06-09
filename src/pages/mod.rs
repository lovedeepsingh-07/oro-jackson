use crate::{
    components::{FileExplorer, Navbar},
    frontmatter, helpers,
};
use leptos::prelude::*;

pub mod file_page;
pub mod folder_page;

#[component]
pub fn BaseHTML(
    children: Children,
    frontmatter: frontmatter::Frontmatter,
    show_file_explorer: bool,
    project_title: String,
    file_tree: Vec<helpers::file_tree::TreeNode>,
) -> impl IntoView {
    let page_title = frontmatter.title.clone();
    let layout_body = match show_file_explorer {
        true => view! {
            <FileExplorer project_title file_tree>
                <div class="overflow-x-auto">
                    <div class="mt-[64px] mb-[24px] px-4">
                        <div class="mx-auto max-w-5xl">
                            <p class="text-4xl font-bold text-wrap">{page_title.clone()}</p>
                        </div>
                    </div>
                    {children()}
                </div>
            </FileExplorer>
        }
        .into_any(),
        false => view! {
            <div class="overflow-x-auto">
                <div class="mt-[64px] mb-[24px] px-4">
                    <div class="mx-auto max-w-5xl">
                        <p class="text-4xl font-bold text-wrap">{frontmatter.title.clone()}</p>
                    </div>
                </div>
                {children()}
            </div>
        }
        .into_any(),
    };
    return view! {
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
            <body>
                <Navbar show_file_explorer/>
                {layout_body}
                <script src="/_static/scripts/theme.control.js"></script>
            </body>
        </html>
    };
}
