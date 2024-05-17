use futures::Stream;
use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_wallet_adapter_base::{Event, WalletReadyState};
use std::pin::Pin;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use wasm_bindgen::prelude::Closure;

use crate::WalletEventEmitter;

type EventHandler = Closure<dyn FnMut(web_sys::Event)>;

/// Handles the `disconnect()` event.
///
/// ## Standard
/// This is based on the `WalletAdapterEvents` interface as it was implemented in the typescript adapter.
///
/// Read: https://github.com/anza-xyz/wallet-adapter/blob/master/packages/core/base/src/adapter.ts#L8
pub(crate) fn handle_disconnect_event(e: web_sys::Event, s: Arc<UnboundedSender<Event>>) {
    if e.is_undefined() {
        log::info!("Disconnect event.");
        s.unbounded_send(Event::Disconnect).unwrap();
    }
}

/// Handles the `connect(Pubkey)` event.
///
/// ## Standard
/// This is based on the `WalletAdapterEvents` interface as it was implemented in the typescript adapter.
///
/// Read: https://github.com/anza-xyz/wallet-adapter/blob/master/packages/core/base/src/adapter.ts#L8
pub(crate) fn handle_connect_event(e: web_sys::Event, s: Arc<UnboundedSender<Event>>) {
    if let Some(pubkey_string) = e.to_string().as_string() {
        match Pubkey::from_str(&pubkey_string) {
            Ok(r) => {
                log::info!("Connect event. Wallet Pubkey: {:?}", r);
                s.unbounded_send(Event::Connect(r)).unwrap();
            }
            Err(e) => {
                log::error!("Connect event. Failed to parse wallet pubkey: {:?}", e);
            }
        }
    }
}

/// Handles the `error(WalletError)` event.
///
/// ## Standard
/// This is based on the `WalletAdapterEvents` interface as it was implemented in the typescript adapter.
///
/// Read: https://github.com/anza-xyz/wallet-adapter/blob/master/packages/core/base/src/adapter.ts#L8
pub(crate) fn handle_error_event(e: web_sys::Event, s: Arc<UnboundedSender<Event>>) {
    if let Some(error) = e.to_string().as_string() {
        log::info!("Error event. {}", error);
        s.unbounded_send(Event::Disconnect).unwrap();
    }
}

/// Handles the `ready_state_change(WalletReadyState)` event.
///
/// ## Standard
/// This is based on the `WalletAdapterEvents` interface as it was implemented in the typescript adapter.
///
/// Read: https://github.com/anza-xyz/wallet-adapter/blob/master/packages/core/base/src/adapter.ts#L8
pub(crate) fn handle_ready_state_change_event(e: web_sys::Event, s: Arc<UnboundedSender<Event>>) {
    if let Some(ready_state) = e.to_string().as_string() {
        log::info!("Ready state change event. {}", ready_state);
        s.unbounded_send(Event::ReadyStateChange(WalletReadyState::Loadable))
            .unwrap();
    }
}

pub(crate) struct WalletEventListener {
    pub(crate) rsc_ep: Mutex<EventProxy>,
    pub(crate) dc_ep: Mutex<EventProxy>,
    pub(crate) c_ep: Mutex<EventProxy>,
    pub(crate) e_ep: Mutex<EventProxy>,
    /// The event receiver.
    pub(crate) receiver: Option<UnboundedReceiver<Event>>,
}

impl WalletEventListener {
    pub fn new(ee: &dyn WalletEventEmitter) -> Self {
        // disconecct
        let (sender, receiver) = unbounded();
        let sender = Arc::new(sender);

        let dc_ep = Mutex::new(EventProxy::new(
            sender.clone(),
            Arc::new(Mutex::new(handle_disconnect_event)),
        ));
        ee.on_disconnect(&dc_ep.lock().unwrap().handler.as_ref().unwrap());
        // readyStateChange
        let rsc_ep = Mutex::new(EventProxy::new(
            sender.clone(),
            Arc::new(Mutex::new(handle_ready_state_change_event)),
        ));
        ee.on_ready_state_change(&rsc_ep.lock().unwrap().handler.as_ref().unwrap());
        // connect
        let c_ep = Mutex::new(EventProxy::new(
            sender.clone(),
            Arc::new(Mutex::new(handle_connect_event)),
        ));
        ee.on_connect(&c_ep.lock().unwrap().handler.as_ref().unwrap());
        // error
        let e_ep = Mutex::new(EventProxy::new(
            sender.clone(),
            Arc::new(Mutex::new(handle_error_event)),
        ));
        ee.on_error(&e_ep.lock().unwrap().handler.as_ref().unwrap());
        Self {
            e_ep,
            dc_ep,
            c_ep,
            rsc_ep,
            receiver: Some(receiver),
        }
    }
}

impl std::fmt::Debug for WalletEventListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WalletEventListener")
            .field("rsc_ep", &format!("{:?}", self.rsc_ep))
            .field("c_ep", &format!("{:?}", self.c_ep))
            .field("dc_ep", &format!("{:?}", self.dc_ep))
            .field("e_ep", &format!("{:?}", self.e_ep))
            .finish()
    }
}

impl Clone for WalletEventListener {
    fn clone(&self) -> Self {
        Self {
            rsc_ep: self.rsc_ep.lock().unwrap().clone().into(),
            c_ep: self.c_ep.lock().unwrap().clone().into(),
            dc_ep: self.dc_ep.lock().unwrap().clone().into(),
            e_ep: self.e_ep.lock().unwrap().clone().into(),
            receiver: None,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.rsc_ep = source.rsc_ep.lock().unwrap().clone().into();
        self.c_ep = source.c_ep.lock().unwrap().clone().into();
        self.dc_ep = source.dc_ep.lock().unwrap().clone().into();
        self.e_ep = source.e_ep.lock().unwrap().clone().into();
        self.receiver = None;
    }
}

impl Stream for WalletEventListener {
    type Item = Event;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(ref mut receiver) = &mut self.receiver {
            Pin::new(receiver).poll_next(cx)
        } else {
            Poll::Ready(None)
        }
    }
}

/// A proxy for events emitted by wallet extensions.
///
/// ## Behavior
/// If the initial reference to the closure gets dropped then the JS side will no longer
/// be able to invoke it.
///
/// Read:
/// https://rustwasm.github.io/wasm-bindgen/reference/passing-rust-closures-to-js.html
///
pub(crate) struct EventProxy {
    /// The rust closure that gets passed into js.
    pub(crate) handler: Option<EventHandler>,
}

impl std::fmt::Debug for EventProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventProxy")
            .field("handler", &format!("{:?}", self.handler))
            .finish()
    }
}

impl EventProxy {
    pub fn new(
        sender: Arc<UnboundedSender<Event>>,
        f: Arc<Mutex<dyn FnMut(web_sys::Event, Arc<UnboundedSender<Event>>)>>,
    ) -> Self {
        let handler = Closure::new(move |e: web_sys::Event| {
            let sender_c = sender.clone();
            log::info!("web_sys::Event {:?}", e);
            f.lock().unwrap()(e, sender_c);
        });

        Self {
            handler: Some(handler),
        }
    }
}

impl Clone for EventProxy {
    fn clone(&self) -> Self {
        Self { handler: None }
    }
}
