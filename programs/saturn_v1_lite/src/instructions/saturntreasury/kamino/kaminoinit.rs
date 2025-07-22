use crate::{
   
    constants::*, saturnaccounts::Treasury
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token};

use klend::{
    cpi::accounts::{InitObligation, InitUserMetadata},
    program::KaminoLending,
    InitObligationArgs,
};


#[derive(Accounts)]
pub struct MarginfiLend<'info> {


        /// CHECK: devnet demo
        pub klend_program: Program<'info, KaminoLending>,
        /// CHECK: devnet demo
        pub seed1_account: AccountInfo<'info>,
        /// CHECK: devnet demo
        pub seed2_account: AccountInfo<'info>,
        /// CHECK: devnet demo
        pub lending_market: AccountInfo<'info>,
        /// CHECK: devnet demo
        #[account(mut)]
        pub obligation: AccountInfo<'info>,
        /// CHECK: devnet demo
        #[account(mut)]
        pub address_look_up_table: AccountInfo<'info>,
        /// CHECK: devnet demo
        #[account(mut)]
        pub user_metadata: AccountInfo<'info>,
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

pub fn handle(ctx: Context<MarginfiLend>, amount: u64) -> Result<()> {
    let _treasury = &ctx.accounts.treasury;
    let _amount = amount;
    // let owner_key = ctx.accounts.treasury_authority.to_account_info();
    let signer_seeds: &[&[u8]] = &[
          SATURN_GROUP_SEED.as_ref(),
        &[ctx.bumps.treasury],
    ];

    klend::cpi::init_user_metadata(
        CpiContext::new_with_signer(
            ctx.accounts.klend_program.to_account_info(),
            InitUserMetadata {
                owner: ctx.accounts.treasury.to_account_info(),
                user_metadata: ctx.accounts.user_metadata.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &[signer_seeds],
        ),
        ctx.accounts.signer.key(), // user_lookup_table
        ctx.accounts.address_look_up_table.key()
    )?;

    klend::cpi::init_obligation(
        CpiContext::new_with_signer(
            ctx.accounts.klend_program.to_account_info(),
            InitObligation {
                obligation_owner: ctx.accounts.treasury.to_account_info(),
                obligation: ctx.accounts.obligation.to_account_info(),
                lending_market: ctx.accounts.lending_market.to_account_info(),
                seed1_account: ctx.accounts.seed1_account.to_account_info(),
                seed2_account: ctx.accounts.seed2_account.to_account_info(),
                owner_user_metadata: ctx.accounts.user_metadata.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &[signer_seeds],
        ),
        InitObligationArgs { tag: 0, id: 0 },
    )?;

    


    Ok(())
}
