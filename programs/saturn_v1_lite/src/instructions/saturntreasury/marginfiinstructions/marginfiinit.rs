use anchor_lang::{prelude::*};


#[derive(Accounts)]
pub struct InitializeMarginAccount<'info> {
    /// CHECK: no validation, for educational purpose only
    pub marginfi_program: AccountInfo<'info>,

    /// CHECK: no validation, for educational purpose only
    pub marginfi_group: AccountInfo<'info>,

    /// CHECK: no validation, for educational purpose only
    #[account(zero)]
    pub marginfi_account: AccountInfo<'info>,

    #[account(mut)]
    signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_account(ctx: Context<InitializeMarginAccount>) -> Result<()> {
    let ctx = CpiContext::new(
        ctx.accounts.marginfi_program.to_account_info(),
        marginfiv2cpi::cpi::accounts::MarginfiAccountInitialize {
            authority: ctx.accounts.signer.to_account_info(),
            marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
            marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            fee_payer: ctx.accounts.signer.to_account_info()
        },
    );
    marginfiv2cpi::cpi::marginfi_account_initialize(ctx)?;
    Ok(())
}
