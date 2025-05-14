use leptos::prelude::*;

pub mod file_page;
pub mod folder_page;

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
