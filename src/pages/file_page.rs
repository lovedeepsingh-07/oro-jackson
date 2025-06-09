use crate::{frontmatter, helpers, pages::BaseHTML};
use leptos::prelude::*;

#[component]
pub fn FilePage(
    content: String,
    show_file_explorer: bool,
    frontmatter: frontmatter::Frontmatter,
    project_title: String,
    file_tree: Vec<helpers::file_tree::TreeNode>,
) -> impl IntoView {
    view! {
        <BaseHTML frontmatter show_file_explorer project_title file_tree>
            <div class="mb-[100px] px-4">
                <div inner_html={content} class="prose dark:prose-invert mx-auto max-w-5xl"></div>
                <hr class="mx-auto mt-[25px] mb-[10px] w-full max-w-5xl opacity-[15%]" />
            </div>
        </BaseHTML>
    }
}
