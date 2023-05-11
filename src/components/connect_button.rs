use clockwork_utils::pubkey::Abbreviated;
use dioxus::prelude::*;
use dioxus_router::{use_route, use_router, RouterService};
use gloo_events::EventListener;
use gloo_storage::{LocalStorage, Storage};
use solana_client_wasm::{solana_sdk::pubkey::Pubkey, WasmClient};
use std::{rc::Rc, str::FromStr};
use url::Url;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use super::backpack::backpack;
use crate::{
    context::{Cluster, User},
    utils::{format_balance, ClockworkWasmClient},
};

pub fn ConnectButton(cx: Scope) -> Element {
    let route = use_route(cx);
    let router = use_router(cx);
    let user_context = use_shared_state::<User>(cx).unwrap();
    let cluster_context = use_shared_state::<Cluster>(cx).unwrap();
    let show_popover = use_state(cx, || false);
    let show_cluster_dropdown = use_state(cx, || false);
    let custom_rpc_url = use_ref(cx, || {
       if let Some(custom_url_query_param) = route.query_param("customUrl") {
            if let Ok(url) = LocalStorage::get::<String>("customUrl") {
                if custom_url_query_param.clone().ne(&url) {
                    custom_url_query_param.to_string()
                } else {
                    url
                }
            } else {
                "http://localhost:8899".to_string()
            }
        } else {
                "http://localhost:8899".to_string()
        }
    });
    let is_custom_rpc_input_focused = use_ref(cx, || false);
    let cluster_query_param = use_state(cx, || {
        route
            .query_param("cluster")
            .unwrap_or(std::borrow::Cow::Borrowed(""))
            .into_owned()
    });

    let path = Url::parse(route.url().as_ref()).unwrap().path().to_string();

    // validate custom rpc url else default to localhost
    use_future!(cx, |(cluster_context,)| {
        let custom_rpc_url = custom_rpc_url.clone();
        to_owned![router];

        async move {
            let client = WasmClient::new_with_config(Cluster::Custom(custom_rpc_url.read().clone()));
            if client.get_slot().await.is_err() && custom_rpc_url.read().ne(&"http://localhost:8899") {
                update_cluster_and_navigate(
                    "custom".to_string(),
                    cluster_context,
                    &router,
                    "http://localhost:8899".to_string(),
                );
                let _ = web_sys::window().unwrap().location().reload();
            } 
        }
    });

    // handle cluster config with query param and local storage
    use_effect(cx, (&path,), |_| {
        let user_context = user_context.clone();
        let cluster_context = cluster_context.clone();
        let cluster_query_param = cluster_query_param.clone();
        let custom_rpc_url = custom_rpc_url.clone();
        to_owned![router];

        async move {
            if let Ok(user) = LocalStorage::get::<User>("user_context") {
                log::info!("user is logged in!");
                // TODO: refetch user account data/ask user to log in after a certain timeframe
                let mut uc_write = user_context.write();
                uc_write.account = user.account;
                uc_write.pubkey = user.pubkey;
            }
            update_cluster_and_navigate(
                cluster_query_param.to_string(),
                cluster_context,
                &router.clone(),
                custom_rpc_url.read().clone(),
            );
        }
    });

    // wallet connect flow and set popover state
    let handle_click = move |_| {
        cx.spawn({
            let user_context = user_context.clone();
            let cluster_context = cluster_context.clone();
            let show_popover = show_popover.clone();
            let cluster_context = cluster_context;
            async move {
                let user_context_read = user_context.read();
                let cluster_context = cluster_context.read();
                match user_context_read.account.is_some() {
                    true => {
                        show_popover.set(!*show_popover.get());
                    }
                    _ => {
                        // Check if the provider is not connected before connecting
                        if !backpack.is_connected() {
                            backpack.connect().await;
                        }
                        if backpack.is_connected() {
                            let client = WasmClient::new_with_config(cluster_context.to_owned());
                            let pubkey =
                                Pubkey::from_str(backpack.pubkey().to_string().as_str()).unwrap();
                            let account = client.get_account(&pubkey).await;
                            match account {
                                Ok(acc) => {
                                    drop(user_context_read);
                                    user_context.write().account = Some(acc);
                                    user_context.write().pubkey = Some(pubkey);
                                    LocalStorage::set(
                                        "user_context",
                                        User {
                                            pubkey: user_context.read().pubkey,
                                            account: user_context.read().account.clone(),
                                        },
                                    )
                                    .unwrap();
                                    LocalStorage::set(
                                        "cluster",
                                        cluster_context.to_string().to_lowercase(),
                                    )
                                    .unwrap();
                                }

                                Err(err) => log::info!("Failed to get user account: {:?}", err),
                            }
                        }
                    }
                }
            }
        });
    };

    // handle cluster change from dropdown and if rpc url input focused
    use_future(cx, (), |_| {
        let cluster_context = cluster_context.clone();
        let show_cluster_dropdown = show_cluster_dropdown.clone();
        let custom_rpc_url = custom_rpc_url.clone();
        let is_custom_rpc_input_focused = is_custom_rpc_input_focused.clone();
        to_owned![router];

        async move {
            let document = gloo_utils::document();
            EventListener::new(&document, "click", move |_| {
                let document = gloo_utils::document();
                if let Some(element) = document.active_element() {
                    let element_id = element.id();
                    let e_id = element_id.as_str();
                    match e_id {
                        "mainnet" | "devnet" | "custom" => {
                            let cluster = Cluster::from_str(e_id).unwrap();
                            update_cluster_and_navigate(
                                cluster.to_string(),
                                cluster_context.clone(),
                                &router,
                                custom_rpc_url.read().clone(),
                            );
                            let _ = web_sys::window().unwrap().location().reload();
                            show_cluster_dropdown.set(false);
                        }
                        "custom_rpc_input" => {
                            *is_custom_rpc_input_focused.write() = true;
                        }
                        _ => {}
                    }
                }
            })
        }
    });

    // handle for custom rpc url change and navigate
    use_future(cx, (), |_| {
        let is_custom_rpc_input_focused = is_custom_rpc_input_focused.clone();
        let cluster_context = cluster_context.clone();
        let custom_rpc_url = custom_rpc_url.clone();
        to_owned![router];

        async move {
            let document = gloo_utils::document();
            EventListener::new(&document, "keydown", move |event| {
                let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                if event.key().as_str() == "Enter"
                    && *is_custom_rpc_input_focused.read()
                    && custom_rpc_url.read().ne(&cluster_context.read().url())
                {
                    update_cluster_and_navigate(
                        "custom".to_string(),
                        cluster_context.clone(),
                        &router,
                        custom_rpc_url.read().clone(),
                    );
                    // trigger refresh with new custom url
                    let _ = web_sys::window().unwrap().location().reload();
                }
            })
        }
    });

    let connect_text = if let Some(pubkey) = user_context.read().pubkey {
        pubkey.abbreviated()
    } else {
        String::from("Connect")
    };

    let current_cluster_text: String = cluster_context
        .read()
        .to_string()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect();

    cx.render(rsx! {
        button {
            class: "px-6 py-3 border rounded-full transition text-slate-100 hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 font-semibold",
            onclick: handle_click,
            connect_text.as_str()
        }
        if *show_popover.get() {
            rsx! {
                div {
                    class: "bg-slate-800 absolute top-[90px] right-[34px] w-72 flex flex-col items-center shadow-xl sm:rounded-lg transition-all",
                    div {
                        class: "px-4 py-5 sm:p-6 space-y-2 w-full",
                        div {
                            class: "w-full flex flex-row items-center justify-between space-x-8",
                            p {
                                class: "text-slate-100 font-semibold",
                                "{connect_text.as_str()}"
                            }
                            Balance {}
                        }
                        button {
                            class: "inline-flex w-full justify-center gap-x-1.5 rounded-md bg-white hover:bg-slate-100 px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50",
                            onclick: move |_| { show_cluster_dropdown.set(!show_cluster_dropdown) },
                            "{current_cluster_text}"
                            svg {
                                class: "-mr-1 h-5 w-5 text-slate-800", 
                                view_box: "0 0 20 20",
                                fill: "currentColor",
                                path {
                                    fill_rule: "evenodd", 
                                    d: "M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z", 
                                }
                            }
                        }
                        if *show_cluster_dropdown.get() {
                            rsx! {
                                 ul { class:"z-10 mt-2 w-full origin-top-right divide-y divide-gray-200 overflow-hidden rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none",
                                    button {
                                        class:"text-gray-900 w-full cursor-default select-none p-4 text-sm hover:bg-slate-200",
                                        id: "mainnet",
                                        div {
                                            class: "flex flex-col",
                                            div {
                                                class: "flex justify-between",
                                                p {
                                                    class: "text-normal",
                                                    "Mainnet"
                                                }
                                            }
                                        }
                                    }
                                    button {
                                        class:"text-gray-900 w-full cursor-default select-none p-4 text-sm hover:bg-slate-200",
                                        id: "devnet",
                                        div {
                                            class: "flex flex-col",
                                            div {
                                                class: "flex justify-between",
                                                p {
                                                    class: "text-normal",
                                                    "Devnet"
                                                }
                                            }
                                        }
                                    }
                                    button {
                                        class:"text-gray-900 w-full cursor-default select-none p-4 text-sm hover:bg-slate-200",
                                        id: "custom",
                                        div {
                                            class: "flex flex-col",
                                            div {
                                                class: "flex justify-between",
                                                p {
                                                    class: "text-normal",
                                                    "Custom"
                                                }
                                            }
                                        }
                                    }
                                    if cluster_context.read().to_string().eq("custom") {
                                        rsx! {
                                            input {
                                                class: "block w-full rounded-md border-b focus:ring-0 focus:outline-0 px-4 py-2 text-gray-900 shadow-sm sm:text-sm sm:leading-6",
                                                r#type: "text",
                                                value: "{custom_rpc_url.read()}",
                                                id: "custom_rpc_input",
                                                oninput: move |e| { 
                                                    custom_rpc_url.set(e.value.clone());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

fn Balance(cx: Scope) -> Element {
    let user_context = use_shared_state::<User>(cx).unwrap();

    let user_balance = if let Some(account) = &user_context.read().account {
        format_balance(account.lamports, true)
    } else {
        String::from("")
    };

    cx.render(rsx! {
        div {
            class: "text-lg",
            user_balance
        }
    })
}

fn update_cluster_and_navigate(
    cluster_query_param: String,
    cluster_context: UseSharedState<Cluster>,
    router: &Rc<RouterService>,
    custom_rpc_url: String,
) {
    let cluster_from_query_param =
        get_cluster_from_query_param(cluster_query_param, custom_rpc_url.clone());
    let cached_cluster = get_cached_cluster();

    let cluster_to_use = cluster_from_query_param
        .or(cached_cluster
            .as_ref()
            .map(|s| {
                if s.eq("custom") {
                    Ok(Cluster::Custom(custom_rpc_url.clone()))
                } else {
                    Cluster::from_str(s)
                }
            })
            .transpose()
            .unwrap())
        .unwrap_or_else(|| Cluster::from_str("mainnet").unwrap());

    update_cluster_context_and_cache(
        cluster_context,
        cluster_to_use.clone(),
        custom_rpc_url.clone(),
    );

    navigate_to_route_with_cluster(
        router,
        &cluster_to_use.to_string(),
        custom_rpc_url,
    );
}

fn get_cluster_from_query_param(query_param: String, custom_rpc_url: String) -> Option<Cluster> {
    if query_param.eq("devnet") || query_param.eq("mainnet") {
        Cluster::from_str(&query_param).ok()
    } else if query_param.eq("custom") {
        Some(Cluster::Custom(custom_rpc_url))
    } else {
        None
    }
}

fn get_cached_cluster() -> Option<String> {
    LocalStorage::get::<String>("cluster").ok()
}

fn update_cluster_context_and_cache(
    cluster_context: UseSharedState<Cluster>,
    cluster: Cluster,
    custom_rpc_url: String,
) {
    log::info!("updating cluster cache to: {}", cluster.to_string());
    *cluster_context.write() = cluster.clone();
    LocalStorage::set("cluster", cluster.to_string().to_lowercase()).unwrap();
    if cluster.to_string() == "custom" {
        LocalStorage::set("customUrl", custom_rpc_url).unwrap();
    } else {
        LocalStorage::set("customUrl", "http://localhost:8899").unwrap();
    }
}

fn navigate_to_route_with_cluster(
    router: &Rc<RouterService>,
    cluster_query_param: &str,
    custom_rpc_url: String,
) {
    let path = router.current_location().url.path().to_string();

    if cluster_query_param.eq("custom") {
        router.replace_route(       
        format!("{}?cluster={}&customUrl={}", path, cluster_query_param, urlencoding::encode(custom_rpc_url.as_str())).as_str(),
        None,
        None)
    } else {
        router.replace_route(format!("{}?cluster={}", path, cluster_query_param).as_str(), None, None)
    }
}
