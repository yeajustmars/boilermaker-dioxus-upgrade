use dioxus::prelude::*;

use ui::{Layout, Navbar, DROPDOWN_LINK_STYLE, INDENTED_DROPDOWN_LINK_STYLE};
use views::{Blog, Home};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(DesktopLayout)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn DesktopLayout() -> Element {
    rsx! {
        Layout {
            Navbar {
                Link { class: DROPDOWN_LINK_STYLE, to: Route::Home {},
                    i { class: "fa-solid fa-house" }
                    span { class: "ml-2", "Home" }
                }
                /*
                Link { class: DROPDOWN_LINK_STYLE, to: Route::Templates {},
                    i { class: "fa-solid fa-note-sticky" }
                    span { class: "ml-2", "Templates" }
                }
                Link {
                    class: INDENTED_DROPDOWN_LINK_STYLE,
                    to: Route::TemplateAdd {},
                    i { class: "fa-solid fa-plus" }
                    span { class: "ml-2", "Add Template" }
                }
                Link { class: DROPDOWN_LINK_STYLE, to: Route::Docs {},
                    i { class: "fa-solid fa-file-code" }
                    span { class: "ml-2", "Docs" }
                }
                Link { class: DROPDOWN_LINK_STYLE, to: Route::GetInvolved {},
                    i { class: "fa-solid fa-hands-helping" }
                    span { class: "ml-2", "Get Involved" }
                }
                 */
            }
            Outlet::<Route> {}
        }
    }
}
