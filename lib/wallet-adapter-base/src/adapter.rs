use crate::{WalletAdapter, WalletAdapterBase};
use std::sync::Arc;

trait_union::trait_union! {
    union Adapter: std::fmt::Debug = Arc<dyn WalletAdapter> | Arc<dyn WalletAdapterBase>;
}
