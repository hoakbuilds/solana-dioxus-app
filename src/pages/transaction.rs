use dioxus::prelude::*;
use dioxus_router::use_route;
use solana_client_wasm::{solana_sdk::signature::Signature, WasmClient};
use solana_extra_wasm::transaction_status::EncodedConfirmedTransactionWithStatusMeta;
use std::str::FromStr;

use crate::{
    components::TransactionInfo, context::Cluster, pages::page::Page, utils::ClockworkWasmClient,
};

pub fn TransactionPage(cx: Scope) -> Element {
    let route = use_route(cx);
    let transaction = use_state::<Option<EncodedConfirmedTransactionWithStatusMeta>>(cx, || None);
    let cluster_context = use_shared_state::<Cluster>(cx).unwrap();
    use_future(cx, (), |_| {
        let transaction = transaction.clone();
        let cluster_context = cluster_context.clone();
        let transaction_signature = Signature::from_str(route.last_segment().unwrap()).unwrap();
        async move {
            let client = WasmClient::new_with_config(cluster_context.read().to_owned());
            let t = client
                .get_account_transaction(&transaction_signature)
                .await
                .unwrap();
            transaction.set(Some(t));
        }
    });

    if let Some(t) = transaction.get() {
        cx.render(rsx! {
            Page {
                div {
                    class: "flex flex-col space-y-16",
                    div {
                        class: "flex flex-col justify-between",
                        h1 {
                             class: "text-2xl font-semibold mb-6",
                             "Transaction"
                        }
                        TransactionInfo { data: t.clone() }
                    }
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
