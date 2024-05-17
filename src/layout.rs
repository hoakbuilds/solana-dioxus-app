use dioxus::prelude::*;
use dioxus_router::prelude::*;
use solana_wallet_adapter_dioxus::{use_connection, use_wallet};

use crate::{
    components::{footer::Footer, Navbar},
    context::{search::use_search_state, use_cluster, user},
    route::Route,
};

#[component]
pub fn Layout() -> Element {
    let route: Route = use_route();
    let user_ctx = user();
    let cluster_ctx = use_cluster();
    let search_ctx = use_search_state();
    let connection = use_connection();
    let wallet = use_wallet();

    use_context_provider(|| search_ctx);
    use_context_provider(|| user_ctx);
    use_context_provider(|| cluster_ctx);

    log::info!("Layout");

    log::info!("Connection: {:?}", connection);

    rsx! {
        div {
            class: "flex flex-col w-full",
            Navbar { route }
        }
        div {
            class: "flex-1 flex flex-col px-1 md:px-2",
            id: "content",
            Outlet::<Route> { }
        }
        Footer { }
    }
}
