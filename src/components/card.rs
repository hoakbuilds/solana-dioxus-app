use dioxus::prelude::*;

#[component]
pub fn Card(title: String, content: Element) -> Element {
    rsx! {
        div {
            class: "w-full max-w-sm p-4 bg-white border border-gray-200 rounded-lg shadow sm:p-6 md:p-8 dark:bg-gray-800 dark:border-gray-700",
            h5 {
                class: "text-xl justify-center font-medium text-gray-900 dark:text-white",
                "{title}"
            }
            {content}
        }
    }
}
