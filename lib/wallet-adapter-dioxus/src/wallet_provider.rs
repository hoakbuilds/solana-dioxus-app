use std::sync::Arc;
use crate::{use_local_storage_opt, WalletContextState, WalletState};
use dioxus::prelude::*;
use solana_wallet_adapter::{IntoWalletAdapter, Wallet};
use solana_wallet_adapter_base::{Event,WalletReadyState, WalletAdapter};

pub const WALLET_NAME_KEY: &'static str = "wallet_name";

#[component]
pub fn WalletProvider(
    wallets: Vec<Wallet>,
    auto_connect: bool,
    local_storage_key: Option<String>,
    children: Element,
) -> Element {
    let wallet_states: Vec<_> = wallets
        .iter()
        .map(|w| WalletState {
            adapter: Some(w.into_wallet_adapter()),
        })
        .filter(|ws| ws.ready_state() != WalletReadyState::Unsupported)
        .collect(); 

    let mut wallet_name =
        use_local_storage_opt::<String>(local_storage_key.unwrap_or(WALLET_NAME_KEY.to_string()));
    let mut maybe_wallet_state = use_signal(|| None);
    let mut maybe_adapter: Signal<Option<Arc<dyn WalletAdapter>>> = use_signal(|| None);
    let mut maybe_pubkey = use_signal(|| None);
    let mut connected = use_signal(|| false);
    let maybe_selected_wallet = use_signal(|| wallet_name());
    let connecting = use_signal(|| false);
    let disconnecting = use_signal(|| false);

    // Wallet event handling
    use_future(move || async move {
        loop {
            if let Some(adapter) = maybe_adapter() {
                // Spawn event listener
                log::info!("Listening to events..");

                adapter
                    .on_event(&mut |msg| match msg {
                        Event::Default => (),
                        Event::Connect(pk) => {
                            log::info!("Wallet Pubkey: {:?}", pk);
                            // At this point we are connected
                            connected.set(true);
                            maybe_pubkey.set(Some(pk));
                        }
                        Event::Disconnect => {
                            // At this point we are disconnected from the wallet
                            connected.set(false);
                            maybe_pubkey.set(None);
                        }
                        Event::Error(e) => {
                            log::error!("Wallet error: {:?}", e);
                        }
                        Event::ReadyStateChange(_) => (),
                    })
                    .await;
            }
            gloo_timers::future::TimeoutFuture::new(100).await;
        }
    });

    // Handle wallet change
    use_effect(move || {
        let prev_wallet_name = wallet_name.peek().clone();
        if let Some(selected_wallet) = maybe_selected_wallet() {
            log::info!("Wallet Name: {:?}", selected_wallet,);

            let wallet_state = wallet_states
                .iter()
                .find(|w| w.name() == selected_wallet)
                .map_or(None, |w| Some(w.clone()));
            log::info!("Wallet State: {:?}", wallet_state);

            if let Some(wallet_state) = &wallet_state {
                log::info!("Selected Wallet State: {:?}", wallet_state,);

                if let Some(adapter) = &wallet_state.adapter {
                    log::info!("Selected Wallet Adapter: {:?}", adapter);

                    let pubkey = adapter.pubkey();
                    log::info!("Selected Wallet Pubkey: {:?}", pubkey);

                    maybe_pubkey.set(pubkey);
                }

                maybe_adapter.set(None);
                maybe_adapter.set(wallet_state.adapter.clone());
            }

            maybe_wallet_state.set(wallet_state);

            if let Some(wn) = prev_wallet_name {
                if wn != selected_wallet {
                    wallet_name.set(Some(selected_wallet));
                }
            }
        }
    });

    // Connect
    use_context_provider(|| maybe_selected_wallet);
    use_context_provider(|| WalletContextState {
        auto_connect: false,
        wallets,
        wallet: maybe_wallet_state,
        pubkey: maybe_pubkey,
        connected,
        connecting,
        disconnecting,
    });

    rsx! {
        {children}
    }
}
