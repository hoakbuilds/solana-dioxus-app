use dioxus::prelude::*;

#[component]
pub fn NotFoundPage(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Not found" }
    }
}
