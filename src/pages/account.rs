use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use dioxus::prelude::*;
use dioxus_router::use_route;
use solana_client_wasm::{solana_sdk::account::Account, WasmClient};

use super::Page;
use crate::{
    client::ClockworkWasmClient,
    components::{account_info_table::AccountInfoTable, TransactionHistoryTable},
    context::Cluster,
};

pub fn AccountPage(cx: Scope) -> Element {
    let route = use_route(cx);
    let account = use_state::<Option<Account>>(cx, || None);
    let cluster_context = use_shared_state::<Cluster>(cx).unwrap();

    // TODO Unwrap address safely
    let address = Pubkey::from_str(route.last_segment().unwrap()).unwrap();

    use_future(cx, (), |_| {
        let account = account.clone();
        let cluster_context = cluster_context.clone();
        async move {
            let client = WasmClient::new_with_config(cluster_context.read().to_owned());
            match client.get_account(&address).await {
                Ok(maybe_account) => {
                    account.set(Some(maybe_account));
                }
                Err(_err) => {
                    // TODO Handle error
                }
            }
        }
    });

    log::info!("Account: {:?}", account.get());
    cx.render(rsx! {
        Page {
            div {
                class: "flex flex-col",
                h1 {
                     class: "text-2xl font-semibold mb-6",
                     "Account"
                }
                if let Some(account) = account.get() {
                    rsx! {
                        AccountInfoTable {
                            account: account.clone(),
                            address: address,
                        }
                        TransactionHistoryTable { address: address }
                    }
                } else {
                    rsx! {
                        "Loading..."
                    }
                }
            }
        }
    })
}
