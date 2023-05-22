use dioxus::prelude::*;

#[derive(Props)]
pub struct PageProps<'a> {
    children: Element<'a>,
}

pub fn Page<'a>(cx: Scope<'a, PageProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full flex flex-row justify-around",
            div {
                class: "w-full max-w-7xl p-8",
                &cx.props.children
            }
        }
    })
}
