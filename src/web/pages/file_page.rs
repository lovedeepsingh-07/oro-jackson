use crate::web::pages::BaseHTML;
use ammonia;
use leptos::prelude::*;

#[component]
pub fn FilePage(content: String) -> impl IntoView {
    let cleaned_content = ammonia::clean(&content);
    view! {
        <BaseHTML>
            <div class="mb-[100px] px-4">
                <div inner_html={cleaned_content} class="prose dark:prose-invert mx-auto mt-[64px] max-w-5xl"></div>
                <hr class="mx-auto mt-[25px] mb-[10px] w-full max-w-5xl opacity-[15%]" />
            </div>
        </BaseHTML>
    }
}
