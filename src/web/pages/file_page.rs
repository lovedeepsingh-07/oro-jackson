use crate::web::pages::BaseHTML;
use leptos::prelude::*;

#[component]
pub fn FilePage(content: String) -> impl IntoView {
    view! {
        <BaseHTML>
            <div inner_html=content></div>
        </BaseHTML>
    }
}
