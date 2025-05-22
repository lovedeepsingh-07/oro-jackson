use crate::web::pages::BaseHTML;
use leptos::prelude::*;

#[component]
pub fn FilePage(content: String) -> impl IntoView {
    view! {
        <BaseHTML>
            <div inner_html={content} class="prose dark:prose-invert mx-auto mt-[64px] mb-[100px] max-w-5xl px-4"></div>
        </BaseHTML>
    }
}
