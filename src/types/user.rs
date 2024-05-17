use anchor_lang::prelude::Pubkey;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct TokenBalance {
    pub symbol: String,
    pub mint: Pubkey,
    pub account: Pubkey,
    pub balance_native: u64,
    pub balance: f64,
    pub decimals: u8,
}
