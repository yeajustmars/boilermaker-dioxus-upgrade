use dioxus::prelude::*;

pub const DROPDOWN_LINK_STYLE: &str =
    "block px-4 py-2 hover:bg-neutral-100 dark:hover:bg-neutral-700";
pub const INDENTED_DROPDOWN_LINK_STYLE: &str =
    "block px-8 py-2 hover:bg-neutral-100 dark:hover:bg-neutral-700";

static MAIN_DROPDOWN_OPEN_STATE: GlobalSignal<bool> = Signal::global(|| false);

fn close_main_dropdown() {
    *MAIN_DROPDOWN_OPEN_STATE.write() = false;
}

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        div {
            id: "navbar",
            class: "flex flex-row space-x-4 p-2 items-center justify-between bg-gradient-to-b from-white to-neutral-100 dark:from-neutral-800 dark:to-neutral-900 border-b border-solid border-neutral-300 dark:border-neutral-950 text-neutral-600 dark:text-neutral-300",
            onmouseleave: move |_| close_main_dropdown(),

            div { class: "w-1/4 text-2xl",
                MainNavDropdownMenu {
                    {children}
                }
                //MainLinks {}
            }
            //div { class: "w-1/2", DesktopSearch {} }
            //div { class: "w-1/4 text-right", MainSettings {} }
        }
    }
}

/*
#[component]
fn MainLinks() -> Element {
    rsx! {
        span { class: "ml-2 text-lg",
            Link { to: Route::Home {}, "Boilermaker" }
            img { class: "inline h-6 w-6 mr-1", src: FAVICON }
        }
    }
}
 */

#[component]
fn MainNavDropdownMenu(children: Element) -> Element {
    let is_open = *MAIN_DROPDOWN_OPEN_STATE.read();

    rsx! {
        span {
            class: "pr-2",
            onclick: move |_| {
                *MAIN_DROPDOWN_OPEN_STATE.write() = !is_open;
            },
            i { class: "fa-solid fa-bars" }
        }
        if is_open {
            div {
                onclick: move |_| close_main_dropdown(),
                onmouseleave: move |_| close_main_dropdown(),
                class: "absolute left-0 top-12 w-48 bg-white dark:bg-neutral-900 rounded shadow-lg border border-l-0 border-t-0 border-neutral-300 dark:border-neutral-700 z-10 text-sm ",
                {children}
            }
        }
    }
}
