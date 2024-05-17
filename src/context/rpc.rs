use dioxus::prelude::*;
use solana_client_wasm::WasmClient;
use std::rc::Rc;

pub struct RpcContext {
    pub url: String,
    pub client: WasmClient,
}
