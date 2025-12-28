use dioxus::prelude::*;
use ui::Echo;

#[component]
pub fn Home() -> Element {
    rsx! {
        Echo {}
    }
}
