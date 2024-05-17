use std::fmt::format;

use anchor_lang::solana_program::native_token::lamports_to_sol;
use chrono::{DateTime, NaiveDateTime, Utc};

pub fn format_lamports(lamports: u64, short: bool) -> String {
    if short {
        format!("⊚ {:.5}", lamports_to_sol(lamports))
    } else {
        format!("⊚ {:.9}", lamports_to_sol(lamports))
    }
}

pub fn format_token_amount(value: f64) -> String {
    format!("{:.2}", value)
}

pub fn token_amount_to_ui(amount: u64, decimals: u8) -> String {
    let denominator = 10u64.pow(decimals as u32) as f64;
    let value = amount as f64 / denominator;
    format_token_amount(value)
}

pub fn format_timestamp(timestamp: i64) -> String {
    let dt = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap(),
        Utc,
    );
    format!("{}", dt.format("%F %H:%M UTC"))
}
