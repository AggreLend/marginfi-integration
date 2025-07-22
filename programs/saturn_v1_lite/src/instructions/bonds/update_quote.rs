use anchor_lang::prelude::*;
use crate::constants::*;

use crate::saturnaccounts::Treasury;
use crate::utils::calculate_bond_quote;



    pub fn handle(ctx: Context<UpdateBondQuote>, current_trading_price: u64, current_treasury_value: i128) -> Result<()> {
        assert_eq!(
            ctx.accounts.saturn_api_key.key(), 
            ctx.accounts.saturn_group.saturn_group_api_key
            ,"Insufficient Permission to Update Bond Quote.");

        
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;
        // 10_000 for USDC treasury value to 1B
        // The amounts handled are 
        // Calculate how much we are backing each token by 
        let backing_value_per_token = current_treasury_value * 1000 / ctx.accounts.saturn_group.token_minted as i128;
        let usdc_value_per_token = backing_value_per_token * 1_000_000;
        let bond_quote = calculate_bond_quote(current_trading_price, usdc_value_per_token as u64);
        msg!("{:?}", bond_quote);
        msg!("usdc_value_per_token: {:?}", usdc_value_per_token);
        msg!("backing_value_per_token {:?}", backing_value_per_token);

        ctx.accounts.saturn_group.last_treasury_value_update = current_timestamp;
        ctx.accounts.saturn_group.bond_quote_usd = bond_quote;
        ctx.accounts.saturn_group.treasury_value = current_treasury_value as u128;
        ctx.accounts.saturn_group.tokens_minted_last_update = ctx.accounts.saturn_group.token_minted;

        Ok(())


    
    }


#[derive(Accounts)]
pub struct UpdateBondQuote<'info> {
    #[account(
        mut,
        seeds = [SATURN_GROUP_SEED.as_ref()],
        bump
    )]
    pub saturn_group: Account<'info, Treasury>,
    #[account(mut)]
    pub saturn_api_key: Signer<'info>,
    pub system_program: Program<'info, System>,
}