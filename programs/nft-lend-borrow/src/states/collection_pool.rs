use anchor_lang::prelude::*;

#[account]
pub struct CollectionPool {
    pub collection_id: Pubkey, 
    pub pool_owner: Pubkey, 
    pub duration: i64, 
    pub total_offers: u64, 
    pub bump: u8,
}

impl CollectionPool {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 1;
}