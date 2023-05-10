use anchor_lang::solana_program::pubkey::Pubkey;
use dioxus::prelude::*;
use dioxus_router::Link;
use solana_client_wasm::{
    utils::rpc_response::RpcConfirmedTransactionStatusWithSignature, WasmClient,
};

use crate::{context::Cluster, utils::ClockworkWasmClient};

#[derive(Clone, Props, PartialEq)]
pub struct TransactionHistoryTableProps {
    pub address: Pubkey,
}

pub fn TransactionHistoryTable(cx: Scope<TransactionHistoryTableProps>) -> Element {
    let address = cx.props.address;
    let cluster_context = use_shared_state::<Cluster>(cx).unwrap();

    let transaction_data = use_future(cx, (), |_| {
        let cluster_context = cluster_context.clone();
        let client = WasmClient::new_with_config(cluster_context.read().to_owned());
        async move {
            client
                .get_transaction_history(address)
                .await
                .unwrap_or(vec![])
        }
    });

    if let Some(transactions) = transaction_data.value() {
        cx.render(rsx! {
            div {
                h1 {
                    class: "text-2xl font-semibold mb-6",
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
        })
    } else {
        cx.render(rsx! {
            div {
                h1 {
                    class: "text-2xl font-semibold mb-6",
                    "Transactions"
                }
            }
        })
    }
}

fn Header(cx: Scope) -> Element {
    let cell_class =
        "table-cell font-medium py-2 px-5 first:pl-3 first:w-full first:truncate last:pr-3";
    cx.render(rsx! {
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
    })
}

#[derive(PartialEq, Props)]
struct RowProps {
    elem_id: String,
    transaction: RpcConfirmedTransactionStatusWithSignature,
}

fn Row(cx: Scope<RowProps>) -> Element {
    let cell_class = "table-cell whitespace-nowrap font-medium py-2 px-5 first:pl-3 first:truncate last:pr-3 first:rounded-tl first:rounded-bl last:rounded-tr last:rounded-br";
    let result_class = if cx.props.transaction.err.is_some() {
        "whitespace-nowrap text-xs font-sans font-medium py-1 px-2 rounded text-slate-100 bg-red-500"
    } else {
        "whitespace-nowrap text-xs font-sans font-medium py-1 px-2 rounded text-slate-100 bg-green-500"
    };
    let result_str = if cx.props.transaction.err.is_some() {
        "Error"
    } else {
        "Succcess"
    };

    let id = cx.props.transaction.signature.to_string();
    cx.render(rsx! {
        Link {
            to: "/transaction/{id}",
            class: "table-row font-mono text-sm items-start transition hover:cursor-pointer hover:bg-slate-800 active:bg-slate-100 active:text-slate-900",
            td {
                class: cell_class,
                "{cx.props.transaction.signature}"
            }
            td {
                class: cell_class,
                "{cx.props.transaction.slot}"
            }
            td {
                class: cell_class,
                p {
                    class: result_class,
                    "{result_str}"
                }
            }
        }
    })
}
