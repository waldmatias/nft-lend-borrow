use anchor_lang::prelude::*;

#[account]
pub struct Offer {
    pub collection: Pubkey, 
    pub offer_lamport_amount: u64, 
    pub repay_lamport_amount: u64, 
    pub lender: Pubkey, 
    pub is_loan_taken: bool, 
    pub borrower: Pubkey, 
    pub bump: u8,
}

impl Offer {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 32 + 1 + 32 + 1;
}