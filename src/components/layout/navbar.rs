use anchor_lang::prelude::Pubkey;
use dioxus::prelude::*;
use dioxus_free_icons::prelude::Icon;
use dioxus_router::prelude::*;
use solana_wallet_adapter_dioxus::{use_wallet, WalletContextState};

use crate::{
    components::{ConnectButton, SearchButton},
    route::Route,
};

#[component]
pub fn Navbar(route: Route) -> Element {
    log::info!("Navbar");

    let WalletContextState {
        connected, pubkey, ..
    } = use_wallet();

    let address = use_memo(move || {
        if let Some(pk) = pubkey() {
            pk
        } else {
            Pubkey::default()
        }
    });

    rsx! {
        div {
            class: "dark:bg-gray-900 flex items-center lg:justify-between h-[48px] lg:h-[60px] w-full px-2.5 lg:px-5",
            div {
                class: "flex lg:hidden items-center flex-1",
                Logo { with_lettering: false, }
                div {
                    class: "ml-2.5",
                    Link {
                        to: Route::Trade { from: Default::default(), to: Default::default(), amount: Default::default(), query_params: Default::default()},
                        button {
                            class: "dark:bg-gray-800 flex items-center transition hover:cursor-pointer hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 font-mono justify-center fill-current h-[48px] lg:h-[60px] px-2 lg:px-4 text-xs lg:text-sm bg-[#192531] dark:-bg[#192531]/[0.25]",
                            "Trade"
                        }
                    }
                }
            }
            div {
                class: "hidden lg:flex items-center flex-1",
                Logo { with_lettering: true,}
            }
            div {
                class: "hidden lg:block",
                Link {
                    to: Route::Trade { from: Default::default(), to: Default::default(), amount: Default::default(), query_params: Default::default()},
                    button {
                        class: "dark:bg-gray-800 flex items-center transition hover:cursor-pointer hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 font-mono justify-center fill-current h-[48px] lg:h-[60px] px-2 lg:px-4 text-xs lg:text-sm bg-[#192531] dark:-bg[#192531]/[0.25]",
                        "Trade"
                    }
                }
            }
            div {
                class: "flex items-center justify-end flex-1 space-x-2",
                div {
                    class: "flex items-center space-x-2",
                }
                div {
                    // Requires usage of Z due to components blocking access to the
                    class: "flex items-center space-x-2 z-50",
                    SearchButton {},
                }
                div {
                    class: "relative",
                    Link {
                        to: Route::Account { address: address(), query_params: Default::default()},
                        button {
                            class: "rounded-full bg-transparent text-slate-100 transition hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 p-3",
                            Icon {
                                width: 16,
                                height: 16,
                                icon: dioxus_bootstrap_icons::BsPersonFill
                            }
                        }
                    }
                }
                div {
                    class: "flex items-center space-x-2",
                    ConnectButton {}
                }
            }
        }
    }
}

#[component]
pub fn Logo(with_lettering: bool) -> Element {
    rsx! {
        Link {
            to: "/",
            class: "flex-shrink-0",
            h1 {
                class: "flex items-center text-base font-semibold dark:text-white",
                img {
                    width: "32",
                    height: "32",
                    src: "/img/hammer-anvil.png",
                }
                if with_lettering {
                    span {
                        class: "flex text-md font-mono ml-[10px]",
                        "Anvil"
                    }
                }
            }
        }
    }
}
