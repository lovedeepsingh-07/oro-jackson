use crate::{components::FolderPageChildrenList, frontmatter, helpers, pages::BaseHTML, plugins};
use leptos::prelude::*;

#[component]
pub fn FolderPage(
    content: String,
    folder_page_children: Vec<plugins::emitters::folder_page::FolderPageChild>,
    show_folder_page_children: bool,
    show_file_explorer: bool,
    frontmatter: frontmatter::Frontmatter,
    project_title: String,
    file_tree: Vec<helpers::file_tree::TreeNode>,
) -> impl IntoView {
    view! {
        <BaseHTML frontmatter show_file_explorer project_title file_tree>
            <div class="mb-[100px] px-4">
                <div class="prose dark:prose-invert mx-auto max-w-5xl" inner_html={content}>
                </div>
                <FolderPageChildrenList folder_page_children show_folder_page_children/>
                <hr class="mx-auto mt-[25px] mb-[10px] w-full max-w-5xl opacity-[15%]" />
            </div>
        </BaseHTML>
    }
}
