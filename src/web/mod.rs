use crate::content;
use leptos::prelude::*;

#[component]
pub fn BaseHTML(children: Children) -> impl IntoView {
    view! {
        <!doctype html>
        <html lang="en" class="dark">
            <head>
                <title>oro-jackson</title>
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="stylesheet" href="/_static/theme.css" />
                <link rel="stylesheet" href="/_static/style.css" />
            </head>
            <body class="bg-(--secondary-light)">
            {children()}
            </body>
        </html>
    }
}

#[component]
pub fn FolderPage(subfiles: Vec<content::FolderPageChildLink>) -> impl IntoView {
    let sub_files = subfiles
        .into_iter()
        .map(|subfile| {
            view! {
                <a href={subfile.href}>{subfile.name}</a>
            }
        })
        .collect_view();

    view! {
        <BaseHTML>
            <div>
            {sub_files}
            </div>
        </BaseHTML>
    }
}

#[component]
pub fn FilePage(content: String) -> impl IntoView {
    view! {
        <BaseHTML>
            <div inner_html=content></div>
        </BaseHTML>
    }
}
