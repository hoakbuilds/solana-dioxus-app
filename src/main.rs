#![allow(non_snake_case)]
mod components;
mod context;
mod hooks;
mod hot_keys;
pub(crate) mod layout;
mod pages;
mod route;
mod storage;
mod types;
mod utils;

use context::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use solana_wallet_adapter::{Wallet, WALLETS};
use solana_wallet_adapter_dioxus::{use_local_storage, ConnectionProvider, WalletProvider};
use wasm_logger;

use crate::route::Route;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    launch_web(App);
}

pub const DEFAULT_RPC_URL: &'static str = "https://mainnet-beta.solana.com";

#[component]
fn App() -> Element {
    log::info!("App");
    let wallets = WALLETS.to_vec();
    let endpoint =
        use_local_storage::<String>("rpc_url".to_string(), Some(DEFAULT_RPC_URL.to_string()));

    rsx! {
        div {
            class: "flex flex-col min-h-screen justify-between bg-[#13283d]",
            ConnectionProvider {
                endpoint,
                WalletProvider {
                    wallets,
                    auto_connect: false,
                    local_storage_key: None,
                    Router::<Route> { },
                }
            }
        }
    }
}
