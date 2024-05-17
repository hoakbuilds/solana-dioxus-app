use dioxus::prelude::*;
use solana_extra_wasm::transaction_status::EncodedConfirmedTransactionWithStatusMeta;

use crate::utils::{format_lamports, format_timestamp};

#[derive(Clone, Props, PartialEq)]
pub struct TransactionInfoProps {
    pub data: EncodedConfirmedTransactionWithStatusMeta,
}

pub fn TransactionInfo(props: TransactionInfoProps) -> Element {
    let slot = props.data.slot.to_string();
    let time_stamp = format_timestamp(props.data.block_time.unwrap());
    let fee = format_lamports(props.data.transaction.meta.as_ref().unwrap().fee, false);
    let error = props
        .data
        .transaction
        .meta
        .as_ref()
        .and_then(|meta| meta.err.as_ref().map(|err| err.to_string()))
        .unwrap_or_else(String::new);
    let signature = props
        .data
        .transaction
        .transaction
        .decode()
        .unwrap()
        .signatures[0]
        .to_string();
    let status = if error == "" { "Success" } else { "Error" }.to_string();

    rsx! {
        table {
            class: "w-full divide-y divide-slate-800",
            tbody {
                Row {
                    label: "Signature".to_string(),
                    value: signature
                }
                Row {
                    label: "Result".to_string(),
                    value: status,
                }
                Row {
                    label: "Error".to_string(),
                    value: error,
                }
                Row {
                    label: "Timestamp".to_string(),
                    value: time_stamp,
                }
                Row {
                    label: "Slot".to_string(),
                    value: slot,
                }
                Row {
                    label: "Fee (SOL)".to_string(),
                    value: fee,
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
struct RowProps {
    label: String,
    value: String,
}

fn Row(props: RowProps) -> Element {
    rsx! {
        div {
            class: "flex justify-between",
            id: props.label.as_str(),
            div {
                class: "table-cell whitespace-nowrap px-2 py-2 text-sm text-slate-500",
                "{props.label}"
            }
            div {
                class: "table-cell whitespace-nowrap px-2 py-2 text-sm font-mono text-slate-100",
                "{props.value}"
            }
        }
    }
}
