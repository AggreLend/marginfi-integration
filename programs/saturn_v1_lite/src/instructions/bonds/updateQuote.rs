use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::{
    create_metadata_accounts_v3,
    CreateMetadataAccountsV3,
};
use mpl_token_metadata::types::DataV2;

use crate::constants::*;



    pub fn handle(ctx: Context<Transfer>) -> Result<()> {

        Ok(())
    }

#[derive(Accounts)]
pub struct Transfer<'info> {

}