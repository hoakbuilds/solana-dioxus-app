use crate::{
    layout::Layout,
    pages::{
        account::AccountPage, not_found::NotFoundPage, trade::Trade, transaction::TransactionPage,
    },
    types::QuerySegments,
};
use anchor_lang::prelude::Pubkey;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use solana_client_wasm::solana_sdk::signature::Signature;

#[derive(Clone, PartialEq, Routable)]
#[rustfmt::skip]
pub(crate) enum Route {
    #[layout(Layout)] // wrap the entire app in the layout
        #[route("/")]
        Trade {
            from: Pubkey,
            to: Pubkey,
            amount: f64,
            // You must include query segments in child variants
            query_params: QuerySegments,
        },
        #[route("/a/:address", AccountPage)]
        Account {
            address: Pubkey,
            // You must include query segments in child variants
            query_params: QuerySegments,
        },
        #[route("/t/:signature", TransactionPage)]
        Transaction {
            signature: Signature,
            // You must include query segments in child variants
            query_params: QuerySegments,
        },


    // Finally, we need to handle the 404 page
    #[route("/:..route", NotFoundPage)]
    NotFound {
        route: Vec<String>,
    },
}

impl Route {
    pub fn custom_rpc_url(&self) -> Option<String> {
        match self {
            Route::Trade { query_params, .. } => Some(query_params.custom_url.clone()),
            Route::Account { query_params, .. } => Some(query_params.custom_url.clone()),
            Route::Transaction { query_params, .. } => Some(query_params.custom_url.clone()),
            _ => None,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Route::Trade { .. } => "/".to_owned(),
            Route::NotFound { .. } => "".to_owned(),
            Route::Account { address, .. } => format!("/a/{:?}", address), // todo: add query params back
            Route::Transaction { signature, .. } => format!("/t/{:?}", signature),
            Route::Trade {
                from,
                to,
                amount,
                query_params,
            } => format!("/{:?}/{:?}/{:?}", from, to, amount),
        }
    }
    pub fn generic_path<'a>(&self) -> &'a str {
        match self {
            Route::Trade { .. } => "/",
            Route::NotFound { .. } => "",
            Route::Account { address, .. } => "/a/:address",
            Route::Transaction { signature, .. } => "/t/:signature",
            Route::Trade {
                from,
                to,
                amount,
                query_params,
            } => "/:from/:to/:amount",
        }
    }
}
