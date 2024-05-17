use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// TODO: Think about doing stateful components.

const KEY: &'static str = "search_state";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SearchState {
    pub active: bool,
    pub busy: bool,
    pub query: String,
    pub results: Vec<SearchResult>,
}

// Search state.
static SEARCH_STATE: GlobalSignal<SearchState> = Signal::global(|| load_or_default());

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub route: String,
}

impl PartialEq for SearchState {
    fn eq(&self, other: &Self) -> bool {
        self.active.eq(&other.active)
            && self.busy.eq(&other.busy)
            && self.query.eq(&other.query)
            && self.results.eq(&other.results)
    }
}

pub fn use_search_state() -> Signal<SearchState> {
    use_hook(|| SEARCH_STATE.signal())
}

fn load() -> gloo_storage::Result<SearchState> {
    crate::storage::get::<SearchState>(KEY)
}

fn save(value: SearchState) -> gloo_storage::Result<()> {
    crate::storage::set(KEY, value)
}

pub fn load_or_default() -> SearchState {
    match load() {
        Ok(data) => data,
        Err(e) => {
            log::error!("Error loading search state from local storage: {:?}", e);
            SearchState::default()
        }
    }
}

pub fn toggle_busy() {
    let mut search_state_c = use_search_state().read().clone();

    search_state_c.busy = !search_state_c.busy;

    match save(search_state_c.clone()) {
        Ok(()) => (),
        Err(e) => {
            log::error!("Error updating search state local storage: {:?}", e);
        }
    };
    set_search_state(search_state_c);
}

pub fn toggle_active() {
    let mut search_state_c = use_search_state().read().clone();

    search_state_c.active = !search_state_c.active;

    match save(search_state_c.clone()) {
        Ok(()) => (),
        Err(e) => {
            log::error!("Error updating search state local storage: {:?}", e);
        }
    };
    set_search_state(search_state_c);
}

pub fn set_search_state(new: SearchState) {
    let mut search_state = use_search_state();

    match crate::storage::set(KEY, new.clone()) {
        Ok(()) => (),
        Err(e) => {
            log::error!("Error updating search state local storage: {:?}", e);
        }
    };
    search_state.set(new)
}
