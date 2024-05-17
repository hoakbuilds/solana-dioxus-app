use std::sync::Arc;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_wallet_adapter::Wallet;
use solana_wallet_adapter_base::{WalletAdapter, WalletReadyState};

pub struct WalletState {
    pub(crate) adapter: Option<Arc<dyn WalletAdapter>>,
}

impl WalletState {
    pub fn name(&self) -> String {
        self.adapter
            .as_ref()
            .map_or("".to_string(), |a| a.name().to_string())
    }

    pub fn icon(&self) -> String {
        self.adapter
            .as_ref()
            .map_or("".to_string(), |a| a.icon().to_string())
    }

    pub fn url(&self) -> String {
        self.adapter
            .as_ref()
            .map_or("".to_string(), |a| a.url().to_string())
    }

    pub fn ready_state(&self) -> WalletReadyState {
        self.adapter
            .as_ref()
            .map_or(WalletReadyState::Unsupported, |a| a.ready_state())
    }

    pub fn pubkey(&self) -> Pubkey {
        self.adapter
            .as_ref()
            .map_or(Pubkey::default(), |a| a.pubkey().unwrap_or_default())
    }

    pub fn is_connected(&self) -> bool {
        self.adapter.as_ref().map_or(false, |a| a.is_connected())
    }

    pub async fn connect(&self, rpc_endpoint: Option<&str>) {
        if let Some(adapter) = &self.adapter {
            adapter.connect(rpc_endpoint).await
        }
    }

    pub async fn disconnect(&self) {
        if let Some(adapter) = &self.adapter {
            adapter.disconnect().await
        }
    }
}

impl Clone for WalletState {
    fn clone(&self) -> Self {
        let adapter = self.adapter.as_ref().map_or(None, |a| Some(a));
        log::info!(
            "clone -> name: {:?}",
            adapter.as_ref().map_or("", |w| w.name())
        );
        log::info!(
            "clone -> isWalletProvider: {:?}",
            adapter
                .as_ref()
                .map_or(String::new(), |w| format!("{:?}", w.is_name()))
        );
        log::info!(
            "clone -> priorityFeesSupported: {:?}",
            adapter.as_ref().map_or(String::new(), |w| format!(
                "{:?}",
                w.priority_fees_supported()
            ))
        );
        Self {
            adapter: self.adapter.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        let adapter = source.adapter.as_ref().map_or(None, |a| Some(a.clone()));
        log::info!(
            "clone_from -> name: {:?}",
            adapter
                .as_ref()
                .map_or("".to_string(), |w| w.name().to_string())
        );
        self.adapter = source.adapter.clone();
    }
}

impl Default for WalletState {
    fn default() -> Self {
        Self { adapter: None }
    }
}

impl std::fmt::Debug for WalletState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WalletState")
            .field("adapter", &format!("{:?}", self.adapter))
            .finish()
    }
}

impl PartialEq for WalletState {
    fn eq(&self, other: &Self) -> bool {
        self.adapter
            .as_ref()
            .map_or("".to_string(), |a| a.name().to_string())
            != other
                .adapter
                .as_ref()
                .map_or("".to_string(), |a| a.name().to_string())
    }
}

pub struct WalletContextState {
    pub auto_connect: bool,
    pub wallets: Vec<Wallet>,
    pub wallet: Signal<Option<WalletState>>,
    pub pubkey: Signal<Option<Pubkey>>,
    pub connecting: Signal<bool>,
    pub connected: Signal<bool>,
    pub disconnecting: Signal<bool>,
}

impl Clone for WalletContextState {
    fn clone(&self) -> Self {
        Self {
            auto_connect: self.auto_connect.clone(),
            wallets: self.wallets.clone(),
            wallet: self.wallet.clone(),
            pubkey: self.pubkey.clone(),
            connected: self.connected.clone(),
            connecting: self.connecting.clone(),
            disconnecting: self.disconnecting.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.auto_connect = source.auto_connect.clone();
        self.wallets = source.wallets.clone();
        self.wallet = source.wallet.clone();
        self.pubkey = source.pubkey.clone();
        self.connected = source.connected.clone();
        self.connecting = source.connecting.clone();
        self.disconnecting = source.disconnecting.clone();
    }
}

impl Default for WalletContextState {
    fn default() -> Self {
        Self {
            auto_connect: false,
            wallets: vec![],
            wallet: use_signal(|| None),
            pubkey: use_signal(|| None),
            connected: use_signal(|| false),
            connecting: use_signal(|| false),
            disconnecting: use_signal(|| false),
        }
    }
}

impl std::fmt::Debug for WalletContextState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WalletContextState")
            .field("auto_connect", &format!("{:?}", self.auto_connect))
            .field("wallets", &format!("{:?}", self.wallets))
            .field("wallet", &format!("{:?}", self.wallet))
            .field("pubkey", &format!("{:?}", self.pubkey))
            .field("connected", &format!("{:?}", self.connected))
            .field("connecting", &format!("{:?}", self.connecting))
            .field("disconnecting", &format!("{:?}", self.disconnecting))
            .finish()
    }
}

impl PartialEq for WalletContextState {
    fn eq(&self, other: &Self) -> bool {
        self.auto_connect != other.auto_connect
            && self
                .wallet
                .peek()
                .as_ref()
                .map_or("".to_string(), |a| a.name().to_string())
                != other
                    .wallet
                    .peek()
                    .as_ref()
                    .map_or("None".to_string(), |a| a.name().to_string())
    }
}

pub static WALLET_CONTEXT: GlobalSignal<WalletContextState> = Signal::global(|| Default::default());

pub fn use_wallet() -> WalletContextState {
    return use_context();
}
