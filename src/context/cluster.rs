use dioxus::prelude::*;
use js_sys::WebAssembly::RuntimeError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

const KEY: &'static str = "cluster";

// Cluster context.
static CLUSTER: GlobalSignal<Cluster> = Signal::global(|| load_or_default());

pub fn use_cluster() -> Signal<Cluster> {
    use_hook(|| CLUSTER.signal())
}

fn load() -> gloo_storage::Result<Cluster> {
    crate::storage::get::<Cluster>(KEY)
}

fn save(value: Cluster) -> gloo_storage::Result<()> {
    crate::storage::set(KEY, value)
}

pub fn load_or_default() -> Cluster {
    match load() {
        Ok(data) => data,
        Err(e) => {
            log::error!("Error loading cluster from local storage: {:?}", e);
            Cluster::Mainnet
        }
    }
}

pub fn set_cluster(new: Cluster) {
    let mut cluster = use_cluster();

    match crate::storage::set(KEY, new.clone()) {
        Ok(()) => (),
        Err(e) => {
            log::error!("Error updating cluster local storage: {:?}", e);
        }
    };
    cluster.set(new)
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum Cluster {
    #[default]
    Mainnet,
    Devnet,
    Custom(String),
}

impl Cluster {
    pub fn url(&self) -> String {
        match self {
            Self::Mainnet => {
                "https://glint.rpcpool.com/e9c697d4-c929-4dcb-83fe-9ee0d4d0c168".to_string()
            }
            Self::Devnet => {
                "https://rpc-devnet.helius.xyz/?api-key=8f29b4e9-37a6-4775-88c6-6f971fe180ca"
                    .to_string()
            }
            Self::Custom(url) => url.to_string(),
        }
    }

    pub fn to_solana_explorer_cluster(&self) -> String {
        match self {
            Self::Mainnet => String::new(),
            Self::Devnet => "?cluster=devnet".to_string(),
            Self::Custom(url) => format!("?cluster={}", url),
        }
    }
}

impl ToString for Cluster {
    fn to_string(&self) -> String {
        match self {
            Self::Mainnet => "Mainnet".to_string(),
            Self::Devnet => "Devnet".to_string(),
            Self::Custom(_) => "Custom".to_string(),
        }
    }
}

impl FromStr for Cluster {
    type Err = RuntimeError;

    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        match expression {
            "Mainnet" => Ok(Self::Mainnet),
            "Devnet" => Ok(Self::Devnet),
            "Custom" => Ok(Self::Custom("http://localhost::8899".to_string())),
            _ => Err(RuntimeError::new("Invalid expression")),
        }
    }
}
