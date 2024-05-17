pub mod settings;

pub use settings::*;

#[derive(Debug, Clone)]
pub enum Provider {
    Jupiter,
    Raydium, // money printer inc innit
}

impl Default for Provider {
    fn default() -> Self {
        Self::Raydium
    }
}

use crate::{types::user::TokenBalance, utils::format_token_amount};
use dioxus::prelude::*;
use dioxus_free_icons::prelude::*;

#[component]
pub fn TokenSelector(
    title: String,
    token_balances: Signal<Vec<TokenBalance>>,
    selected_token: Signal<String>,
    disable_input: Signal<bool>,
    display_amount_helpers: bool,
) -> Element {
    let selected_token_balance = use_memo(move || {
        token_balances()
            .iter()
            .map(|tb| tb.clone())
            .find(|tb| tb.mint.to_string() == selected_token() || tb.symbol == selected_token())
            .unwrap_or_default()
    });

    log::info!("{} {} {:?}", title, selected_token, selected_token_balance);

    let token_balance = format_token_amount(selected_token_balance().balance);
    let token_symbol = selected_token_balance().symbol;

    rsx! {
        div {
            class: "flex flex-col px-1",
            div {
                class: "flex flex-row justify-between px-1",
                label {
                    class: "text-xs sm:text-sm font-medium text-black/90 dark:text-white px-1",
                    "{title}"
                }
                div {
                    class: "flex space-x-2",
                    div {
                        class: "flex space-x-1 items-center text-black-50 dark:text-[#CFF3FF] dark:text-opacity-[0.35]",
                        Icon {
                            width: 16,
                            height: 16,
                            icon: dioxus_bootstrap_icons::BsPiggyBankFill
                        }
                        div {
                            class: "text-xs font-small whitespace-nowrap",
                            span {
                                class: "p-1",
                                "{token_balance}"
                            }
                            span {
                                class: "p-1",
                                "{token_symbol}"
                            }
                        }
                    }
                    if display_amount_helpers {
                        div {
                            class: "flex justify-between items-center space-x-1",
                            button {
                                class: "cursor-pointer text-[10px] text-black-35 border-black-10 leading-4 py-[1px] px-2 rounded-lg border hover:bg-[#2C3F54]",
                                onclick: move |e| {},
                                "HALF"
                            }
                            button {
                                class: "cursor-pointer text-[10px] text-black-35 border-black-10 leading-4 py-[1px] px-2 rounded-lg border hover:bg-[#2C3F54]",
                                onclick: move |e| {},
                                "MAX"
                            }
                        }
                    }
                }
            }
            div {
                class: "flex justify-between",
                TokenAmountInput {
                    title,
                    token_balances,
                    selected_token,
                    disable_input
                }
            }
        }
    }
}

#[component]
fn TokenAmountInput(
    title: String,
    token_balances: Signal<Vec<TokenBalance>>,
    selected_token: Signal<String>,
    disable_input: Signal<bool>,
) -> Element {
    let mut show_dropdown = use_signal(|| false);

    let selected_token_balance = use_memo(move || {
        token_balances()
            .iter()
            .map(|tb| tb.clone())
            .find(|tb| tb.mint.to_string() == selected_token() || tb.symbol == selected_token())
            .unwrap_or_default()
    });

    log::info!("{} {} {:?}", title, selected_token, selected_token_balance);

    let selected_token = selected_token_balance();

    rsx! {
        div {
            class: "p-1 h-[72px] relative rounded-xl flex flex-col space-y-3 group bg-dark",
            div {
                class: "relative",
                div {
                    class: "flex justify-between items-center group/select absolute inset-y-0 start-0 flex items-center",
                    if selected_token != Default::default() {
                        button {
                            class: "py-2 px-1 md:px-2 lg:px-3 h-10 rounded-xl flex space-x-3 items-center text-black-35 hover:bg-[#2C3F54]",
                            onclick: move |_| {
                                let show_dropdown_c = show_dropdown.read().clone();
                                show_dropdown.set(!show_dropdown_c);
                            },
                            div {
                                class: "w-6 h-6 flex items-center justify-center rounded-full",
                                img {
                                    class: "rounded-full",
                                    src: format!("https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/{}/logo.png", selected_token.mint),
                                }
                            }
                            div {
                                class: "text-sm font-medium whitespace-nowrap",
                                "{selected_token.symbol}"
                            }
                            Icon {
                                width: 16,
                                height: 16,
                                icon: dioxus_bootstrap_icons::BsChevronDown
                            }
                        }
                    }
                }
                span {
                    class: "flex-1 text-right h-[24px]",
                    div {
                        class: "flex justify-between items-center group/select",
                        input {
                            class: "h-full w-full p-3 text-xl text-right text-gray-900 border border-gray-300 rounded-2xl bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-700 dark:focus:border-blue-800",
                            inputmode: "decimal",
                            autocomplete: "off",
                            disabled: disable_input(),
                            name: format!("{}Value", title),
                            "data-lpignore": true,
                            placeholder: 0.00,
                            "type": "text",
                            oninput: move |e| {},
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
        button {
            class: "h-full w-full rounded-xl relative inline-flex items-center justify-center mt-3 overflow-hidden text-sm font-medium text-gray-900 group bg-gradient-to-br from-cyan-500 to-blue-500 hover:from-cyan-500 hover:to-blue-500 hover:text-white dark:text-white focus:ring-4 focus:outline-none focus:ring-cyan-200 dark:focus:ring-cyan-800",
            onclick: move |e| {},
            span {
                class: "rounded-xl text-lg font-medium bg-clip-text bg-none py-5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 hover:bg-opacity-0 leading-none",
                "Confirm"
            }
        }
    }
}
