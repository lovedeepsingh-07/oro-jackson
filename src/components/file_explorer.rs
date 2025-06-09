use crate::helpers;
use leptos::prelude::*;

#[component]
pub fn TreeFile(name: String, href: String) -> impl IntoView {
    view! {
        <li>
            <a href={href}>
                {name}
            </a>
        </li>
    }
}

#[component]
pub fn TreeFolder(name: String, href: String, children: Children) -> impl IntoView {
    view! {
        <li>
            <details>
                <summary><a href={href}>{name}</a></summary>
                <ul>
                    {children()}
                </ul>
            </details>
        </li>
    }
}

pub fn render_file_tree(file_tree_input: Vec<helpers::file_tree::TreeNode>) -> Vec<AnyView> {
    let mut res: Vec<AnyView> = Vec::new();
    for tree_node in file_tree_input {
        match tree_node {
            helpers::file_tree::TreeNode::File(curr_file) => res.push(
                view! {
                    <TreeFile name={curr_file.name} href={curr_file.href}/>
                }
                .into_any(),
            ),
            helpers::file_tree::TreeNode::Folder(curr_folder) => {
                let tree_children = render_file_tree(curr_folder.children);
                res.push(
                    view! {
                        <TreeFolder name={curr_folder.name} href={curr_folder.href}>
                            {tree_children}
                        </TreeFolder>
                    }
                    .into_any(),
                );
            }
        }
    }
    return res.collect_view();
}

#[component]
pub fn FileExplorer(
    children: Children,
    project_title: String,
    file_tree: Vec<helpers::file_tree::TreeNode>,
) -> impl IntoView {
    view! {
        <div class="drawer lg:drawer-open">
            <input id="oj-file-explorer" type="checkbox" class="drawer-toggle" />
            <div class="drawer-content">
                {children()}
            </div>
            <div class="drawer-side z-[100]">
                <label for="oj-file-explorer" aria-label="close sidebar" class="drawer-overlay"></label>
                <div class="bg-base-200 text-base-content relative min-h-full w-[320px] lg:flex lg:flex-col">
                    <label
                        for="oj-file-explorer"
                        aria-label="close sidebar"
                        class="btn btn-square btn-ghost drawer-button stroke-base-content hover:bg-base-300 absolute top-[5px] right-[5px] lg:hidden"
                    >
                        <svg
                            class="size-[1.5rem]"
                            viewBox="0 0 24 24"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                            ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
                                id="SVGRepo_tracerCarrier"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            ></g><g id="SVGRepo_iconCarrier">
                                <path
                                    d="M13 5V19M16 8H18M16 11H18M16 14H18M6.2 19H17.8C18.9201 19 19.4802 19 19.908 18.782C20.2843 18.5903 20.5903 18.2843 20.782 17.908C21 17.4802 21 16.9201 21 15.8V8.2C21 7.0799 21 6.51984 20.782 6.09202C20.5903 5.71569 20.2843 5.40973 19.908 5.21799C19.4802 5 18.9201 5 17.8 5H6.2C5.0799 5 4.51984 5 4.09202 5.21799C3.71569 5.40973 3.40973 5.71569 3.21799 6.09202C3 6.51984 3 7.07989 3 8.2V15.8C3 16.9201 3 17.4802 3.21799 17.908C3.40973 18.2843 3.71569 18.5903 4.09202 18.782C4.51984 19 5.07989 19 6.2 19Z"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                ></path>
                            </g></svg
                        >
                    </label>
                    <div class="mt-[32px] flex w-full justify-start px-4">
                        <a href="/" class="text-accent drawer-button max-w-[260px] text-2xl font-bold text-wrap">
                            {project_title}
                        </a>
                    </div>
                    <ul class="menu max-w-xs w-full">
                        {render_file_tree(file_tree)}
                    </ul>
                </div>
            </div>
        </div>
    }
}
