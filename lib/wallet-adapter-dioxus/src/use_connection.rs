use dioxus::prelude::*;
use solana_wallet_adapter_base::Connection;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ConnectionContextState {
    pub connection: Connection,
}

pub fn use_connection() -> Connection {
    return use_context();
}
