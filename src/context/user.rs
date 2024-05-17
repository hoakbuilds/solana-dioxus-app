use anchor_lang::{prelude::Pubkey, solana_program::native_token::lamports_to_sol};
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum User {
    #[default]
    Unknown,
    Wallet {
        pubkey: Option<Pubkey>,
        lamports: u64,
    },
}

// User context.
static USER_CONTEXT: GlobalSignal<User> = Signal::global(|| User::default());

pub fn user() -> Signal<User> {
    use_hook(|| USER_CONTEXT.signal())
}

pub fn set_user(new: User) {
    let mut user = user();

    match LocalStorage::set("user_context", new.clone()) {
        Ok(()) => (),
        Err(e) => {
            log::error!("Error updating user local storage: {:?}", e);
        }
    };
    user.set(new)
}

impl User {
    pub fn lamports_display(&self) -> String {
        use User::*;
        match self {
            Unknown => format!("⊚ 0"),
            Wallet { lamports, .. } => format!("⊚ {:.2}", lamports_to_sol(*lamports)),
        }
    }

    pub fn lamports_float(&self) -> f64 {
        use User::*;
        match self {
            Unknown => 0.0,
            Wallet { lamports, .. } => lamports_to_sol(*lamports),
        }
    }
}
