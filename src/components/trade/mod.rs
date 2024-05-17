use crate::{
    components::{
        search_bar::{SearchBar, SearchResults},
        swap::{Footer, Provider as SwapProvider, TokenSelector},
        Chart,
    },
    context::{
        search::{toggle_active, SearchState},
        Cluster, User,
    },
    types::user::TokenBalance,
    utils::format_lamports,
};
use anchor_lang::{prelude::Pubkey, solana_program::native_token::lamports_to_sol};
use dioxus::prelude::*;
use dioxus_free_icons::prelude::*;
use serde_json::Value;
use solana_client_wasm::{utils::rpc_filter::TokenAccountsFilter, WasmClient};
use solana_extra_wasm::{
    account_decoder::UiAccountData,
    program::spl_token::{self, native_mint},
};
use solana_wallet_adapter_dioxus::use_connection;
use std::str::FromStr;

#[component]
pub fn TradeComponent(display_chart: Signal<bool>) -> Element {
    log::info!("TradeComponent");

    let user_ctx = use_context::<Signal<User>>();
    let cluster_ctx = use_context::<Signal<Cluster>>();
    let search_ctx = use_context::<Signal<SearchState>>();

    let mut swap_provider = use_signal(|| SwapProvider::default());

    let mut native_balance = use_signal(|| user_ctx().lamports_display());
    let mut search_state = use_signal(|| search_ctx());
    let mut query = use_signal(|| search_state().query);
    let mut disable_input = use_signal(|| false);

    native_balance.set(format_lamports(
        use_resource(move || async move {
            if let User::Wallet { pubkey, .. } = user_ctx() {
                if let Some(owner) = pubkey {
                    let cluster = cluster_ctx();
                    log::info!("Using cluster: {:?}", cluster);

                    let connection = use_connection();

                    match connection.client.get_balance(&owner).await {
                        Ok(res) => res,
                        Err(e) => {
                            log::error!("Error fetching balance: {:?}", e);
                            0
                        }
                    }
                } else {
                    0
                }
            } else {
                0
            }
        })
        .read()
        .unwrap_or(0),
        true,
    ));

    let mut token_balances = use_signal(|| vec![]);

    token_balances.set(
        use_resource(move || async move {
            if let User::Wallet { pubkey, lamports } = user_ctx() {
                if let Some(owner) = pubkey {
                    let mut token_account_balances = vec![];
                    let cluster = cluster_ctx();
                    log::info!("Using cluster: {:?}", cluster);

                    let connection = use_connection();

                    let token_accounts = match connection
                        .client
                        .get_token_accounts_by_owner(
                            &owner,
                            TokenAccountsFilter::ProgramId(spl_token::ID),
                        )
                        .await
                    {
                        Ok(res) => res,
                        Err(e) => {
                            log::error!("Error fetching token accounts: {:?}", e);
                            return vec![];
                        }
                    };

                    log::info!("Found {} token accounts.", token_accounts.len());

                    for keyed_account in &token_accounts {
                        match &keyed_account.account.data {
                            UiAccountData::Json(parsed) => {
                                if let Value::Object(object) = &parsed.parsed {
                                    let mut token_balance = TokenBalance {
                                        account: Pubkey::from_str(&keyed_account.pubkey).unwrap(),
                                        ..Default::default()
                                    };

                                    if let Some(Value::Object(object)) = object.get("info") {
                                        if let Some(Value::String(mint)) = object.get("mint") {
                                            token_balance.mint = Pubkey::from_str(&mint).unwrap();
                                            if token_balance.mint == native_mint::ID {
                                                token_balance.symbol = "SOL".to_string();
                                            }
                                            if token_balance.mint
                                                == Pubkey::from_str(
                                                    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
                                                )
                                                .unwrap()
                                            {
                                                token_balance.symbol = "USDC".to_string();
                                            }
                                        }
                                    }

                                    if let Some(Value::Object(object)) = object.get("tokenAmount") {
                                        if let Some(Value::String(amount)) = object.get("amount") {
                                            token_balance.balance_native =
                                                u64::from_str(&amount).unwrap();
                                            if token_balance.mint == native_mint::ID {
                                                token_balance.balance_native += lamports;
                                            }
                                        }
                                        if let Some(Value::String(amount)) =
                                            object.get("uiAmountString")
                                        {
                                            token_balance.balance = f64::from_str(&amount).unwrap();
                                            if token_balance.mint == native_mint::ID {
                                                token_balance.balance +=
                                                    user_ctx().lamports_float();
                                            }
                                        }
                                        if let Some(Value::String(decimals)) =
                                            object.get("decimals")
                                        {
                                            token_balance.decimals =
                                                u8::from_str(decimals).unwrap();
                                        }
                                    }
                                    log::debug!("Token Balance: {:?}", token_balance);
                                    token_account_balances.push(token_balance);
                                }
                            }
                            _ => (),
                        };
                    }

                    if let None = token_account_balances
                        .iter()
                        .find(|tb| tb.mint == native_mint::ID)
                    {
                        let token_balance = TokenBalance {
                            account: owner,
                            mint: native_mint::ID,
                            symbol: "SOL".to_string(),
                            balance_native: lamports,
                            balance: lamports_to_sol(lamports),
                            decimals: 9,
                        };
                        token_account_balances.push(token_balance);
                    }

                    token_account_balances
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        })
        .read()
        .clone()
        .unwrap_or(vec![]),
    );

    let selected_source_token =
        use_signal(|| "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string());
    let selected_dest_token = use_signal(|| native_mint::ID.to_string());

    rsx! {
        div {
            class: "flex-1 flex flex-col items-center px-1 md:px-2 lg:px-3 border border-blue-200 rounded-2xl p-2 shadow dark:bg-black/[.25]",
            div {
                class: "mt-2 w-[100%] max-w-full px-1 md:px-2 lg:px-3",
                div {
                    class: "flex justify-end mb-2 px-1 md:px-2 lg:px-3",
                    div {
                        class: "flex flex-row w-full",
                        div {
                            class: "flex flex-row space-x-1",
                            button {
                                class: "bg-transparent cursor-pointer text-[10px] text-white border-black-10 leading-4 px-2 rounded-lg border hover:bg-[#13283d]",
                                Icon {
                                    width: 14,
                                    height: 14,
                                    icon: dioxus_bootstrap_icons::BsArrowClockwise,
                                }
                            }
                            button {
                                class: "bg-transparent cursor-pointer text-[10px] text-white border-black-10 leading-4 px-2 rounded-lg border hover:bg-[#13283d] ",
                                onclick: move |_| {
                                    display_chart.set(!display_chart());
                                },
                                Icon {
                                    width: 14,
                                    height: 14,
                                    icon: dioxus_bootstrap_icons::BsGraphUp,
                                },
                            }
                        }
                        div {
                            class: "ml-auto flex flex-row space-x-1",
                            button {
                                class: "bg-transparent cursor-pointer text-[10px] text-white border-black-10 leading-4 px-2 rounded-lg border hover:bg-[#13283d]",
                                div {
                                    class: "flex flex-row items-center m-1",
                                    Icon {
                                        width: 14,
                                        height: 14,
                                        icon: dioxus_bootstrap_icons::BsSliders,
                                    }
                                    span {
                                        class: "text p-1 text-xs font-small whitespace-nowrap",
                                        "0.5%"
                                    }
                                }
                            }
                            button {
                                class: "bg-transparent cursor-pointer text-[10px] text-white border-black-10 leading-4 px-2 rounded-lg border hover:bg-[#13283d] ",
                                Icon {
                                    width: 14,
                                    height: 14,
                                    icon: dioxus_bootstrap_icons::BsGearFill,
                                },
                            }
                        }
                    }
                }
                div {
                    class: "border border-gray-200 rounded-2xl p-2 shadow dark:bg-gray-900 dark:border-gray-500 px-1 md:px-2 lg:px-3",
                    form {
                        div {
                            class: "flex-col space-y-2 relative px-1 py-1 md:px-2 md:py-2 lg:px-3 lg:py-3",
                            TokenSelector {
                                title: "Source",
                                token_balances,
                                selected_token: selected_source_token,
                                disable_input,
                                display_amount_helpers: true,
                            }
                            div {
                                class: "relative flex justify-center",
                                hr {
                                    class: "absolute w-full border dark:border-[rgba(25,35,45,0.35)] top-[calc(50%-1px)]"
                                }
                                div {
                                    class: "inline-block z-10",
                                    button {
                                        class: "cursor-pointer text-[10px] text-white border-black-10 leading-4 px-2 rounded-full border hover:bg-[#13283d]",
                                        onclick: move |e| {},
                                        Icon {
                                            width: 14,
                                            height: 14,
                                            icon: dioxus_bootstrap_icons::BsArrowDownUp
                                        }
                                    }
                                }
                            }
                            TokenSelector {
                                title: "Destination",
                                token_balances,
                                selected_token: selected_dest_token,
                                disable_input,
                                display_amount_helpers: false,
                            }
                        }
                        div { class: "mt-3 flex gap-x-1" }
                        div { class: "flex flex-col space-y-2" }
                        Footer {}
                    }
                }

            }
        }
        if search_ctx().active {
            div {
                onclick: move |_| {
                    toggle_active()
                },
                class: "absolute top-0 left-0 w-screen h-screen backdrop-opacity-10 bg-white/10 transition content-center flex flex-col z-10",

                div {
                    class: "max-w-3xl w-full mx-auto mt-40 bg-[#0e0e10] p-1 flex flex-col rounded drop-shadow-md",
                    SearchBar {}
                    SearchResults {}
                }
            }
        }
    }
}
