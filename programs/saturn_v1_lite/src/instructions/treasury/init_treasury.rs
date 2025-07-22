use anchor_lang::prelude::*;

use crate::constants::*;
use crate::saturnaccounts::Treasury;


    pub fn handle(ctx: Context<InitializeSaturn>) -> Result<()> {
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        // let mut saturn_group = &ctx.accounts.saturn_group;
        ctx.accounts.saturn_group.staking_index = 1_000_000_000;
        ctx.accounts.saturn_group.token_minted = 1_000_000_000;
        ctx.accounts.saturn_group.treasury_admin = ctx.accounts.payer.key();
        ctx.accounts.saturn_group.saturn_group_api_key = ctx.accounts.saturn_api_key.key();

        ctx.accounts.saturn_group.last_treasury_value_update = current_timestamp; // 1 DOLLAR
        ctx.accounts.saturn_group.treasury_value = 1_000_000_000; /// 1 USDC 

        Ok(())
    }

#[derive(Accounts)]
pub struct InitializeSaturn<'info> {
    #[account(
        init,
        payer = payer,
        space = 144 + 8,
        seeds = [SATURN_GROUP_SEED.as_ref()],
        bump
    )]
    pub saturn_group: Account<'info, Treasury>,
    /// CHECK: Saturn Group API Key.
    #[account(mut)]
    pub saturn_api_key: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

}