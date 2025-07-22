use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Treasury {

    pub treasury_admin: Pubkey, // 32

    pub saturn_group_api_key: Pubkey, // 32 

    pub staking_index: u64, // 8

    pub treasury_value: u128, // Value of Treasury in USD dollars;
    pub last_treasury_value_update: i64,
    pub bond_quote_usd: u64, // 8 
    
    pub tokens_minted_last_update: u64, // The reason for this is to ensure that the amount of tokens doesn't change between the admin calling the update bond instruction instruction and 

    pub token_minted: u64, // 8
    
    pub token_staked: u64, // 8

    pub tokens_in_bonds: u64, // 8 


}  