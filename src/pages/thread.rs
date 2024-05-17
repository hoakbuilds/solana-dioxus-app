use anchor_lang::solana_program::pubkey::Pubkey;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use solana_client_wasm::{solana_sdk::account::Account, WasmClient};
use std::str::FromStr;

use super::Page;
use crate::{
    client::ClockworkWasmClient,
    components::{
        thread_info_table::ThreadInfoTable, thread_sim_logs::ThreadSimLogs, TransactionHistoryTable,
    },
    context::Cluster,
    Route,
};

#[component]
pub fn ThreadPage(cx: Scope) -> Element {
    let route: Route = use_route(cx).unwrap();
    let thread = use_state::<Option<(String, Account)>>(cx, || None);
    let cluster_context = use_shared_state::<Cluster>(cx).unwrap();

    use_future(cx, (), |_| {
        let thread = thread.clone();
        let cluster_context = cluster_context.clone();
        let thread_pubkey = Pubkey::from_str(route.last_segment().unwrap()).unwrap();
        async move {
            let client = WasmClient::new_with_config(cluster_context.read().to_owned());
            // let t = client.get_thread(thread_pubkey).await.unwrap();
            // thread.set(Some(t));
        }
    });

    if let Some(t) = thread.get() {
        cx.render(rsx! {
            Page {
                div {
                    class: "flex flex-col space-y-16",
                    div {
                        class: "flex flex-col justify-between",
                        h1 {
                             class: "text-2xl font-semibold mb-6",
                             "Thread"
                        }
                        ThreadInfoTable { account: t.clone().1, thread: t.clone().0 }
                    }
                    ThreadSimLogs { thread: t.clone().0 }
                    TransactionHistoryTable { address: t.clone().0 }
                }
            }
        })
    } else {
        cx.render(rsx! {
            Page {
                div {
                    "Loading..."
                }
            }
        })
    }
}
