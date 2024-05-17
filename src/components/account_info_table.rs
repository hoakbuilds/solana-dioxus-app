use anchor_lang::prelude::Pubkey;
use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::account::Account;

use crate::utils::format_lamports;

// #[derive(PartialEq, Clone, Props)]
// pub struct AccountInfoTableProps {
//     account: Account,
//     address: Pubkey,
// }

#[component]
pub fn AccountInfoTable(account: Account, address: Pubkey) -> Element {
    let balance = format_lamports(account.lamports, false);
    let executable = account.executable;
    let owner = account.owner;
    rsx! {
        table {
            class: "w-full divide-y divide-slate-800",
            tbody {
                Row {
                    label: "Address".to_string(),
                    value: address.to_string()
                }
                Row {
                    label: "Balance".to_string(),
                    value: balance,
                }
                Row {
                    label: "Executable".to_string(),
                    value: executable.to_string(),
                }
                Row {
                    label: "Owner".to_string(),
                    value: owner.to_string(),
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
                class: "table-cell whitespace-nowrap px-1 py-1 md:py-2 md:px-2 text-sm text-slate-500",
                span {
                    class: "font-mono text-medium",
                    "{props.label}"
                }
            }
            div {
                class: "table-cell whitespace-nowrap px-1 py-1 md:py-2 md:px-2 text-sm font-mono text-slate-100",
                span {
                    class: "font-mono text-sm",
                    "{props.value}"
                }
            }
        }
    }
}
