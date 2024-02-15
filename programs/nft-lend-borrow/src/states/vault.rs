use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub offer: Pubkey, 
    pub bump: u8, 
}

impl Vault {
    pub const LEN: usize = 8 + 32 + 1;
}