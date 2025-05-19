use crate::{plugins, web::pages::BaseHTML};
use leptos::prelude::*;

// #[component]
// pub fn FolderPage(subfiles: Vec<processors::parse::FolderPageChildLink>) -> impl IntoView {
//     let sub_files = subfiles
//         .into_iter()
//         .map(|subfile| {
//             view! {
//                 <a href={subfile.href}>{subfile.name}</a>
//             }
//         })
//         .collect_view();
//
//     view! {
//         <BaseHTML>
//             <div>
//             {sub_files}
//             </div>
//         </BaseHTML>
//     }
// }

#[component]
pub fn FolderPage(
    content: String,
    subfiles: Vec<plugins::emitters::folder_page::FolderPageChildLink>,
) -> impl IntoView {
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
                <div inner_html={content}>
                </div>
                <div class="flex flex-col items-left gap-[5px] text-xl font-bold text-(--primary-light)">
                {sub_files}
                </div>
            </div>
        </BaseHTML>
    }
}
