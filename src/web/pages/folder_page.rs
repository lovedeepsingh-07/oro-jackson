use crate::{
    plugins,
    web::{self, pages::BaseHTML},
};
use leptos::prelude::*;

#[component]
pub fn FolderPage(
    content: String,
    subfiles: Vec<plugins::emitters::folder_page::FolderPageChildLink>,
    show_folder_page_children: bool,
    frontmatter: web::pages::PageFrontmatter,
) -> impl IntoView {
    let sub_files = match show_folder_page_children {
        true => {
            let children_files = subfiles
                .into_iter()
                .map(|subfile| {
                    view! {
                        <a
                            href={subfile.href}
                            class="hover:bg-accent/15 flex w-fit items-center gap-[16px] pr-[10px] transition-all pr-[16px]"
                        >
                            <span class="bg-accent/75 h-[28px] w-[5px] items-center justify-center"></span>
                            <p class="w-fit text-wrap">{subfile.name}</p>
                        </a>
                    }
                })
                .collect_view();
            let total_children = children_files.len();
            view! {
                <div class="items-left mx-auto flex max-w-5xl flex-col gap-[30px]">
                    <div class="flex gap-[4px]">
                        <p>{total_children}</p>
                        <p>items under this folder.</p>
                    </div>
                    <div class="mx-auto flex w-full max-w-5xl flex-col items-stretch gap-[18px] text-xl font-bold">
                        {children_files}
                    </div>
                </div>
            }
            .into_any()
        }
        false => view! {}.into_any(),
    };
    view! {
        <BaseHTML frontmatter>
            <div class="mb-[100px] px-4">
                <div class="prose dark:prose-invert mx-auto max-w-5xl" inner_html={content}>
                </div>
                {sub_files}
                <hr class="mx-auto mt-[25px] mb-[10px] w-full max-w-5xl opacity-[15%]" />
            </div>
        </BaseHTML>
    }
}
