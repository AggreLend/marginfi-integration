use crate::{
   
    constants::*, saturnaccounts::Treasury
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use marginfiv2cpi::{

    cpi::accounts::LendingAccountWithdraw,
    program::Marginfi,
};



#[derive(Accounts)]
pub struct MarginfiWithdraw<'info> {

     /*
    * marginfi accounts
    */
    pub marginfi_program: Program<'info, Marginfi>,

    /// CHECK: no validation, for educational purpose only
    #[account(mut)]
    pub marginfi_account: AccountInfo<'info>,

    /// CHECK: no validation, for educational purpose only
    #[account(mut)]
    pub marginfi_group: AccountInfo<'info>,

    /// CHECK: no validation, for educational purpose only
    #[account(mut)]
    pub funding_account: AccountInfo<'info>,

    /// CHECK: no validation, for educational purpose only
    #[account(mut)]
    pub token_vault: AccountInfo<'info>,

    #[account(mut)]
    pub user_liquidity: Account<'info, TokenAccount>,

    /// CHECK: marginfi account
    #[account(mut)]
    pub bank_liquidity_vault: AccountInfo<'info>,
    /// CHECK: no validation, for educational purpose only
    #[account(mut)]
    pub bank_liquidity_vault_authority: AccountInfo<'info>,
    ///  CHECK: Only for authority
    #[account(mut)]
    pub bank: AccountInfo<'info>,

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

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,

   
}

pub fn handle(ctx: Context<MarginfiWithdraw>, amount: u64) -> Result<()> {
    let _treasury = &ctx.accounts.treasury;
    let amount = amount;
    // let owner_key = ctx.accounts.treasury_authority.to_account_info();
    let signer_seeds: &[&[u8]] = &[
          SATURN_GROUP_SEED.as_ref(),
        &[ctx.bumps.treasury],
    ];

    let _ = marginfiv2cpi::cpi::lending_account_withdraw(
        CpiContext::new_with_signer(
            ctx.accounts.marginfi_program.to_account_info(),
            LendingAccountWithdraw {
                marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
                marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
                signer: ctx.accounts.treasury.to_account_info(),
                bank: ctx.accounts.bank.to_account_info(),
                destination_token_account: ctx.accounts.user_liquidity.to_account_info(),
                bank_liquidity_vault_authority: ctx.accounts.bank_liquidity_vault_authority.to_account_info(),
                bank_liquidity_vault: ctx.accounts.bank_liquidity_vault.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
            &[signer_seeds], //remaining_accounts: ctx.remaining_accounts.into(),
        ),
        amount,
        None,
    );
    Ok(())
}
