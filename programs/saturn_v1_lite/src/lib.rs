use anchor_lang::prelude::*;

// pub mod instructions;
// mod constants;
// use constants::*;

mod errors;
mod constants;
mod instructions;
mod saturnaccounts;
mod utils;
use instructions::*;


declare_id!("Bnu6XjbbozD3irEo4H9nLNsBixxmQhU4DMjFhMfWD7mq");

#[program]
pub mod saturn_v1_lite {



    // use instructions::CreateSTF;


    use super::*;


    // Init Instructions

    pub fn mint_token(ctx: Context<CreateSTF>, amount: u64, name: String, symbol: String, uri: String) -> Result<()> {
        let _ = instructions::inittoken::handle(ctx, amount, name, symbol, uri);
        Ok(())
    }

    

    pub fn init_treasury(ctx: Context<InitializeSaturn>) -> Result<()> {
        let _ = instructions::init_treasury::handle(ctx);
        Ok(())
    }


    // Stake
    pub fn stake_stf(ctx: Context<StakeSTF>, amount: u64) -> Result<()> {
        let _ = instructions::stake::handle(ctx, amount);
        Ok(())
    }

    pub fn unstake_stf(ctx: Context<UnstakeSTF>, amount: u64) -> Result<()> {
   
        let _ = instructions::unstake::handle(ctx, amount);
        Ok(())
    }

    // Bonds 

    pub fn update_quote(ctx: Context<UpdateBondQuote>, current_trading_price: u64, current_treasury_value: i128) -> Result<()> {
        let _ = instructions::update_quote::handle(ctx, current_trading_price, current_treasury_value);
        Ok(())
    }

    pub fn create_bond(ctx: Context<CreateBond>, id: String, token_amount: u64) -> Result<()> {
        let _ = instructions::create_bond::handle(ctx,  id, token_amount);
        Ok(())
    }

    pub fn redeem_bond(ctx: Context<RedeemBond>, id: String) -> Result<()> {
        let _ = instructions::redeem_bond::handle(ctx,  id);
        Ok(())
    }

    pub fn redeem_stf(ctx: Context<RedeemTokens>, amount: u64) -> Result<()> {
        let _ = instructions::stf::burn_tokens::handle(ctx, amount);
        Ok(())
    }



    // MarginFi

    pub fn marginfi_lending_initialize(ctx: Context<InitializeMarginAccount>) -> Result<()> {
        let _ = instructions::marginfiinstructions::initialize_account(ctx);
        Ok(())
    }

    pub fn marginfi_deposit(ctx: Context<MarginfiLend>, amount: u64) -> Result<()> {
        let _ = instructions::marginfi_deposit::handle(ctx, amount);
        Ok(())
    }

    pub fn marginfi_withdraw(ctx: Context<MarginfiWithdraw>, amount: u64) -> Result<()> {
        let _ = instructions::marginfiwithdraw::handle(ctx, amount);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {}
