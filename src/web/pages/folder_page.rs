use crate::{processors, web::pages::BaseHTML};
use leptos::prelude::*;

#[component]
pub fn FolderPage(subfiles: Vec<processors::parse::FolderPageChildLink>) -> impl IntoView {
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
