use anchor_lang::prelude::*;

#[account]
pub struct ActiveLoan {
    pub collection: Pubkey, 
    pub offer_account: Pubkey, 
    pub lender: Pubkey, 
    pub borrower: Pubkey, 
    pub mint: Pubkey, 
    pub loan_ts: i64, // loan taken timestamp
    pub repay_ts: i64, 
    pub is_repaid: bool, 
    pub is_liquidated: bool, 
    pub bump: u8, 
}

impl ActiveLoan {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 32 + 32 + 8 + 8 + 1 + 1 + 1;
}