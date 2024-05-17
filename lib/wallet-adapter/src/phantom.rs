#![allow(non_upper_case_globals)]
use self::wasmgen::solana;
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
    use crate::{JsPublicKey, WalletEventEmitter};
    use wasm_bindgen::{prelude::*, JsStatic};

    #[wasm_bindgen]
    extern "C" {
        pub type Solana;

        // Phantom re-exports window.phantom.solana as window.solana
        pub static solana: Solana;

        #[wasm_bindgen(method, getter=isConnected)]
        pub fn is_connected(this: &Solana) -> bool;

        #[wasm_bindgen(method, getter=isPhantom)]
        pub fn is_phantom(this: &Solana) -> bool;

        #[wasm_bindgen(method, getter=priorityFeesSupported)]
        pub fn priority_fees_supported(this: &Solana) -> bool;

        #[wasm_bindgen(method, getter=_eventsCount)]
        pub fn events_count(this: &Solana) -> JsValue;

        #[wasm_bindgen(method, getter=publicKey)]
        pub fn pubkey(this: &Solana) -> JsPublicKey;

        #[wasm_bindgen(method)]
        pub fn on(this: &Solana, e: &str, cb: &Closure<dyn FnMut(web_sys::Event)>) -> bool;

        #[wasm_bindgen(method, catch, js_name=connect)]
        pub async fn connect(this: &Solana) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method, js_name=disconnect)]
        pub async fn disconnect(this: &Solana);

        #[wasm_bindgen(method, catch, js_name=signMessage)]
        pub async fn sign_message(
            this: &Solana,
            message: Vec<u8>,
            pubkey: Option<JsPublicKey>,
        ) -> Result<JsValue, JsValue>;
    }

    impl WalletEventEmitter for JsStatic<Solana> {
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
pub struct PhantomWalletAdapter {
    connecting: Mutex<bool>,
    connected: Mutex<bool>,
    pubkey: Pubkey,
    wrs: WalletReadyState,
    wel: Mutex<WalletEventListener>,
}

impl Clone for PhantomWalletAdapter {
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

impl PartialEq for PhantomWalletAdapter {
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

impl PhantomWalletAdapter {
    pub fn new() -> Self {
        let pubkey = solana.pubkey();
        let wel = WalletEventListener::new(&solana);

        let wrs = if let Some(window) = web_sys::window() {
            if let Some(_) = window.document() {
                if solana.is_undefined() {
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
            connected: solana.is_connected().into(),
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

impl WalletAdapterMetadata for PhantomWalletAdapter {
    fn name(&self) -> &str {
        Wallet::Phantom.into_wallet_name()
    }

    fn url(&self) -> &str {
        Wallet::Phantom.into_wallet_url()
    }

    fn icon(&self) -> &str {
        Wallet::Phantom.into_wallet_icon()
    }
}

#[async_trait(?Send)]
impl WalletAdapterBase for PhantomWalletAdapter {
    fn is_name(&self) -> bool {
        solana.is_phantom()
    }

    fn is_connected(&self) -> bool {
        solana.is_connected()
    }

    fn ready_state(&self) -> WalletReadyState{
        self.wrs.clone()
    }

    fn pubkey(&self) -> Option<Pubkey> {
        if solana.pubkey().is_null() {
            None
        } else {
            Some(solana.pubkey().to_pubkey())
        }
    }

    fn connecting(&self) -> bool {
        *self.connecting.lock().unwrap()
    }

    async fn connect(&self, _: Option<&str>) {
        *self.connecting.lock().unwrap() = true;
        //solana.connect().await;
        match solana.connect().await {
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
        solana.disconnect().await
    }
}

#[async_trait(?Send)]
impl WalletAdapter for PhantomWalletAdapter {
    fn priority_fees_supported(&self) -> bool {
        solana.priority_fees_supported()
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
