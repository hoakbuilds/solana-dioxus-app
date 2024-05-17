use anchor_lang::prelude::Pubkey;
use dioxus::prelude::*;
use dioxus_free_icons::prelude::Icon;
use dioxus_router::prelude::*;
use gloo_events::EventListener;
use gloo_storage::{LocalStorage, Storage};
use solana_wallet_adapter::{prelude::*, IntoWalletIcon};
use solana_wallet_adapter_dioxus::{use_local_storage, use_wallet, WalletContextState};
use std::str::FromStr;

use crate::{
    context::{set_cluster, Cluster, User},
    DEFAULT_RPC_URL,
};

#[component]
pub fn ConnectButton() -> Element {
    log::info!("ConnectButton");

    let mut show_wallet_popover = use_signal(|| false);
    let mut show_wallet_select_dropdown = use_signal(|| false);

    let WalletContextState {
        wallets,
        connected,
        wallet,
        pubkey,
        ..
    } = use_wallet();
    let mut wallet_name: Signal<Option<String>> = use_context();
    let mut user_ctx = use_context::<Signal<User>>();
    let cluster_ctx = use_context::<Signal<Cluster>>();
    let event_listeners = use_signal(|| vec![]);

    let endpoint =
        use_local_storage::<String>("rpc_url".to_string(), Some(DEFAULT_RPC_URL.to_string()));

    log::info!("User: {:?}", user_ctx());

    let handle_wallet_click = move |_| {
        let value = *show_wallet_select_dropdown.peek();
        show_wallet_select_dropdown.set(!value);
        show_wallet_popover.set(false);
    };

    // Wallet connect flow and set popover state
    let handle_click = move |_| {
        show_wallet_select_dropdown.set(false);

        spawn(async move {
            log::info!("Wallet Name: {:?} ", wallet_name());

            if let Some(wallet) = wallet() {
                log::info!("Wallet {:?}", wallet);

                // If the wallet is already connected
                if wallet.is_connected() {
                    log::info!("Wallet already connected!");

                    // Set wallet connected
                    if !*connected.read() {
                        let pubkey = wallet.pubkey();
                        log::info!("Connected Wallet: {}", pubkey);

                        if let User::Wallet {
                            pubkey: user_pubkey,
                            lamports,
                        } = user_ctx()
                        {
                            // If there's already a user in the app ctx
                            log::info!("User Context: {:?} Lamports {}", user_pubkey, lamports);
                            // If the user in the app ctx is different from the connected wallet
                            if let Some(user_pubkey) = user_pubkey {
                                if user_pubkey != pubkey {
                                    user_ctx.set(User::Wallet {
                                        pubkey: Some(pubkey),
                                        lamports: 0,
                                    });
                                }
                            }
                        } else {
                            // There's no user in the app ctx, set it
                            user_ctx.set(User::Wallet {
                                pubkey: Some(pubkey),
                                lamports: 0,
                            });
                        }
                    } else {
                        // Negat
                        let value = !*show_wallet_popover.peek();
                        show_wallet_popover.set(value);
                    }
                } else {
                    log::info!(
                        "Wallet not connected. Attempting to connect with wallet provider.."
                    );
                    wallet.connect(Some(&endpoint())).await;
                }
            } else {
                log::info!("Wallet not selected!");
            }
        });
    };

    // Wallet disconnect
    let handle_disconnect_click = {
        move |_| {
            spawn(async move {
                log::info!("Attempting to disconnect with wallet provider..");
                log::info!("WalletName: {:?} ", wallet_name());

                if let Some(wallet) = wallet() {
                    // If the wallet is already connected
                    if connected() {
                        // Disable display of popover and dropdown
                        show_wallet_popover.set(false);
                        show_wallet_select_dropdown.set(false);
                        // Reset the user context
                        user_ctx.set(User::Unknown);
                        //wallet_connected.set(false);
                        wallet.disconnect().await;
                    } else {
                        log::info!("Wallet not connected..");
                    }
                } else {
                    log::info!("Adapter not selected!");
                }
            });
        }
    };

    let wallets_c = wallets.clone();
    use_effect(move || {
        let mut event_listeners = event_listeners.clone();

        if !show_wallet_select_dropdown() {
            // Clear dynamic button display event listeners
            if !event_listeners.peek().is_empty() {
                event_listeners.clear();
            }
        } else {
            // Add dynamic button display event listeners
            let wallets_iter = wallets_c.clone();
            let document = gloo_utils::document();

            for wallet in wallets_iter {
                if let Some(button) = document.get_element_by_id(&wallet.into_wallet_name()) {
                    let el = EventListener::new(&button, "click", move |_| {
                        let wallet = wallet.clone();
                        wallet_name.set(Some(wallet.into_wallet_name().to_string()));
                        show_wallet_select_dropdown.set(false);
                    });
                    event_listeners.push(el);
                }
            }
        }
    });

    let connect_text = if connected() {
        if let Some(owner) = pubkey().as_ref() {
            let string = owner.to_string();
            let initial = &string[..4];
            let end = &string[string.len().saturating_sub(4)..];
            format!("{}..{}", initial, end)
        } else {
            String::from("Connect")
        }
    } else if let Some(_) = wallet_name() {
        String::from("Connect")
    } else {
        String::from("Connect Wallet")
    };

    let address_long_form = if let User::Wallet { pubkey, .. } = user_ctx() {
        if let Some(owner) = pubkey {
            let string = owner.to_string();
            let initial = &string[..8];
            let end = &string[string.len().saturating_sub(8)..];
            format!("{}..{}", initial, end)
        } else {
            String::from("")
        }
    } else {
        String::from("")
    };

    let current_cluster_text: String = cluster_ctx()
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

    log::info!("Current cluster: {}", current_cluster_text);
    log::info!("Connected: {}", *connected.peek());

    rsx! {
        if let Some(wallet) = wallet() {
            div {
                class: "w-full flex flex-row items-center justify-between space-x-8",
                Balance { token_mint: None },
                if connected() {
                    button {
                        class: "cursor-pointer flex items-center h-8 py-2 px-3 z-50 border rounded-full transition text-slate-100 hover:bg-slate-800 active:bg-slate-100 active:text-slate-900",
                        onclick: handle_click,
                        div {
                            class: "flex flex-row items-center",
                                img {
                                    class: "px-1 rounded",
                                    // Only peek at the value here, to avoid multiple subscriptions to the same signal
                                    src: wallet.icon(),
                                    height: 32,
                                    width: 32,
                                }
                                span {
                                    class: "font-mono text-sm px-1",
                                    "{connect_text}"
                                }
                                Icon {
                                    width: 15,
                                    height: 15,
                                    icon: dioxus_bootstrap_icons::BsChevronDown
                                }
                        }
                    }
                } else {
                    if wallet_name().is_some() {
                        // At this point, wallet_state should also be set
                        div {
                            class: "cursor-pointer flex items-center h-8 py-1 px-1 border rounded-full transition text-slate-100 hover:bg-slate-800 active:bg-slate-100 active:text-slate-900",
                            button {
                                class: "cursor-pointer flex items-center py-1 px-1 rounded-full transition text-slate-100",
                                onclick: handle_click,
                                div {
                                    class: "flex flex-row items-center",
                                    img {
                                        class: "px-1 rounded",
                                        src: wallet.icon(),
                                        height: 32,
                                        width: 32,
                                    }
                                    span {
                                        class: "font-mono text-sm px-1",
                                        "{connect_text}"
                                    }
                                }
                            }
                            button {
                                class: "cursor-pointer flex items-center py-1 px-1 rounded-full transition text-slate-100",
                                onclick: handle_wallet_click,
                                div {
                                    class: "flex flex-row items-center",
                                    Icon {
                                        width: 15,
                                        height: 15,
                                        icon: dioxus_bootstrap_icons::BsChevronDown
                                    }
                                }
                            }
                        }
                    } else {
                        span {
                            class: "font-mono text-sm",
                            "{connect_text}"
                        }
                    }
                }
            }
        }

        if show_wallet_popover() {
            div {
                class: "z-50 dark:bg-gray-900 absolute top-[40px] lg:top-[46px] right-[0px] flex px-1 md:px-2 lg:px-3 items-center shadow-xl rounded-lg transition-all",
                div {
                    class: "z-50 px-4 py-5 sm:p-6 space-y-2 w-full",
                    div {
                        class: "z-50 w-full flex flex-row items-center justify-between space-x-8",
                        span {
                            class: "font-mono text-sm px-1",
                            "{address_long_form}"
                        }
                        Balance { token_mint: None }
                    }
                    div {
                        class: "z-50 w-full flex flex-col items-center justify-between",
                        button {
                            class: "cursor-pointer flex items-center h-8 px-1 py-1 sm:px-2 sm:py-2 sm:px-3 sm:py-3 z-50 border rounded-full transition text-slate-100 hover:bg-slate-800 active:bg-slate-100 active:text-slate-900",
                            onclick: handle_disconnect_click,
                            "Disconnect"
                        }
                    }
                }
            }
        }

        if show_wallet_select_dropdown() {
            div {
                class: "z-50 dark:bg-gray-900 absolute top-[40px] lg:top-[46px] right-[0px] flex px-2 lg:px-3 items-center shadow-xl rounded-lg transition-all",
                ul {
                    class: "z-50 px-2 lg:px-3 py-2 lg:py-3 origin-top-right items-center overflow-hidden rounded-lg shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none",
                    for wallet in wallets.iter() {
                        button {
                            class: "cursor-pointer flex items-center h-8 m-1 px-1 py-1 sm:px-2 sm:py-2 sm:px-3 sm:py-3 z-100 w-full border rounded-full transition text-slate-100 hover:bg-slate-800 active:bg-slate-100 active:text-slate-900",
                            id: "{wallet.into_wallet_name()}",
                            div {
                                class: "flex flex-row items-center",
                                img {
                                    class: "px-1 rounded",
                                    src: wallet.into_wallet_icon(),
                                    height: 24,
                                    width: 24,
                                }
                                span {
                                    class: "font-mono px-1 text-sm",
                                    "{wallet.into_wallet_name()}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Balance(token_mint: Option<Pubkey>) -> Element {
    let user_ctx = use_context::<Signal<User>>();

    if let User::Unknown = user_ctx() {
        return None;
    }

    let user_balance = if let Some(mint) = token_mint {
        user_ctx().lamports_display()
    } else {
        user_ctx().lamports_display()
    };

    rsx! {
        div {
            class: "font-mono text-sm flex whitespace-nowrap",
            "{user_balance}"
        }
    }
}

fn update_cluster_and_navigate(
    path: &str,
    cluster_query_param: String,
    router: &Navigator,
    custom_rpc_url: String,
) {
    let cluster_from_query_param =
        get_cluster_from_query_param(cluster_query_param, custom_rpc_url.clone());
    log::info!("Cluster from query param: {:?}", cluster_from_query_param);
    let cached_cluster = get_cached_cluster();
    log::info!("Cached cluster: {:?}", cached_cluster);

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
    log::info!("Using cluster: {:?}", cluster_to_use);

    update_cluster_context_and_cache(cluster_to_use.clone(), custom_rpc_url.clone());

    navigate_to_route_with_cluster(path, router, &cluster_to_use.to_string(), custom_rpc_url);
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

fn update_cluster_context_and_cache(cluster: Cluster, custom_rpc_url: String) {
    log::info!("updating cluster cache to: {}", cluster.to_string());
    set_cluster(cluster.clone());
    if cluster.to_string() == "custom" {
        LocalStorage::set("customUrl", custom_rpc_url).unwrap();
    } else {
        LocalStorage::set("customUrl", "http://localhost:8899").unwrap();
    }
}

fn navigate_to_route_with_cluster(
    path: &str,
    router: &Navigator,
    cluster_query_param: &str,
    custom_rpc_url: String,
) {
    if cluster_query_param.eq("custom") {
        router.push(
            format!(
                "{}?cluster={}&customUrl={}",
                path,
                cluster_query_param,
                urlencoding::encode(custom_rpc_url.as_str())
            )
            .as_str(),
        );
    } else {
        router.push(format!("{}?cluster={}", path, cluster_query_param).as_str());
    }
}
