use std::str::FromStr;

use anchor_lang::solana_program::pubkey::Pubkey;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use solana_client_wasm::{
    solana_sdk::{commitment_config::CommitmentConfig, signature::Signature},
    utils::{
        rpc_config::GetConfirmedSignaturesForAddress2Config,
        rpc_response::RpcConfirmedTransactionStatusWithSignature,
    },
    WasmClient,
};
use solana_wallet_adapter_dioxus::use_connection;

use crate::{context::Cluster, route::Route};

#[derive(Clone, Props, PartialEq)]
pub struct TransactionHistoryTableProps {
    pub address: Pubkey,
}

pub fn TransactionHistoryTable(props: TransactionHistoryTableProps) -> Element {
    let cluster = use_context::<Signal<Cluster>>();
    let address = props.address;

    let transaction_data = use_resource(move || async move {
        let cluster = cluster();
        let connection = use_connection();
        if address != Pubkey::default() {
            connection
                .client
                .get_signatures_for_address_with_config(
                    &address,
                    GetConfirmedSignaturesForAddress2Config {
                        limit: Some(1000),
                        commitment: Some(CommitmentConfig::confirmed()), // Maximum
                        ..Default::default()
                    },
                )
                .await
                .unwrap_or(vec![])
        } else {
            vec![]
        }
    });

    let txs = transaction_data.read().clone();

    if let Some(transactions) = txs {
        rsx! {
            div {
                class: "flex-1 flex flex-col items-center px-1 md:px-2 lg:px-3 border border-blue-200 rounded-2xl p-2 shadow dark:bg-black/[.25]",
                h1 {
                    class: "text-2xl font-mono mb-6",
                    "Transactions"
                }
                table {
                    class: "w-full",
                    Header {}
                    tbody {
                        for transaction in transactions {
                            Row {
                                elem_id: "0".to_string(),
                                transaction: transaction.clone()
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "flex-1 flex flex-col items-center px-1 md:px-2 lg:px-3 border border-blue-200 rounded-2xl p-2 shadow dark:bg-black/[.25]",
                h1 {
                    class: "text-2xl font-mono mb-6",
                    "Transactions"
                }
            }
        }
    }
}

fn Header() -> Element {
    let cell_class =
        "table-cell font-mono py-2 px-5 first:pl-3 first:w-full first:truncate last:pr-3";
    rsx! {
        thead {
            tr {
                class: "table-row text-left text-sm text-slate-500",
                th {
                    class: cell_class,
                    scope: "col",
                    "Signature"
                }
                th {
                    class: cell_class,
                    scope: "col",
                    "Block"
                }
                th {
                    class: cell_class,
                    scope: "col",
                    "Result"
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Props)]
struct RowProps {
    elem_id: String,
    transaction: RpcConfirmedTransactionStatusWithSignature,
}

fn Row(props: RowProps) -> Element {
    let cluster = use_context::<Signal<Cluster>>();
    let id = props.transaction.signature.to_string();
    let cluster_prop = cluster().to_solana_explorer_cluster();
    let cell_class = "table-cell whitespace-nowrap font-medium py-2 px-5 first:pl-3 first:truncate last:pr-3 first:rounded-tl first:rounded-bl last:rounded-tr last:rounded-br";
    let result_class = if props.transaction.err.is_some() {
        "whitespace-nowrap text-xs font-mono font-medium py-1 px-2 rounded text-slate-100 bg-red-500"
    } else {
        "whitespace-nowrap text-xs font-mono font-medium py-1 px-2 rounded text-slate-100 bg-green-500"
    };
    let result_str = if props.transaction.err.is_some() {
        "Error"
    } else {
        "Success"
    };
    rsx! {
        Link {
            to: Route::Transaction {
                signature: Signature::from_str(&id).unwrap_or(Signature::default()),
                query_params: Default::default(),
            },
            new_tab: true,
            class: "table-row font-mono text-sm items-start transition hover:cursor-pointer hover:bg-slate-800 active:bg-slate-100 active:text-slate-900",
            td {
                class: cell_class,
                "{props.transaction.signature}"
            }
            td {
                class: cell_class,
                "{props.transaction.slot}"
            }
            td {
                class: cell_class,
                p {
                    class: result_class,
                    "{result_str}"
                }
            }
        }
    }
}
