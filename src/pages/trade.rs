use anchor_lang::prelude::Pubkey;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use solana_client_wasm::{solana_sdk::account::Account, WasmClient};
use solana_wallet_adapter_dioxus::use_connection;
use std::str::FromStr;

use crate::{
    components::{
        account_info_table::AccountInfoTable, Chart, TradeComponent, TransactionHistoryTable,
    },
    context::{Cluster, User},
    types::QuerySegments,
};

#[component]
pub fn Trade(from: Pubkey, to: Pubkey, amount: f64, query_params: QuerySegments) -> Element {
    log::info!("Trade Page");
    let cluster = use_context::<Signal<Cluster>>();
    let user_ctx = use_context::<Signal<User>>();
    let cluster_ctx = use_context::<Signal<Cluster>>();

    let display_chart = use_signal(|| false);

    let account = use_resource(move || async move {
        let cluster = cluster();
        if let User::Wallet { pubkey, .. } = user_ctx() {
            if let Some(address) = pubkey {
                log::info!("Fetching account: {}", address);
                let connection = use_connection();
                match connection.client.get_account(&address).await {
                    Ok(maybe_account) => Some(maybe_account),
                    Err(e) => {
                        // TODO Handle error
                        log::error!("{:?}", e);
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    });

    log::info!("Account: {:?}", account.value());

    rsx! {
        div {
            class: "flex-1 flex flex-row items-center px-1 md:px-2",
            div {
                class: "flex-1 lg:flex xl:flex xs:flex-col sm:flex-col md:flex-row lg:flex-row xl:flex-row items-center px-1",
                div {
                    class: "flex-1 flex flex-col items-center px-1 py-1 md:py-2 md:px-2",
                    TradeComponent { display_chart, }
                }
                if display_chart() {
                    div {
                        class: "flex-1 flex flex-col items-center px-1 py-1 md:py-2 md:px-2",
                        Chart { }
                    }
                }
            }
        }
    }
}
