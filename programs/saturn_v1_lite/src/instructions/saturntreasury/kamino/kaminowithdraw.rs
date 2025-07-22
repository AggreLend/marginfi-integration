
use anchor_lang::solana_program::sysvar;
use anchor_lang::{
    prelude::*,
    solana_program::sysvar::{instructions::Instructions as SysInstructions, SysvarId},
};
use anchor_spl::token::{Token, TokenAccount, Mint};
use klend::{
    cpi::accounts::WithdrawObligationCollateralAndRedeemReserveCollateral,
    program::KaminoLending,
    state::{Obligation, Reserve},
};

use crate::constants::SATURN_GROUP_SEED;
use crate::saturnaccounts::Treasury;

#[derive(Accounts)]
pub struct KlendWithdraw<'info> {
    #[account(
        mut,
    )]
    signer: Signer<'info>,

    #[account(
        mut,
        seeds = [SATURN_GROUP_SEED.as_ref()],
        bump,
    )]
    pub treasury: Box<Account<'info, Treasury>>,

    #[account(mut,
        token::mint = withdraw_reserve.liquidity.mint_pubkey
    )]
    pub user_destination_liquidity: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    /// CHECK: address on account checked
    #[account(address = sysvar::instructions::ID)]
    pub instruction_sysvar_account: AccountInfo<'info>,

    /*
     * klend accounts
     */
    pub klend_program: Program<'info, KaminoLending>,
    /// CHECK: devnet demo
    #[account(
        mut,
        has_one = lending_market,
        constraint = obligation.owner == treasury.key(),
    )]
    pub obligation: Box<Account<'info, Obligation>>,
    /// CHECK: devnet demo
    pub lending_market: AccountInfo<'info>,

    #[account(mut,
        has_one = lending_market
    )]
    pub withdraw_reserve: Box<Account<'info, Reserve>>,

    #[account(mut, address = withdraw_reserve.collateral.supply_vault)]
    pub reserve_source_collateral: Box<Account<'info, TokenAccount>>,
    #[account(mut, address = withdraw_reserve.collateral.mint_pubkey)]
    pub reserve_collateral_mint: Box<Account<'info, Mint>>,
    #[account(mut, address = withdraw_reserve.liquidity.supply_vault)]
    pub reserve_liquidity_supply: Box<Account<'info, TokenAccount>>,

    /// CHECK: just authority
    pub lending_market_authority: AccountInfo<'info>,
    #[account(mut,
        token::mint = reserve_collateral_mint.key()
    )]
    pub user_destination_collateral: Box<Account<'info, TokenAccount>>,
}

pub fn handle(ctx: Context<KlendWithdraw>, amount: u64) -> Result<()> {
    // let owner_key = ctx.accounts.saturn_lending.treasury_admin;
    let signer_seeds: &[&[u8]] = &[
        SATURN_GROUP_SEED.as_ref(),
        &[ctx.bumps.treasury],
    ];

    klend::cpi::withdraw_obligation_collateral_and_redeem_reserve_collateral(
        CpiContext::new_with_signer(
            ctx.accounts.klend_program.to_account_info(),
            WithdrawObligationCollateralAndRedeemReserveCollateral {
                owner: ctx.accounts.treasury.to_account_info(),
                obligation: ctx.accounts.obligation.to_account_info(),
                lending_market: ctx.accounts.lending_market.to_account_info(),
                lending_market_authority: ctx.accounts.lending_market_authority.to_account_info(),
                withdraw_reserve: ctx.accounts.withdraw_reserve.to_account_info(),
                reserve_liquidity_supply: ctx.accounts.reserve_liquidity_supply.to_account_info(),
                reserve_collateral_mint: ctx.accounts.reserve_collateral_mint.to_account_info(),
                user_destination_collateral: ctx
                    .accounts
                    .user_destination_collateral
                    .to_account_info(),
                // placeholder_user_destination_collateral: Some(ctx.accounts.user_destination_collateral.clone().expect("REASON").to_account_info()),
                token_program: ctx.accounts.token_program.to_account_info(),
                instruction_sysvar_account: ctx.accounts.instruction_sysvar_account.to_account_info(),
                user_destination_liquidity: ctx
                    .accounts
                    .user_destination_liquidity
                    .to_account_info(),
                reserve_source_collateral: ctx.accounts.reserve_source_collateral.to_account_info(),
            },
            &[signer_seeds],
        ),
        amount,
    )?;

    Ok(())
}