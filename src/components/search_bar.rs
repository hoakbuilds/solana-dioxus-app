use crate::{
    context::{
        search::{set_search_state, toggle_active},
        Cluster,
    },
    SearchResult, SearchState,
};
use anchor_lang::prelude::Pubkey;
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_free_icons::prelude::Icon;
use dioxus_router::prelude::*;
use solana_client_wasm::WasmClient;
use solana_wallet_adapter_dioxus::use_connection;
use std::str::FromStr;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[component]
pub fn SearchBar() -> Element {
    log::info!("SearchBar");

    let search_ctx = use_context::<Signal<SearchState>>();
    let query = use_memo(move || search_ctx().query);
    let router = use_navigator();

    // Move the focus to the search bar.
    // autofocus property on input is having issues: https://github.com/DioxusLabs/dioxus/issues/725
    use_future(move || async move {
        gloo_timers::future::TimeoutFuture::new(50).await;
        let document = gloo_utils::document();
        if let Some(element) = document.get_element_by_id("search-bar") {
            element.unchecked_into::<HtmlElement>().focus().ok();
        }
    });

    rsx! {
        input {
            class: "rounded bg-[#0e0e10] text-slate-100 p-5 w-full focus:ring-0 focus:outline-0 text-base",
            id: "search-bar",
            r#type: "text",
            placeholder: "Search",
            value: "{query}",
            oninput: move |e| {
                let query_str = e.data.value().clone().as_str().to_string();
                if query_str.ne(&String::from("/")) {
                    let mut search_state_c = search_ctx().clone();
                    search_state_c.query = query_str;
                    set_search_state(search_state_c);
                }
            },
            onclick: move |e| e.stop_propagation(),
            onkeydown: move |e| {
                if e.key() == Key::Enter {
                    let mut search_state_c = search_ctx().clone();
                    let query = &search_state_c.query;

                    // TODO Select navigation desination from the search results.
                    if let Ok(address) = Pubkey::from_str(&query) {
                        router.push(&*format!("/a/{}", address.to_string()));
                        search_state_c.active = false;
                        search_state_c.query = "".to_string();
                    } else {
                        // TODO Display "invalid address" error to user
                        log::info!("Invalid address");
                    }

                    set_search_state(search_state_c);
                }
            },
        }
    }
}

#[component]
pub fn SearchResults() -> Element {
    let cluster = use_context::<Signal<Cluster>>();
    let search_ctx = use_context::<Signal<SearchState>>();

    // Search for search results.
    let results = use_resource(move || async move {
        let cluster = cluster();
        let query = search_ctx().query;
        log::info!("Parsing query: {:?}", query);
        let connection = use_connection();

        if let Ok(address) = Pubkey::from_str(&query) {
            // Fetch the account
            match connection.client.get_account(&address).await {
                Ok(account) => {
                    vec![SearchResult {
                        title: format!("Go to account {}", address),
                        route: format!("/a/{}", address),
                    }]
                }
                Err(_) => {
                    vec![SearchResult {
                        title: format!("Go to account {}", address),
                        route: format!("/a/{}", address),
                    }]
                }
            }
        } else {
            // TODO Display "invalid address" error to user
            log::info!("Invalid address");
            vec![]
        }
    });

    let results = results.read().clone();

    if let Some(search_results) = results {
        rsx! {
            div {
                class: "flex flex-col w-full",
                for search_result in search_results.iter() {
                    SearchResultRow {
                        result: search_result.clone(),
                    }
                }
            }
        }
    } else {
        None
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct SearchResultRowProps {
    result: SearchResult,
}

pub fn SearchResultRow(props: SearchResultRowProps) -> Element {
    let route = &props.result.route;
    let title = &props.result.title;
    rsx! {
        Link {
            to: route,
            class: "flex flex-row gap-x-2 mx-2 p-3 text-slate-100 transition hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 rounded last:mb-2",
            onclick: move |_| {
                toggle_active()
            },
            svg {
                class: "w-6 h-6",
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M13.5 4.5L21 12m0 0l-7.5 7.5M21 12H3",
                }
            }
            p {
                class: "text-base my-auto",
                "{title}"
            }
        }
    }
}

pub fn SearchButton() -> Element {
    rsx! {
        button {
            class: "rounded-full bg-transparent text-slate-100 transition hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 p-3",
            onclick: move |_| {
                toggle_active();
            },
            Icon {
                width: 16,
                height: 16,
                icon: dioxus_bootstrap_icons::BsSearch
            }
        }
    }
}
