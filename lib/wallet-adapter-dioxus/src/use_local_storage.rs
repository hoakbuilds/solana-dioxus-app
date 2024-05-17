use dioxus::prelude::*;
use gloo_storage::Storage;
use serde::{de::Deserialize, Serialize};

/// Creates a new signal where the initial value is tentatively fetched from local storage
/// and subscribes to changes to it, updating local storage values whenever the
/// value of the subscribed signal changes.
///
/// # Initial value
///
/// If there is no value found in local storage, a `None` is used as the initial value.
pub fn use_local_storage_opt<T>(key: String) -> Signal<Option<T>>
where
    T: Default + Clone + Serialize + for<'de> Deserialize<'de> + std::fmt::Debug,
{
    let state = use_signal(|| match gloo_storage::LocalStorage::get(&key) {
        Ok(r) => Some(r),
        Err(_) => None,
    });

    use_effect(move || {
        if state().is_none() {
            log::info!("use_local_storage_opt -> use_effect: None");
            gloo_storage::LocalStorage::set::<Option<T>>(&key, None).unwrap();
        } else {
            log::info!("use_local_storage_opt -> use_effect: {:?}", state());
            gloo_storage::LocalStorage::set::<Option<T>>(&key, state()).unwrap();
        }
    });

    state
}

/// Creates a new signal where the initial value is tentatively fetched from local storage,
/// and subscribes to changes to it, updating local storage values whenever the
/// value of the subscribed signal changes.
///
/// # Initial value
///
/// If there is no value found in local storage and no default is provided, the `Default` impl is used as the initial value.
pub fn use_local_storage<T>(key: String, default: Option<T>) -> Signal<T>
where
    T: Default + Clone + Serialize + for<'de> Deserialize<'de>,
{
    let state = use_signal(|| match gloo_storage::LocalStorage::get(&key) {
        Ok(r) => r,
        Err(_) => default.unwrap_or(Default::default()),
    });

    use_effect(move || {
        log::info!("use_local_storage -> use_effect");
        gloo_storage::LocalStorage::set::<T>(&key, state()).unwrap();
    });

    state
}
