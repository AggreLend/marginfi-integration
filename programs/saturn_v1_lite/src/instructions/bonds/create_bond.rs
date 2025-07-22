use std::mem;

use anchor_lang::prelude::*;

use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};



use crate::{constants::*};
use crate::saturnaccounts::{SaturnBond, Treasury};
use crate::utils::safe_divide;



    pub fn handle(ctx: Context<CreateBond>,_id: String, token_amount: u64) -> Result<()> {
        
    assert_eq!(
        ctx.accounts.bond_account.creator.to_string(),
        ctx.accounts.system_program.key().to_string(),
        "This Is Has Been Initialized Already. Please choose a different ID"
    );

    // Transfer USDC to Treasury 
    let source_token_account: &mut &Account<'_, TokenAccount> = &mut &ctx.accounts.user_token_account_usdc;
    let dest_usdc_account = &mut &ctx.accounts.treasury_token_account_usdc;
    let user = &mut ctx.accounts.user;

    // assert!(
    //     stf_token_mint.key().to_string().as_str() == STF_MINT,
    //     "STF_TOKEN_MINT ERROR"
    // );

    // Transfer Tokens To Treasury 
    let cpi_accounts = Transfer {
        from: source_token_account.to_account_info().clone(),
        to: dest_usdc_account.to_account_info().clone(),
        authority: user.to_account_info().clone(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    
    token::transfer(
        CpiContext::new(cpi_program, cpi_accounts),
        token_amount
    )?;

    // Handle data 
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    assert!(
        ctx.accounts.treasury.last_treasury_value_update + FIME_MIN > current_timestamp,
        "Bond Quote Expired."
    );

    ctx.accounts.bond_account.creator = ctx.accounts.user.key();
    ctx.accounts.bond_account.end_timestamp = current_timestamp + BOND_DURATION;
    ctx.accounts.bond_account.start_timestamp = current_timestamp;
    ctx.accounts.bond_account.token_amount = token_amount;

    // Find How Many Tokens The User Should Get 
    // Note: The reason token_amount is mul by 10M instead of 1B and then it's at the end is to try to minimize potential overflows : ) 
    let stf_token_amount_bond_reward = safe_divide(token_amount * 10_000_000, ctx.accounts.treasury.bond_quote_usd) * 100;

    ctx.accounts.bond_account.num_token_to_redeem = stf_token_amount_bond_reward;

    ctx.accounts.treasury.token_minted += token_amount;



        Ok(())
    }

#[derive(Accounts)]
#[instruction(id: String)]
pub struct CreateBond<'info> {

    #[account(
        init_if_needed,
        space = mem::size_of::<SaturnBond>() as usize + 8,
        payer = user,
        seeds=[id.as_ref(), "test".as_bytes()], // Static Seed Path (1)
        bump, 
    )]
    pub bond_account: Account<'info, SaturnBond>,


    #[account(
        mut,
        seeds = [SATURN_GROUP_SEED.as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        associated_token::mint = usdc_token_mint,
        associated_token::authority = user
    )]
    pub user_token_account_usdc: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = usdc_token_mint,
        associated_token::authority = treasury
    )]
    pub treasury_token_account_usdc: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub usdc_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}