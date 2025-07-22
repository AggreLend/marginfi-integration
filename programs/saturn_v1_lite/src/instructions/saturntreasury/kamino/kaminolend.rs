
use anchor_lang::{
    prelude::*,
    solana_program::sysvar::{instructions::Instructions as SysInstructions, SysvarId},
};
use anchor_spl::token::{Token, TokenAccount, Mint};
use klend::{
    cpi::accounts::DepositReserveLiquidityAndObligationCollateral,
    program::KaminoLending,
    state::{LendingMarket, Obligation, Reserve},
};

use crate::{constants::SATURN_GROUP_SEED, saturnaccounts::Treasury};

#[derive(Accounts)]
pub struct KaminoLend<'info> {
    #[account(
        mut,
        constraint = signer.key() == treasury.treasury_admin.key()
    )]
    signer: Signer<'info>,


    #[account(
        mut,
        seeds = [SATURN_GROUP_SEED.as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,



    #[account(mut,
        has_one = lending_market,
        constraint = obligation.owner == treasury.key(),
    )]
    pub obligation: Account<'info, Obligation>,

    pub lending_market: Account<'info, LendingMarket>,

    /// CHECK: just authority
    pub lending_market_authority: AccountInfo<'info>,

    #[account(mut,
        has_one = lending_market
    )]
    pub reserve: Account<'info, Reserve>,

    #[account(mut,
        address = reserve.liquidity.supply_vault
    )]
    pub reserve_liquidity_supply: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        address = reserve.collateral.mint_pubkey
    )]
    pub reserve_collateral_mint: Box<Account<'info, Mint>>,

    #[account(mut,
        address = reserve.collateral.supply_vault
    )]
    pub reserve_destination_deposit_collateral: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        token::mint = reserve.liquidity.mint_pubkey
    )]
    pub user_source_liquidity: Account<'info, TokenAccount>,

    #[account(mut,
        token::mint = reserve_collateral_mint.key()
    )]
    pub user_destination_collateral: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    #[account(address = SysInstructions::id())]
    /// CHECK:address checked
    pub instruction: AccountInfo<'info>,
    pub klend_program: Program<'info, KaminoLending>,
}

pub fn handle(ctx: Context<KaminoLend>, amount: u64) -> Result<()> {
    
    // let owner_key = ctx.accounts.saturn_lending.treasury_admin;
    let signer_seeds: &[&[u8]] = &[
        SATURN_GROUP_SEED.as_ref(),
        &[ctx.bumps.treasury],
    ];

    klend::cpi::deposit_reserve_liquidity_and_obligation_collateral(
        CpiContext::new_with_signer(
            ctx.accounts.klend_program.to_account_info(),
            DepositReserveLiquidityAndObligationCollateral {
                owner: ctx.accounts.treasury.to_account_info(),
                obligation: ctx.accounts.obligation.to_account_info(),
                lending_market: ctx.accounts.lending_market.to_account_info(),
                lending_market_authority: ctx.accounts.lending_market_authority.to_account_info(),
                reserve: ctx.accounts.reserve.to_account_info(),
                reserve_liquidity_supply: ctx.accounts.reserve_liquidity_supply.to_account_info(),
                reserve_collateral_mint: ctx.accounts.reserve_collateral_mint.to_account_info(),
                reserve_destination_deposit_collateral: ctx
                    .accounts
                    .reserve_destination_deposit_collateral
                    .to_account_info(),
                user_source_liquidity: ctx.accounts.user_source_liquidity.to_account_info(),
                user_destination_collateral: ctx
                    .accounts
                    .user_destination_collateral
                    .to_account_info(),
                // placeholder_user_destination_collateral: Some(ctx.accounts.user_destination_collateral.clone().expect("REASON").to_account_info()),
                token_program: ctx.accounts.token_program.to_account_info(),
                instruction_sysvar_account: ctx
                    .accounts
                    .instruction
                    .to_account_info(),
                // user_destination_collateral: todo!(),
            },
            &[signer_seeds],
        ),
        amount as u64,
    )?;

    Ok(())
}
