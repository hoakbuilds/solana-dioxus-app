use dioxus::prelude::*;
use dioxus_router::prelude::*;
use gloo_events::EventListener;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::context::search::toggle_active;

#[component]
pub fn HotKeys() -> Element {
    let router = use_navigator();

    use_future(move || async move {
        let router = router.clone();
        let document = gloo_utils::document();
        let mut goto_mode = false;

        Some(EventListener::new(&document, "keydown", move |event| {
            let document = gloo_utils::document();
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();

            match (event.modifiers(), event.key().as_str()) {
                (modifiers, "/") => {
                    if let Some(element) = document.active_element() {
                        // condition to prevent triggering search when user typing '/' in "https://..."
                        if element.id().as_str().ne("custom_rpc_input") {
                            toggle_active();
                        }
                    }
                }
                (modifiers, "Space") => {
                    log::debug!("Space key pressed, checking modifiers.");
                    if modifiers.ctrl() {
                        log::debug!("Alt modifier pressed..");
                        toggle_active();
                    }
                }
                (modifiers, "Escape") => toggle_active(),
                (modifiers, "H" | "h") => {
                    if goto_mode {
                        router.push("/");
                        goto_mode = false;
                    }
                }
                // "J" | "j" => {
                //     goto_mode = false;
                //     let id = list_index.map_or(0, |i| i + 1);
                //     let elem_id = format!("list-item-{}", id);
                //     if let Some(element) = document.get_element_by_id(&*elem_id) {
                //         if element.unchecked_into::<HtmlElement>().focus().is_ok() {
                //             list_index = Some(id);
                //         }
                //     }
                // }
                // "K" | "k" => {
                //     goto_mode = false;
                //     let id = list_index.map_or(0, |i| i.saturating_sub(1));
                //     let elem_id = format!("list-item-{}", id);
                //     if let Some(element) = document.get_element_by_id(&*elem_id) {
                //         if element.unchecked_into::<HtmlElement>().focus().is_ok() {
                //             list_index = Some(id);
                //         }
                //     }
                // }
                _ => {}
            }
        }))
    });
    rsx! {
        div {}
    }
}
