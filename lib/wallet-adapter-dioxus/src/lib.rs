mod connection_provider;
mod use_connection;
mod use_local_storage;
mod use_wallet;
mod wallet_provider;

pub use connection_provider::*;
pub use use_connection::*;
pub use use_local_storage::*;
pub use use_wallet::*;
pub use wallet_provider::*;

/// Represents the Solana network cluster.
#[derive(Debug, Clone)]
pub enum Cluster {
    /// Mainnet cluster
    Mainnet,
    /// Testenet cluster
    Testnet,
    /// Devnet cluster
    Devnet,
}

/// Attempts to infer the cluster from the given endpoint.
pub fn get_inferred_cluster_from_endpoint(endpoint: Option<&str>) -> Cluster {
    if let Some(endpoint) = endpoint {
        if endpoint.contains("testnet") {
            Cluster::Testnet
        } else if endpoint.contains("devnet") {
            Cluster::Devnet
        } else {
            Cluster::Mainnet
        }
    } else {
        Cluster::Mainnet
    }
}
