use crate::ConnectionContextState;
use dioxus::prelude::*;
use solana_wallet_adapter_base::Connection;

/// A connection provider.
///
/// ## Usage
///
/// ```
/// let connection = use_connection();
/// // do something with `connection.rpc()`
/// ```
#[component]
pub fn ConnectionProvider(endpoint: Signal<String>, children: Element) -> Element {
    let connection = use_memo(move || ConnectionContextState {
        connection: Connection::new(&endpoint()),
    });

    use_context_provider(|| connection().connection);

    rsx! {
        {children}
    }
}
