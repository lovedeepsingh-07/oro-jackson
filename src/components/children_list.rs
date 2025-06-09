use crate::plugins;
use leptos::prelude::*;

#[component]
pub fn FolderPageChildrenList(
    folder_page_children: Vec<plugins::emitters::folder_page::FolderPageChild>,
    show_folder_page_children: bool,
) -> impl IntoView {
    match show_folder_page_children {
        true => {
            let children_files = folder_page_children
                .into_iter()
                .map(|subfile| {
                    return view! {
                        <a
                            href={subfile.href}
                            class="hover:bg-accent/15 flex w-fit items-center gap-[16px] pr-[10px] transition-all pr-[16px]"
                        >
                            <span class="bg-accent/75 h-[28px] w-[5px] items-center justify-center"></span>
                            <p class="w-fit text-wrap">{subfile.name}</p>
                        </a>
                    };
                })
                .collect_view();

            let total_children = children_files.len();

            return view! {
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
            .into_any();
        }
        false => view! {}.into_any(),
    }
}
