pub mod user;

use serde::de::DeserializeOwned;
pub use user::*;

use gloo_storage::Storage;

pub fn get<'a, T: Clone + DeserializeOwned>(key: &str) -> gloo_storage::Result<T>
where
    T: Clone + for<'de> serde::Deserialize<'de>,
{
    gloo_storage::LocalStorage::get::<T>(key)
}

pub fn set<T: serde::Serialize>(key: &str, value: T) -> gloo_storage::Result<()>
where
    T: serde::Serialize,
{
    gloo_storage::LocalStorage::set(key, value)
}
