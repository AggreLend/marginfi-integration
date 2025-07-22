use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserStakeAccount {
    pub total_staked_index: u64, // Weighted Staking
    pub total_points: u64, // Not yet implemented
}