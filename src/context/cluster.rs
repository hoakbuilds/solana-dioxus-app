use js_sys::WebAssembly::RuntimeError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, PartialEq, Default, Serialize, Deserialize)]
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
                "https://rpc.helius.xyz/?api-key=cafb5acc-3dc2-47a0-8505-77ea5ebc7ec6".to_string()
            }
            Self::Devnet => {
                "https://rpc-devnet.helius.xyz/?api-key=8f29b4e9-37a6-4775-88c6-6f971fe180ca"
                    .to_string()
            }
            Self::Custom(url) => url.to_string(),
        }
    }
}

impl ToString for Cluster {
    fn to_string(&self) -> String {
        match self {
            Self::Mainnet => "mainnet".to_string(),
            Self::Devnet => "devnet".to_string(),
            Self::Custom(_) => "custom".to_string(),
        }
    }
}

impl FromStr for Cluster {
    type Err = RuntimeError;

    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        match expression {
            "mainnet" => Ok(Self::Mainnet),
            "devnet" => Ok(Self::Devnet),
            "custom" => Ok(Self::Custom("http://localhost::8899".to_string())),
            _ => Err(RuntimeError::new("Invalid expression")),
        }
    }
}
