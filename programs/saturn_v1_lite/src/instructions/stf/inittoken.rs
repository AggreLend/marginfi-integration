use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::{
    create_metadata_accounts_v3,
    CreateMetadataAccountsV3,
};
use mpl_token_metadata::types::DataV2;

use crate::constants::*;



    pub fn handle(ctx: Context<CreateSTF>, amount: u64, name: String, symbol: String, uri: String) -> Result<()> {
        let mint_seeds = &[MINT_SEED.as_bytes().as_ref(), &[ctx.bumps.mint]];
    
        // Create metadata
    
    
        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info().clone(), 
                CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata.to_account_info().clone(),
                mint: ctx.accounts.mint.to_account_info().clone(),
                mint_authority: ctx.accounts.mint.to_account_info().clone(),
                payer: ctx.accounts.payer.to_account_info().clone(),
                update_authority: ctx.accounts.mint.to_account_info().clone(),
                system_program: ctx.accounts.system_program.to_account_info().clone(),
                rent: ctx.accounts.rent.to_account_info(),
                },
             &[&mint_seeds[..]],
            ),
          DataV2 {
              name,
              symbol,
              seller_fee_basis_points: 0,
              uri,
              creators: None,
              collection: None,
              uses: None,
            }, 
          true, 
          false, 
          None
          )?;
    
    
        // Mint tokens
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint.to_account_info(),
                },
                &[mint_seeds],
            ),
            amount,
        )?;
    
        Ok(())
    }

#[derive(Accounts)]
pub struct CreateSTF<'info> {
    
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = mint,
        seeds = [MINT_SEED.as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: This is safe as it's used only as a signing PDA
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    /// CHECK: This insturction is only used by the admin not a user so it's safe to add it without an address constraint or similar.
    pub token_metadata_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
}