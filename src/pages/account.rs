use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use solana_client_wasm::{solana_sdk::account::Account, WasmClient};
use solana_wallet_adapter_dioxus::use_connection;

use crate::{
    components::{account_info_table::AccountInfoTable, TransactionHistoryTable},
    context::Cluster,
    types::QuerySegments,
};

#[component]
pub fn AccountPage(address: Pubkey, query_params: QuerySegments) -> Element {
    let cluster = use_context::<Signal<Cluster>>();

    let account = use_resource(move || async move {
        let cluster = cluster();
        let address = address.clone();
        let connection = use_connection();
        match connection.client.get_account(&address).await {
            Ok(maybe_account) => Some(maybe_account),
            Err(e) => {
                // TODO Handle error
                log::error!("{:?}", e);
                None
            }
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
                    div {
                        class: "flex-1 flex flex-col items-center px-1 md:px-2 lg:px-3 border border-blue-200 rounded-2xl p-2 shadow dark:bg-black/[.25]",
                        h1 {
                            class: "text-2xl font-mono mb-6",
                            "Account"
                        }
                        if let Some(account) = account() {
                            if let Some(account) = account {
                                AccountInfoTable {
                                    account: account.clone(),
                                    address,
                                }

                            } else {
                                "Something went wrong.. :("
                            }
                        } else {
                            "Loading..."
                        }
                    }
                    TransactionHistoryTable { address }
                }
            }
        }
    }
}
