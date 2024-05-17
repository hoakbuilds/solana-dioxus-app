use dioxus::prelude::*;
use dioxus_router::prelude::*;
use solana_client_wasm::{
    solana_sdk::{commitment_config::CommitmentConfig, signature::Signature},
    utils::rpc_config::RpcTransactionConfig,
    WasmClient,
};
use solana_extra_wasm::transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding,
};
use solana_wallet_adapter_dioxus::use_connection;
use std::str::FromStr;

use crate::{
    components::TransactionInfo,
    context::{use_cluster, Cluster},
    route::Route,
    types::QuerySegments,
};

#[component]
pub fn TransactionPage(signature: Signature, query_params: QuerySegments) -> Element {
    let cluster = use_cluster();
    let mut transaction = use_signal::<Option<EncodedConfirmedTransactionWithStatusMeta>>(|| None);

    use_future(move || async move {
        let cluster = cluster();
        let signature = signature.clone();
        if signature != Signature::default() {
            let connection = use_connection();
            let t = connection
                .client
                .get_transaction_with_config(
                    &signature,
                    RpcTransactionConfig {
                        encoding: Some(UiTransactionEncoding::JsonParsed),
                        commitment: Some(CommitmentConfig::confirmed()),
                        max_supported_transaction_version: Some(1),
                    },
                )
                .await
                .unwrap();
            *transaction.write() = Some(t);
        } else {
            *transaction.write() = None;
        }
    });

    let tx = transaction.read().clone();

    if let Some(t) = tx {
        rsx! {
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
    } else {
        rsx! {
            div {
                "Loading..."
            }
        }
    }
}
