#![allow(non_upper_case_globals)]
use self::wasmgen::solflare;
use crate::{
    proxy::WalletEventListener, IntoPubkey, IntoWalletIcon, IntoWalletName, IntoWalletUrl, Wallet,
};
use async_trait::async_trait;
use futures::StreamExt;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_wallet_adapter_base::{
    Event, WalletAdapter, WalletAdapterBase, WalletAdapterMetadata, WalletReadyState,
};
use std::sync::Mutex;

pub(crate) mod wasmgen {
    use wasm_bindgen::{prelude::*, JsStatic};

    use crate::{JsPublicKey, WalletEventEmitter};

    #[wasm_bindgen]
    extern "C" {
        pub type Solflare;

        pub static solflare: Solflare;

        #[wasm_bindgen(method, getter=isConnected)]
        pub fn is_connected(this: &Solflare) -> bool;

        #[wasm_bindgen(method, getter=isSolflare)]
        pub fn is_solflare(this: &Solflare) -> bool;

        #[wasm_bindgen(method, getter=priorityFeesSupported)]
        pub fn priority_fees_supported(this: &Solflare) -> bool;

        #[wasm_bindgen(method, getter=publicKey)]
        pub fn pubkey(this: &Solflare) -> JsPublicKey;

        #[wasm_bindgen(method)]
        pub fn on(this: &Solflare, e: &str, cb: &Closure<dyn FnMut(web_sys::Event)>) -> bool;

        #[wasm_bindgen(method, catch, js_name=connect)]
        pub async fn connect(this: &Solflare) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method)]
        pub async fn disconnect(this: &Solflare);

        #[wasm_bindgen(method, catch, js_name=signMessage)]
        pub async fn sign_message(
            this: &Solflare,
            message: Vec<u8>,
            pubkey: Option<JsPublicKey>,
        ) -> Result<JsValue, JsValue>;
    }

    impl WalletEventEmitter for JsStatic<Solflare> {
        fn on_connect(&self, closure: &Closure<dyn FnMut(web_sys::Event)>) {
            self.on("connect", closure);
        }

        fn on_disconnect(&self, closure: &Closure<dyn FnMut(web_sys::Event)>) {
            self.on("disconnect", closure);
        }

        fn on_error(&self, closure: &Closure<dyn FnMut(web_sys::Event)>) {
            self.on("error", closure);
        }

        fn on_ready_state_change(&self, closure: &Closure<dyn FnMut(web_sys::Event)>) {
            self.on("readyStateChange", closure);
        }
    }
}

#[derive(Debug)]
pub struct SolflareWalletAdapter {
    connecting: Mutex<bool>,
    connected: Mutex<bool>,
    pubkey: Pubkey,
    wrs: WalletReadyState,
    wel: Mutex<WalletEventListener>,
}

impl Clone for SolflareWalletAdapter {
    fn clone(&self) -> Self {
        Self {
            connecting: self.connecting().into(),
            connected: Mutex::new(*self.connected.lock().unwrap()),
            pubkey: self.pubkey.clone(),
            wrs: self.wrs.clone(),
            wel: self.wel.lock().unwrap().clone().into(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.connecting = source.connecting().into();
        self.connected = Mutex::new(*source.connected.lock().unwrap());
        self.pubkey = source.pubkey.clone();
        self.wel = source.wel.lock().unwrap().clone().into();
        self.wrs = source.wrs.clone();
    }
}

impl PartialEq for SolflareWalletAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.pubkey == other.pubkey
            && self.connecting() == other.connecting()
            && *self.connected.lock().unwrap() == *other.connected.lock().unwrap()
    }

    fn ne(&self, other: &Self) -> bool {
        self.pubkey != other.pubkey
            && self.connecting() != other.connecting()
            && *self.connected.lock().unwrap() != *other.connected.lock().unwrap()
    }
}

impl SolflareWalletAdapter {
    pub fn new() -> Self {
        let pubkey = solflare.pubkey();
        let wel = WalletEventListener::new(&solflare);

        let wrs = if let Some(window) = web_sys::window() {
            if let Some(_) = window.document() {
                if solflare.is_undefined() {
                    WalletReadyState::NotDetected
                } else {
                    WalletReadyState::Installed
                }
            } else {
                WalletReadyState::Unsupported
            }
        } else {
            WalletReadyState::Unsupported
        };
        Self {
            connecting: false.into(),
            connected: solflare.is_connected().into(),
            pubkey: if pubkey.is_null() {
                Pubkey::default()
            } else {
                pubkey.to_pubkey()
            },
            wrs,
            wel: Mutex::new(wel),
        }
    }
}

impl WalletAdapterMetadata for SolflareWalletAdapter {
    fn name(&self) -> &str {
        Wallet::Solflare.into_wallet_name()
    }

    fn url(&self) -> &str {
        Wallet::Solflare.into_wallet_url()
    }

    fn icon(&self) -> &str {
        Wallet::Solflare.into_wallet_icon()
    }
}

#[async_trait(?Send)]
impl WalletAdapterBase for SolflareWalletAdapter {
    fn is_name(&self) -> bool {
        solflare.is_solflare()
    }

    fn is_connected(&self) -> bool {
        solflare.is_connected()
    }

    fn ready_state(&self) -> WalletReadyState{
        self.wrs.clone()
    }

    fn pubkey(&self) -> Option<Pubkey> {
        let pubkey = solflare.pubkey();
        if pubkey.is_null() {
            None
        } else {
            Some(pubkey.to_pubkey())
        }
    }

    fn connecting(&self) -> bool {
        *self.connecting.lock().unwrap()
    }

    async fn connect(&self, _: Option<&str>) {
        *self.connecting.lock().unwrap() = true;
        match solflare.connect().await {
            Ok(r) => {
                log::info!("Successfully invoked connect. {:?}", r.as_string());
            }
            Err(e) => {
                log::error!("Error invoking connect. {:?}", e.as_string());
            }
        }
        *self.connecting.lock().unwrap() = false;
    }

    async fn disconnect(&self) {
        solflare.disconnect().await
    }
}

#[async_trait(?Send)]
impl WalletAdapter for SolflareWalletAdapter {
    fn priority_fees_supported(&self) -> bool {
        solflare.priority_fees_supported()
    }

    async fn on_event(&self, f: &mut dyn FnMut(Event)) {
        let mut wel = self.wel.lock().unwrap();
        if let Some(ref mut receiver) = &mut wel.receiver {
            loop {
                if let Some(msg) = receiver.next().await {
                    f(msg)
                }
            }
        }
    }
}
