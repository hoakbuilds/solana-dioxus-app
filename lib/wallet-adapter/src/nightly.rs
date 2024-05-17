#![allow(non_upper_case_globals)]
use self::wasmgen::nightly;
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
    use wasm_bindgen::prelude::*;

    use crate::{JsPublicKey, WalletEventEmitter};

    #[wasm_bindgen]
    extern "C" {
        pub type Nightly;
        pub type NightlySolana;

        pub static nightly: Nightly;

        #[wasm_bindgen(method, getter=solana)]
        pub fn solana(this: &Nightly) -> NightlySolana;

        #[wasm_bindgen(method, getter=isConnected)]
        pub fn is_connected(this: &NightlySolana) -> bool;

        #[wasm_bindgen(method, getter=isNightly)]
        pub fn is_nightly(this: &NightlySolana) -> bool;

        #[wasm_bindgen(method, getter=priorityFeesSupported)]
        pub fn priority_fees_supported(this: &NightlySolana) -> bool;

        #[wasm_bindgen(method, getter=publicKey)]
        pub fn pubkey(this: &NightlySolana) -> JsPublicKey;

        // This does not seem to work
        // #[wasm_bindgen(method, getter=onAccountChange)]
        // pub fn on(this: &NightlySolana, e: &str, cb: &Closure<dyn FnMut(web_sys::Event)>) -> bool;

        #[wasm_bindgen(method, catch, js_name=connect)]
        pub async fn connect(this: &NightlySolana) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method, js_name=disconnect)]
        pub async fn disconnect(this: &NightlySolana);

        #[wasm_bindgen(method, catch, js_name=signMessage)]
        pub async fn sign_message(
            this: &NightlySolana,
            message: Vec<u8>,
            pubkey: Option<JsPublicKey>,
        ) -> Result<JsValue, JsValue>;
    }

    impl WalletEventEmitter for NightlySolana {
        fn on_connect(&self, _: &Closure<dyn FnMut(web_sys::Event)>) {
            //self.on("connect", closure);
        }

        fn on_disconnect(&self, _: &Closure<dyn FnMut(web_sys::Event)>) {
            //self.on("disconnect", closure);
        }

        fn on_error(&self, _: &Closure<dyn FnMut(web_sys::Event)>) {
            //self.on("error", closure);
        }

        fn on_ready_state_change(&self, _: &Closure<dyn FnMut(web_sys::Event)>) {
            //self.on("readyStateChange", closure);
        }
    }
}

#[derive(Debug)]
pub struct NightlyWalletAdapter {
    connecting: Mutex<bool>,
    connected: Mutex<bool>,
    pubkey: Pubkey,
    wrs: WalletReadyState,
    wel: Mutex<WalletEventListener>,
}

impl Clone for NightlyWalletAdapter {
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

impl PartialEq for NightlyWalletAdapter {
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

impl NightlyWalletAdapter {
    pub fn new() -> Self {
        let pubkey = nightly.solana().pubkey();
        let event_listener = WalletEventListener::new(&nightly.solana());
        let ready_state = if let Some(window) = web_sys::window() {
            if let Some(_) = window.document() {
                WalletReadyState::NotDetected
            } else {
                WalletReadyState::Unsupported
            }
        } else {
            WalletReadyState::Unsupported
        };

        Self {
            connecting: false.into(),
            connected: nightly.solana().is_connected().into(),
            pubkey: if pubkey.is_null() {
                Pubkey::default()
            } else {
                pubkey.to_pubkey()
            },
            wrs: ready_state,
            wel: Mutex::new(event_listener),
        }
    }
}

impl WalletAdapterMetadata for NightlyWalletAdapter {
    fn name(&self) -> &str {
        Wallet::Nightly.into_wallet_name()
    }

    fn url(&self) -> &str {
        Wallet::Nightly.into_wallet_url()
    }

    fn icon(&self) -> &str {
        Wallet::Nightly.into_wallet_icon()
    }
}

#[async_trait(?Send)]
impl WalletAdapterBase for NightlyWalletAdapter {
    fn is_name(&self) -> bool {
        nightly.solana().is_nightly()
    }

    fn is_connected(&self) -> bool {
        nightly.solana().is_connected()
    }

    fn ready_state(&self) -> WalletReadyState{
        self.wrs.clone()
    }

    fn pubkey(&self) -> Option<Pubkey> {
        if nightly.solana().pubkey().is_null() {
            None
        } else {
            Some(nightly.solana().pubkey().to_pubkey())
        }
    }

    fn connecting(&self) -> bool {
        *self.connecting.lock().unwrap()
    }

    async fn connect(&self, _: Option<&str>) {
        *self.connecting.lock().unwrap() = true;
        match nightly.solana().connect().await {
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
        nightly.solana().disconnect().await
    }
}

#[async_trait(?Send)]
impl WalletAdapter for NightlyWalletAdapter {
    fn priority_fees_supported(&self) -> bool {
        nightly.solana().priority_fees_supported()
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
