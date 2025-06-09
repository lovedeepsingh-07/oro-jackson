use leptos::prelude::*;

#[component]
pub fn ThemeButton() -> impl IntoView {
    view! {
        <label class="swap swap-rotate btn btn-square btn-ghost hover:bg-base-300 stroke-base-content">
            <input type="checkbox" id="oj-theme-toggle-checkbox" />

            <svg
                class="swap-off size-[1.5rem]"
                viewBox="0 0 24 24"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
                    id="SVGRepo_tracerCarrier"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                ></g><g id="SVGRepo_iconCarrier">
                    <path
                        d="M12 3V4M12 20V21M4 12H3M6.31412 6.31412L5.5 5.5M17.6859 6.31412L18.5 5.5M6.31412 17.69L5.5 18.5001M17.6859 17.69L18.5 18.5001M21 12H20M16 12C16 14.2091 14.2091 16 12 16C9.79086 16 8 14.2091 8 12C8 9.79086 9.79086 8 12 8C14.2091 8 16 9.79086 16 12Z"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    ></path>
                </g></svg
            >

            <svg
                class="swap-on size-[1.5rem]"
                viewBox="0 0 24 24"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
                    id="SVGRepo_tracerCarrier"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                ></g><g id="SVGRepo_iconCarrier">
                    <path
                        d="M3.32031 11.6835C3.32031 16.6541 7.34975 20.6835 12.3203 20.6835C16.1075 20.6835 19.3483 18.3443 20.6768 15.032C19.6402 15.4486 18.5059 15.6834 17.3203 15.6834C12.3497 15.6834 8.32031 11.654 8.32031 6.68342C8.32031 5.50338 8.55165 4.36259 8.96453 3.32996C5.65605 4.66028 3.32031 7.89912 3.32031 11.6835Z"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    ></path>
                </g></svg
            >
        </label>
    }
}

#[component]
pub fn Navbar(show_file_explorer: bool) -> impl IntoView {
    match show_file_explorer {
        true => view! {
            <div class="fixed top-0 right-0 left-0 z-[80] flex items-center justify-between px-3 py-[10px] backdrop-blur-sm lg:justify-end">
                <label
                    for="oj-file-explorer"
                    class="btn btn-square btn-ghost drawer-button hover:bg-base-300 stroke-base-content lg:hidden"
                >
                    <svg class="size-[1.5rem]" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"
                        ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
                            id="SVGRepo_tracerCarrier"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        ></g><g id="SVGRepo_iconCarrier">
                            <path
                                d="M11 5V19M6 8H8M6 11H8M6 14H8M6.2 19H17.8C18.9201 19 19.4802 19 19.908 18.782C20.2843 18.5903 20.5903 18.2843 20.782 17.908C21 17.4802 21 16.9201 21 15.8V8.2C21 7.0799 21 6.51984 20.782 6.09202C20.5903 5.71569 20.2843 5.40973 19.908 5.21799C19.4802 5 18.9201 5 17.8 5H6.2C5.0799 5 4.51984 5 4.09202 5.21799C3.71569 5.40973 3.40973 5.71569 3.21799 6.09202C3 6.51984 3 7.07989 3 8.2V15.8C3 16.9201 3 17.4802 3.21799 17.908C3.40973 18.2843 3.71569 18.5903 4.09202 18.782C4.51984 19 5.07989 19 6.2 19Z"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            ></path>
                        </g></svg
                    >
                </label>
                <ThemeButton/>
            </div>
        }.into_any(),
        false => view! {
            <div class="fixed top-0 right-0 left-0 z-[80] flex items-center justify-end px-3 py-[10px] backdrop-blur-sm">
                <ThemeButton/>
            </div>
        }.into_any(),
    }
}
