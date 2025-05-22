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
                <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.22/dist/katex.min.css" integrity="sha384-5TcZemv2l/9On385z///+d7MSYlvIEw9FuZTIdZ14vJLqWphw7e7ZPuOiCHJcFCP" crossorigin="anonymous"/>

                <script src="/_static/scripts/katex.render.js"></script>
                <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.22/dist/katex.min.js" integrity="sha384-cMkvdD8LoxVzGF/RPUKAcvmm49FQ0oxwDF3BGKtDXcEc+T1b2N+teh/OJfpU0jr6" crossorigin="anonymous"></script>
            </head>
            <body class="bg-(--background-light) text-(--foreground-light) dark:bg-(--background-dark) dark:text-(--foreground-dark) overflow-x-auto">
            {children()}
            <script>
                r#"
                (function() {
                    const html = document.documentElement;
                    const storedTheme = localStorage.getItem("oro-jackson-theme");
                    if (storedTheme) {
                        html.className = storedTheme;
                    } else {
                        localStorage.setItem("oro-jackson-theme", html.className || "light");
                    }
                })();
                "#
            </script>
            </body>
        </html>
    }
}
