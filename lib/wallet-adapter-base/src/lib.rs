pub mod adapter;
pub mod errors;

pub use errors::*;

use async_trait::async_trait;
use solana_client_wasm::{solana_sdk::pubkey::Pubkey, WasmClient};

/// Events fired by the wallet adapter.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub enum Event {
    #[default]
    Default,
    Connect(Pubkey),
    Disconnect,
    Error(WalletError),
    ReadyStateChange(WalletReadyState),
}

/// An owned wallet name.
pub struct WalletName(String);

impl WalletName {
    pub fn new<T: Into<String>>(w: T) -> Self {
        Self(w.into())
    }
}

impl From<String> for WalletName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for WalletName {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum WalletReadyState {
    Installed,
    NotDetected,
    Loadable,
    Unsupported,
}

impl Default for WalletReadyState {
    fn default() -> Self {
        Self::Unsupported
    }
}

#[async_trait(?Send)]
pub trait WalletAdapterMetadata: std::fmt::Debug {
    fn name(&self) -> &str;
    fn url(&self) -> &str;
    fn icon(&self) -> &str;
}

#[async_trait(?Send)]
pub trait WalletAdapterBase: WalletAdapterMetadata {
    fn is_name(&self) -> bool;
    fn is_connected(&self) -> bool;

    fn ready_state(&self) -> WalletReadyState;
    
    fn pubkey(&self) -> Option<Pubkey>;
    fn connecting(&self) -> bool;

    async fn connect(&self, rpc_endpoint: Option<&str>);
    async fn disconnect(&self);
}

#[async_trait(?Send)]
pub trait WalletAdapter: WalletAdapterBase {
    fn priority_fees_supported(&self) -> bool;

    async fn on_event(&self, f: &mut dyn FnMut(Event));
}

impl PartialEq for dyn WalletAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }

    fn ne(&self, other: &Self) -> bool {
        self.name() != other.name()
    }
}

/// A connection to the cluster.
pub struct Connection {
    pub(crate) endpoint: String,
    pub client: WasmClient,
}

impl Connection {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            client: WasmClient::new(&endpoint),
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint
    }

    fn ne(&self, other: &Self) -> bool {
        self.endpoint != other.endpoint
    }
}

impl std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection")
            .field("endpoint", &format!("{}", self.endpoint))
            .finish()
    }
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            endpoint: String::new(),
            client: WasmClient::new(""),
        }
    }
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        Self {
            endpoint: self.endpoint.clone(),
            client: WasmClient::new(&self.endpoint),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.endpoint = source.endpoint.clone();
        self.client = WasmClient::new(&source.endpoint);
    }
}
