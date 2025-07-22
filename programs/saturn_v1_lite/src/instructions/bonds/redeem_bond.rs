


use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;

use anchor_spl::token::{self, Mint, Token, TokenAccount};



use crate::{constants::*};
use crate::saturnaccounts::{SaturnBond, Treasury};




    pub fn handle(ctx: Context<RedeemBond>,_id: String) -> Result<()> {
        let mint_seeds = &[MINT_SEED.as_bytes().as_ref(), &[ctx.bumps.stf_token_mint]];

        let clock = Clock::get()?;
        let _current_timestamp = clock.unix_timestamp;

        // assert!(
        //     ctx.accounts.bond_account.end_timestamp <= current_timestamp, 
        //     "Bond Redemption Time Not Over"
        // );

        assert!(
            !ctx.accounts.bond_account.is_finished,
            "Bond Has Been Redeemed"
        );


        // Give Tokens To User
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.stf_token_mint.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.stf_token_mint.to_account_info(),
                },
                &[mint_seeds],
            ),
            ctx.accounts.bond_account.num_token_to_redeem,
        )?;

        // Update Data To Ensure that the user won't be able to double spend bond

        ctx.accounts.bond_account.is_finished = true;


    

        Ok(())
    }

#[derive(Accounts)]
#[instruction(id: String)]
pub struct RedeemBond<'info> {

    #[account(
        mut,
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
    #[account(
        mut,
        mint::decimals = 9,
        mint::authority = stf_token_mint,
        seeds = [MINT_SEED.as_ref()],
        bump
    )]
    pub stf_token_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}