use crate::{frontmatter, pages::BaseHTML};
use leptos::prelude::*;

#[component]
pub fn FilePage(content: String, frontmatter: frontmatter::Frontmatter) -> impl IntoView {
    view! {
        <BaseHTML frontmatter>
            <div class="mb-[100px] px-4">
                <div inner_html={content} class="prose dark:prose-invert mx-auto max-w-5xl"></div>
                <hr class="mx-auto mt-[25px] mb-[10px] w-full max-w-5xl opacity-[15%]" />
            </div>
        </BaseHTML>
    }
}
