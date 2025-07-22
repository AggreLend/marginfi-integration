use anchor_lang::prelude::*;
use crate::{constants::{PERSONAL_SEED, SATURN_GROUP_SEED, STF_MINT}, errors::BondError, saturnaccounts::{Treasury, UserStakeAccount}};

use anchor_spl::{associated_token::AssociatedToken, token::{self, Mint, Token, TokenAccount, Transfer}};
use solana_program::pubkey::Pubkey;
use std::{mem, str::FromStr};


#[derive(Accounts)]

pub struct UnstakeSTF<'info> {


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

// Amount to unstake is in sSTF
pub fn handle(ctx: Context<UnstakeSTF>, amount_to_unstake: u64) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;

    let user_token_account = &mut &ctx.accounts.user_token_account;
    let treasury_token_account = &mut &ctx.accounts.treasury_token_account;
    let _stf_token_mint = &mut &ctx.accounts.stf_token_mint;
    let _user = &mut ctx.accounts.user;
    let personal_account = &mut ctx.accounts.user_stake_account;
    let authority_bump = ctx.bumps.treasury;


    require!(personal_account.total_staked_index as u64 >= amount_to_unstake, BondError::UnstakingError); 

    // assert!(
    //     stf_token_mint.key().to_string().as_str() == STF_MINT,
    //     "STF_TOKEN_MINT ERROR"
    // );
    // Add STF
    let amount_to_transfer = amount_to_unstake * treasury.staking_index;
    personal_account.total_staked_index  -= amount_to_unstake;
    treasury.token_staked -= amount_to_unstake;

    let accounts = Transfer {
        from: treasury_token_account.to_account_info(),
        to: user_token_account.to_account_info(),
        authority: treasury.to_account_info(),

    };

    let authority_bump_seeds = [authority_bump];
    let signer_seeds: &[&[&[u8]]] = &[&[SATURN_GROUP_SEED.as_bytes(), authority_bump_seeds.as_ref()]];

    let ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        accounts,
        signer_seeds
    );

    let _ = token::transfer(ctx, amount_to_transfer);



    Ok(())
}