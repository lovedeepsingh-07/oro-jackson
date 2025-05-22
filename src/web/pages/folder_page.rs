use crate::{plugins, web::pages::BaseHTML};
use leptos::prelude::*;

#[component]
pub fn FolderPage(
    content: String,
    subfiles: Vec<plugins::emitters::folder_page::FolderPageChildLink>,
    show_folder_page_children: bool,
) -> impl IntoView {
    let sub_files = match show_folder_page_children {
        true => {
            let children_files = subfiles
                .into_iter()
                .map(|subfile| {
                    view! {
                        <a href={subfile.href} class="flex w-fit items-center gap-[16px] pr-[10px] border border-(--primary-light)/25 dark:border-(--primary-dark)/25 rounded-(--radius-light) dark:rounded-(--radius-dark)">
                            <span class="h-[30px] w-[25px] bg-(--primary-light)/75 dark:bg-(--primary-dark)/75 flex items-center justify-center">
                                <svg class="stroke-(--foreground-light) dark:stroke-(--foreground-dark)" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M19 9V17.8C19 18.9201 19 19.4802 18.782 19.908C18.5903 20.2843 18.2843 20.5903 17.908 20.782C17.4802 21 16.9201 21 15.8 21H8.2C7.07989 21 6.51984 21 6.09202 20.782C5.71569 20.5903 5.40973 20.2843 5.21799 19.908C5 19.4802 5 18.9201 5 17.8V6.2C5 5.07989 5 4.51984 5.21799 4.09202C5.40973 3.71569 5.71569 3.40973 6.09202 3.21799C6.51984 3 7.0799 3 8.2 3H13M19 9L13 3M19 9H14C13.4477 9 13 8.55228 13 8V3" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path> </g></svg>
                            </span>
                            <p class="w-fit">{subfile.name}</p>
                        </a>
                    }
                })
                .collect_view();
            let total_children = children_files.len();
            view! {
                <div class="items-left mx-auto flex max-w-5xl flex-col gap-[30px]">
                    <div class="flex gap-[4px] text-(--primary)">
                        <p>{total_children}</p>
                        <p>items under this folder.</p>
                    </div>
                    <div class="flex flex-col items-stretch mx-auto w-full max-w-5xl gap-[16px] text-xl font-bold">
                        {children_files}
                    </div>
                </div>
            }
            .into_any()
        }
        false => view! {}.into_any(),
    };
    view! {
        <BaseHTML>
            <div class="mb-[100px] px-4">
                <div class="prose dark:prose-invert mx-auto mt-[64px] max-w-5xl" inner_html={content}></div>
                {sub_files}
                <hr class="mx-auto mt-[25px] mb-[10px] w-full max-w-5xl opacity-[15%]" />
            </div>
        </BaseHTML>
    }
}
