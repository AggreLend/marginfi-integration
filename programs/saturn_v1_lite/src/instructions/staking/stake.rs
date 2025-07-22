use anchor_lang::prelude::*;
use crate::{constants::{PERSONAL_SEED, SATURN_GROUP_SEED, STF_MINT}, saturnaccounts::{Treasury, UserStakeAccount}};

use anchor_spl::{associated_token::AssociatedToken, token::{self, Mint, Token, TokenAccount, Transfer}};
use solana_program::pubkey::Pubkey;
use std::{mem, str::FromStr};


#[derive(Accounts)]

pub struct StakeSTF<'info> {


    #[account(
        init_if_needed,
        space = mem::size_of::<UserStakeAccount>() as usize + 8,
        payer = user,
        seeds=[PERSONAL_SEED.as_ref(), user.key.as_ref()], // Static Seed Path (1)
        bump, 
    )]
    pub user_stake_account: Account<'info, UserStakeAccount>,

    #[account(
        mut,
        seeds = [SATURN_GROUP_SEED.as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,
    


    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = stf_token_mint,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = stf_token_mint,
        associated_token::authority = treasury
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
       address = Pubkey::from_str(STF_MINT).unwrap()
       )]
    pub stf_token_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<StakeSTF>, amount_to_stake: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let source_token_account: &mut &Account<'_, TokenAccount> = &mut &ctx.accounts.user_token_account;
    let dest_stf_account = &mut &ctx.accounts.treasury_token_account;
    let user = &mut ctx.accounts.user;
    let user_stake_account = &mut ctx.accounts.user_stake_account;

    // assert!(
    //     stf_token_mint.key().to_string().as_str() == STF_MINT,
    //     "STF_TOKEN_MINT ERROR"
    // );


    // Transfer Tokens To Treasury 
    let cpi_accounts = Transfer {
        from: source_token_account.to_account_info().clone(),
        to: dest_stf_account.to_account_info().clone(),
        authority: user.to_account_info().clone(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    
    token::transfer(
        CpiContext::new(cpi_program, cpi_accounts),
    amount_to_stake)?;

    // Add STF
    let amount_to_transfer = amount_to_stake / treasury.staking_index;
    user_stake_account.total_staked_index  += amount_to_transfer;
    treasury.token_staked += amount_to_stake;

    Ok(())
}