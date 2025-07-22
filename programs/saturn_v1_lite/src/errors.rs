use anchor_lang::prelude::*;

#[error_code]
pub enum BondError {
    #[msg("Insufficient Funds")]
    InsufficientFundsError,
    #[msg("Token Mint Error")]
    TokenMintError,
    #[msg("Get Back Price Error")]
    BackPriceError,
    #[msg("Get Spot Price Error")]
    SpotPriceError,
    #[msg("Get Deduction Error")]
    DeductionError,
    #[msg("Treasury Fund Error")]
    TreasuryFundError,
    #[msg("Collateral Not in List Error")]
    CollateralError,
    #[msg("Bond Not finished")]
    BondNotFinished,
    #[msg("Not the Creator")]
    CreatorError,
    #[msg("Already Redeemed")]
    AlreadyRedeem,
    #[msg("Not enough staked Saturn To Unstake")]
    UnstakingError,
    #[msg("IncorrectOwner")]
    IncorrectOwner
}