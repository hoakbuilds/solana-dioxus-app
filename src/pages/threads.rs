use dioxus::prelude::*;

//use crate::components::ThreadsTable;

#[component]
pub fn ThreadsPage() -> Element {
    rsx! {
        h1 {
            class: "text-2xl font-semibold mb-6",
            "Threads"
        }
        //ThreadsTable {}
    }
}
