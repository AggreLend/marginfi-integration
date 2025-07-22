use std::mem;
use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::spl_token::instruction;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};



use crate::{constants::*, stf};
use crate::saturnaccounts::{SaturnBond, Treasury};
use crate::utils::safe_divide;



    pub fn handle(ctx: Context<RedeemTokens>, token_amount: u64) -> Result<()> {
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    assert!(
        ctx.accounts.user_token_account.amount > token_amount,
        "Not Enough Tokens To Cash-Out"
    );

    assert!(
        ctx.accounts.treasury.last_treasury_value_update + FIME_MIN > current_timestamp,
        "Bond Quote Expired."
    );

    assert_eq!(
        ctx.accounts.treasury.token_minted,
        ctx.accounts.treasury.tokens_minted_last_update as u64,
        "There Was A Difference In The Tokens Minted. Please Update Your Quote Again"
    );


    let token_backing_price = safe_divide(ctx.accounts.treasury.treasury_value as u64 * 1_000_000_000 , ctx.accounts.treasury.token_minted);
    // assert!(
    //     stf_token_mint.key().to_string().as_str() == STF_MINT,
    //     "STF_TOKEN_MINT ERROR"
    // );

    msg!("token_backing_price: {}", token_backing_price);
    msg!("treasury_value: {}", ctx.accounts.treasury.treasury_value);

    let payout = (token_amount * token_backing_price) / 1_000_000_000 ;
    
    // Update Global Data 
    ctx.accounts.treasury.token_minted -= token_amount;


    let cpi_accounts= Burn {
        from: ctx.accounts.user_token_account.to_account_info().clone(),
        authority: ctx.accounts.user.to_account_info().clone(),
        mint: ctx.accounts.stf_token_mint.to_account_info().clone(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    
    token::burn(
        CpiContext::new(cpi_program, cpi_accounts),
    token_amount)?;
    


    

    let accounts = Transfer {
        from: ctx.accounts.treasury_token_account_usdc.to_account_info(),
        to: ctx.accounts.user_token_account_usdc.to_account_info(),
        authority: ctx.accounts.treasury.to_account_info(),

    };

    let authority_bump_seeds = [ctx.bumps.treasury];
    let signer_seeds: &[&[&[u8]]] = &[&[SATURN_GROUP_SEED.as_bytes(), authority_bump_seeds.as_ref()]];

    let ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        accounts,
        signer_seeds
    );

    let _ = token::transfer(ctx, payout);

     


        Ok(())
    }

#[derive(Accounts)]

pub struct RedeemTokens<'info> {
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

    #[account(
        mut,
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
    pub usdc_token_mint: Account<'info, Mint>,
    #[account(
        mut,
        mint::decimals = 9,
        mint::authority = stf_token_mint,
        seeds = [MINT_SEED.as_ref()],
        bump
    )]
    pub stf_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}