use anchor_lang::prelude::*;



use crate::constants::SATURN_GROUP_SEED;
use crate::saturnaccounts::Treasury;



/// Need to check whether we can convert to unchecked account
#[derive(Accounts)]
pub struct MeteoraWithdraw<'info> {
    #[account(
        mut,
        constraint = signer.key() == treasury.treasury_admin.key()
    )]
    signer: Signer<'info>,

    /// CHECK: this is pda
    #[account(
        mut,
        seeds = [SATURN_GROUP_SEED.as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    /// CHECK: Pool account (PDA)
    pub pool: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: LP token mint of the pool
    pub lp_mint: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: user pool lp token account. lp will be burned from this account upon success liquidity removal.
    pub user_pool_lp: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub a_vault_lp: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    pub b_vault_lp: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    pub a_vault: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    pub b_vault: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: LP token mint of vault a
    pub a_vault_lp_mint: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: LP token mint of vault b
    pub b_vault_lp_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Token vault account of vault A
    pub a_token_vault: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Token vault account of vault B
    pub b_token_vault: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: User token A account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_a_token: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: User token B account. Token will be transfer from this account if it is add liquidity operation. Else, token will be transfer into this account.
    pub user_b_token: UncheckedAccount<'info>,
    /// CHECK: User account. Must be owner of user_a_token, and user_b_token.
    pub user: Signer<'info>,

    /// CHECK: Vault program. the pool will deposit/withdraw liquidity from the vault.
    pub vault_program: UncheckedAccount<'info>,
    #[account(address = meteorapool::ID)]
    /// CHECK: Dynamic AMM program account
    pub dynamic_amm_program: UncheckedAccount<'info>,
    /// CHECK: Token program.
    pub token_program: UncheckedAccount<'info>,
}

#[allow(unused_variables)]
pub fn handle(
    ctx: Context<MeteoraWithdraw>,
    pool_token_amount: u64,
    maximum_token_a_amount: u64,
    maximum_token_b_amount: u64,
) -> Result<()> {
    let accounts = meteorapool::cpi::accounts::AddBalanceLiquidity {
        pool: ctx.accounts.pool.to_account_info(),
        lp_mint: ctx.accounts.lp_mint.to_account_info(),
        user_pool_lp: ctx.accounts.user_pool_lp.to_account_info(),
        a_vault_lp: ctx.accounts.a_vault_lp.to_account_info(),
        b_vault_lp: ctx.accounts.b_vault_lp.to_account_info(),
        a_vault: ctx.accounts.a_vault.to_account_info(),
        b_vault: ctx.accounts.b_vault.to_account_info(),
        a_vault_lp_mint: ctx.accounts.a_vault_lp_mint.to_account_info(),
        b_vault_lp_mint: ctx.accounts.b_vault_lp_mint.to_account_info(),
        a_token_vault: ctx.accounts.a_token_vault.to_account_info(),
        b_token_vault: ctx.accounts.b_token_vault.to_account_info(),
        user_a_token: ctx.accounts.user_a_token.to_account_info(),
        user_b_token: ctx.accounts.user_b_token.to_account_info(),
        user: ctx.accounts.user.to_account_info(),
        vault_program: ctx.accounts.vault_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
    };

    let cpi_context = CpiContext::new(ctx.accounts.dynamic_amm_program.to_account_info(), accounts);
    meteorapool::cpi::add_balance_liquidity(
        cpi_context,
        pool_token_amount,
        maximum_token_a_amount,
        maximum_token_b_amount,
    )
}
